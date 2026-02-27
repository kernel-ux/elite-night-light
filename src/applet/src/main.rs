use cosmic::app::{Core, Task};
use cosmic::iced::window::Id;
use cosmic::iced::{Length, Rectangle, Alignment};
use cosmic::iced_runtime::core::window;
use cosmic::surface::action::{app_popup, destroy_popup};
use cosmic::widget::{list_column, settings, toggler, button};
use cosmic::iced::widget::row;
use cosmic::Element;
use zbus::blocking::Connection;
use std::time::{Duration, SystemTime};
use chrono::{Local, Timelike};

#[zbus::proxy(
    interface = "com.system76.CosmicComp.NightLight",
    default_service = "com.system76.CosmicComp",
    default_path = "/com/system76/CosmicComp/NightLight"
)]
trait NightLight {
    fn enabled(&self) -> zbus::Result<bool>;
    fn level(&self) -> zbus::Result<u8>;
    fn set_enabled(&self, enabled: bool) -> zbus::Result<()>;
    fn set_level(&self, level: u8) -> zbus::Result<()>;
}

const ID: &str = "com.system76.CosmicAppletNightLight";

pub struct Window {
    core: Core,
    popup: Option<Id>,
    enabled: bool,
    level: u8,
    auto: bool,
    last_auto_toggle: Option<bool>,
}

impl Default for Window {
    fn default() -> Self {
        Self {
            core: Core::default(),
            popup: None,
            enabled: false,
            level: 1,
            auto: false,
            last_auto_toggle: None,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    PopupClosed(Id),
    ToggleEnabled(bool),
    SetLevel(u8),
    ToggleAuto(bool),
    CheckSchedule,
    CheckState,
    Surface(cosmic::surface::Action),
    UpdateState(bool, u8),
    NoOp,
}

impl cosmic::Application for Window {
    type Executor = cosmic::SingleThreadExecutor;
    type Flags = ();
    type Message = Message;
    const APP_ID: &'static str = ID;

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, _flags: Self::Flags) -> (Self, Task<Message>) {
        let window = Window {
            core,
            ..Default::default()
        };
        
        // Initial state sync
        let task = Task::perform(async move {
            if let Ok(conn) = Connection::session() {
                if let Ok(proxy) = NightLightProxyBlocking::new(&conn) {
                    let enabled = proxy.enabled().unwrap_or(false);
                    let level = proxy.level().unwrap_or(1);
                    return Message::UpdateState(enabled, level);
                }
            }
            Message::NoOp
        }, |m| cosmic::Action::App(m));

        (window, task)
    }

    fn on_close_requested(&self, id: window::Id) -> Option<Message> {
        Some(Message::PopupClosed(id))
    }

    fn subscription(&self) -> cosmic::iced::Subscription<Self::Message> {
        // Check schedule every 30s, and sync state every 1s
        let schedule = cosmic::iced::time::every(Duration::from_secs(30)).map(|_| Message::CheckSchedule);
        let sync = cosmic::iced::time::every(Duration::from_secs(1)).map(|_| Message::CheckState);
        cosmic::iced::Subscription::batch(vec![schedule, sync])
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::PopupClosed(id) => {
                if self.popup.as_ref() == Some(&id) {
                    self.popup = None;
                }
            }
            Message::ToggleEnabled(toggled) => {
                if self.enabled != toggled {
                    self.enabled = toggled;
                    // If user manually toggles, we should probably disable auto or at least respect this choice
                    // For now, let's keep auto on but update the 'last_auto_toggle' to match user choice
                    // so it doesn't immediately flip back.
                    if self.auto {
                        self.last_auto_toggle = Some(toggled);
                    }
                    
                    return Task::perform(async move {
                        if let Ok(conn) = Connection::session() {
                            if let Ok(proxy) = NightLightProxyBlocking::new(&conn) {
                                let _ = proxy.set_enabled(toggled);
                            }
                        }
                        Message::NoOp
                    }, |m| cosmic::Action::App(m));
                }
            }
            Message::SetLevel(level) => {
                if self.level != level {
                    self.level = level;
                    let enabled = self.enabled;
                    return Task::perform(async move {
                        if let Ok(conn) = Connection::session() {
                            if let Ok(proxy) = NightLightProxyBlocking::new(&conn) {
                                let _ = proxy.set_level(level);
                                if !enabled {
                                    let _ = proxy.set_enabled(true);
                                }
                            }
                        }
                        if !enabled {
                            Message::UpdateState(true, level)
                        } else {
                            Message::NoOp
                        }
                    }, |m| cosmic::Action::App(m));
                }
            }
            Message::ToggleAuto(auto) => {
                self.auto = auto;
                if auto {
                    self.last_auto_toggle = None; // Reset so it checks immediately
                    return cosmic::task::message(cosmic::Action::App(Message::CheckSchedule));
                }
            }
            Message::CheckSchedule => {
                if self.auto {
                    let now = Local::now();
                    let hour = now.hour();
                    // Night is 7 PM (19) to 7 AM (7)
                    let should_be_enabled = hour >= 19 || hour < 7;
                    
                    if Some(should_be_enabled) != self.last_auto_toggle && self.enabled != should_be_enabled {
                        self.enabled = should_be_enabled;
                        self.last_auto_toggle = Some(should_be_enabled);
                        return Task::perform(async move {
                            if let Ok(conn) = Connection::session() {
                                if let Ok(proxy) = NightLightProxyBlocking::new(&conn) {
                                    let _ = proxy.set_enabled(should_be_enabled);
                                }
                            }
                            Message::NoOp
                        }, |m| cosmic::Action::App(m));
                    }
                }
            }
            Message::CheckState => {
                // Poll the actual D-Bus state
                return Task::perform(async move {
                    if let Ok(conn) = Connection::session() {
                        if let Ok(proxy) = NightLightProxyBlocking::new(&conn) {
                            let enabled = proxy.enabled().unwrap_or(false);
                            let level = proxy.level().unwrap_or(1);
                            return Message::UpdateState(enabled, level);
                        }
                    }
                    Message::NoOp
                }, |m| cosmic::Action::App(m));
            }
            Message::UpdateState(enabled, level) => {
                // If state changed externally and we are in auto mode, 
                // we sync our 'last_auto_toggle' to avoid immediate conflict.
                if self.auto && self.enabled != enabled {
                     let now = Local::now();
                     let hour = now.hour();
                     let should_be_enabled = hour >= 19 || hour < 7;
                     if enabled != should_be_enabled {
                         // User manually overrode the schedule (e.g. turned OFF at night)
                         // We set last_auto_toggle to the CURRENT hour's expected state
                         // so the scheduler thinks it already did its job.
                         self.last_auto_toggle = Some(should_be_enabled);
                     } else {
                         // State matches schedule now (maybe it was just turned back ON)
                         self.last_auto_toggle = Some(enabled);
                     }
                }
                
                self.enabled = enabled;
                self.level = level;
            }
            Message::Surface(a) => {
                return cosmic::task::message(cosmic::Action::Cosmic(
                    cosmic::app::Action::Surface(a),
                ));
            }
            Message::NoOp => {}
        };
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let have_popup = self.popup.clone();
        let icon_name = if self.enabled { "weather-clear-night-symbolic" } else { "weather-clear-night-disabled-symbolic" };
        
        let btn = self
            .core
            .applet
            .icon_button(icon_name)
            .on_press_with_rectangle(move |offset, bounds| {
                if let Some(id) = have_popup {
                    Message::Surface(destroy_popup(id))
                } else {
                    Message::Surface(app_popup::<Window>(
                        move |state: &mut Window| {
                            let new_id = Id::unique();
                            state.popup = Some(new_id);
                            let mut popup_settings = state.core.applet.get_popup_settings(
                                state.core.main_window_id().unwrap(),
                                new_id,
                                None,
                                None,
                                None,
                            );

                            popup_settings.positioner.anchor_rect = Rectangle {
                                x: (bounds.x - offset.x) as i32,
                                y: (bounds.y - offset.y) as i32,
                                width: bounds.width as i32,
                                height: bounds.height as i32,
                            };

                            popup_settings
                        },
                        Some(Box::new(move |state: &Window| {
                            let content_list = list_column()
                                .padding(10)
                                .spacing(10)
                                .add(settings::item(
                                    "Night Light",
                                    cosmic::widget::container(
                                        toggler(state.enabled)
                                            .on_toggle(|value| Message::ToggleEnabled(value)),
                                    )
                                    .height(Length::Fixed(50.)),
                                ))
                                .add(settings::item(
                                    "Auto (7PM - 7AM)",
                                    cosmic::widget::container(
                                        toggler(state.auto)
                                            .on_toggle(|value| Message::ToggleAuto(value)),
                                    )
                                    .height(Length::Fixed(50.)),
                                ))
                                .add(settings::item(
                                    "Intensity",
                                    row![
                                        button::text("Soft").on_press(Message::SetLevel(1)).class(if state.level == 1 { button::ButtonClass::Suggested } else { button::ButtonClass::Standard }),
                                        button::text("Warm").on_press(Message::SetLevel(2)).class(if state.level == 2 { button::ButtonClass::Suggested } else { button::ButtonClass::Standard }),
                                        button::text("Strong").on_press(Message::SetLevel(3)).class(if state.level == 3 { button::ButtonClass::Suggested } else { button::ButtonClass::Standard }),
                                    ]
                                    .spacing(5)
                                    .align_y(Alignment::Center)
                                ));
                            Element::from(state.core.applet.popup_container(content_list))
                                .map(cosmic::Action::App)
                        })),
                    ))
                }
            });

        Element::from(self.core.applet.applet_tooltip::<Message>(
            btn,
            "Night Light",
            self.popup.is_some(),
            |a| Message::Surface(a),
            None,
        ))
    }

    fn view_window(&self, _id: Id) -> Element<'_, Message> {
        "oops".into()
    }

    fn style(&self) -> Option<cosmic::iced_runtime::Appearance> {
        Some(cosmic::applet::style())
    }
}

fn main() -> cosmic::iced::Result {
    let env = env_logger::Env::default()
        .filter_or("MY_LOG_LEVEL", "warn")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);
    cosmic::applet::run::<Window>(())
}
