use cosmic::app::{Core, Task};
use cosmic::iced::window::Id;
use cosmic::iced::Rectangle;
use cosmic::iced_runtime::core::window;
use cosmic::surface::action::{app_popup, destroy_popup};
use cosmic::widget::{list_column, toggler, button, text, row, horizontal_space};
use cosmic::Element;
use zbus::Connection;
use std::time::Duration;
use cosmic::iced::Alignment;
use chrono::{Local, Timelike};
use serde::{Deserialize, Serialize};
use std::fs;

const SERVICE: &str = "io.github.kernel_ux.EliteNightLight";
const PATH: &str = "/io/github/kernel_ux/EliteNightLight";
const INTERFACE: &str = "io.github.kernel_ux.EliteNightLight";

const ID: &str = "io.github.kernel_ux.EliteNightLight";

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
struct Config {
    auto_mode: bool,
    manual_enabled: bool,
    level: u8,
    last_scheduled_is_night: bool,
}

impl Default for Config {
    fn default() -> Self {
        let hour = Local::now().hour();
        Self {
            auto_mode: false,
            manual_enabled: false,
            level: 1,
            last_scheduled_is_night: hour >= 19 || hour < 7,
        }
    }
}

pub struct Window {
    core: Core,
    popup: Option<Id>,
    enabled: bool,
    level: u8,
    auto_mode: bool,
    last_scheduled_is_night: bool,
    last_interaction: std::time::Instant,
}

impl Default for Window {
    fn default() -> Self {
        Self {
            core: Core::default(),
            popup: None,
            enabled: false,
            level: 1,
            auto_mode: false,
            last_scheduled_is_night: false,
            last_interaction: std::time::Instant::now(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    PopupClosed(Id),
    Surface(cosmic::surface::Action),
    Toggle(bool),
    ToggleAuto(bool),
    SetLevel(u8),
    Tick,
    None,
}

impl cosmic::Application for Window {
    type Executor = cosmic::executor::Default;
    type Message = Message;
    type Flags = ();
    const APP_ID: &'static str = ID;

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, _flags: Self::Flags) -> (Self, Task<Message>) {
        let mut window = Window {
            core,
            ..Default::default()
        };
        
        let saved_cfg = if let Ok(content) = fs::read_to_string(config_path()) {
            serde_json::from_str::<Config>(&content).unwrap_or_default()
        } else {
            Config::default()
        };

        window.auto_mode = saved_cfg.auto_mode;
        window.level = saved_cfg.level;
        
        let hour = Local::now().hour();
        let current_is_night = hour >= 19 || hour < 7;

        if window.auto_mode {
            if current_is_night != saved_cfg.last_scheduled_is_night {
                window.enabled = current_is_night;
            } else {
                window.enabled = saved_cfg.manual_enabled;
            }
        } else {
            window.enabled = saved_cfg.manual_enabled;
        }
        
        window.last_scheduled_is_night = current_is_night;

        let level = window.level;
        let enabled = window.enabled;
        let apply_task = Task::future(async move {
            if let Ok(conn) = Connection::session().await {
                let _ = conn.call_method(Some(SERVICE), PATH, Some(INTERFACE), "SetLevel", &(level,)).await;
                let _ = conn.call_method(Some(SERVICE), PATH, Some(INTERFACE), "SetEnabled", &(enabled,)).await;
            }
            cosmic::Action::App(Message::Tick)
        });

        window.save_config();

        (window, apply_task)
    }

    fn on_close_requested(&self, id: window::Id) -> Option<Message> {
        Some(Message::PopupClosed(id))
    }

    fn update(&mut self, message: Message) -> Task<Self::Message> {
        match message {
            Message::PopupClosed(id) => {
                if self.popup.as_ref() == Some(&id) {
                    self.popup = None;
                }
            }
            Message::Surface(a) => {
                return cosmic::task::message(cosmic::Action::Cosmic(
                    cosmic::app::Action::Surface(a),
                ));
            }
            Message::Tick => {
                if self.auto_mode && self.last_interaction.elapsed() > Duration::from_secs(5) {
                    let hour = Local::now().hour();
                    let current_is_night = hour >= 19 || hour < 7;
                    
                    if current_is_night != self.last_scheduled_is_night {
                        self.enabled = current_is_night;
                        self.last_scheduled_is_night = current_is_night;
                        self.save_config();
                        let val = current_is_night;
                        return Task::future(async move {
                            if let Ok(conn) = Connection::session().await {
                                let _ = conn.call_method(Some(SERVICE), PATH, Some(INTERFACE), "SetEnabled", &(val,)).await;
                            }
                            cosmic::Action::App(Message::Tick)
                        });
                    }
                }

                if self.last_interaction.elapsed() > Duration::from_secs(3) {
                    self.sync_from_dbus();
                }

                return Task::future(async {
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    cosmic::Action::App(Message::Tick)
                });
            }
            Message::Toggle(val) => {
                self.enabled = val;
                self.last_interaction = std::time::Instant::now();
                self.save_config();
                return Task::future(async move {
                    if let Ok(conn) = Connection::session().await {
                        let _ = conn.call_method(Some(SERVICE), PATH, Some(INTERFACE), "SetEnabled", &(val,)).await;
                    }
                    cosmic::Action::App(Message::None)
                });
            }
            Message::ToggleAuto(val) => {
                self.auto_mode = val;
                self.last_interaction = std::time::Instant::now();
                
                if val {
                    let hour = Local::now().hour();
                    let is_night = hour >= 19 || hour < 7;
                    self.enabled = is_night;
                    self.last_scheduled_is_night = is_night;
                    let enabled = self.enabled;
                    self.save_config();
                    return Task::future(async move {
                        if let Ok(conn) = Connection::session().await {
                            let _ = conn.call_method(Some(SERVICE), PATH, Some(INTERFACE), "SetEnabled", &(enabled,)).await;
                        }
                        cosmic::Action::App(Message::None)
                    });
                }
                self.save_config();
            }
            Message::SetLevel(val) => {
                self.level = val;
                self.last_interaction = std::time::Instant::now();
                self.save_config();
                return Task::future(async move {
                    if let Ok(conn) = Connection::session().await {
                        let _ = conn.call_method(Some(SERVICE), PATH, Some(INTERFACE), "SetLevel", &(val,)).await;
                    }
                    cosmic::Action::App(Message::None)
                });
            }
            Message::None => {}
        };
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let have_popup = self.popup.clone();
        let btn = self
            .core
            .applet
            .icon_button("weather-clear-night-symbolic")
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
                            let mut soft_btn = button::text("Soft").on_press(Message::SetLevel(1));
                            if state.level == 1 { soft_btn = soft_btn.class(cosmic::theme::Button::Suggested); }

                            let mut warm_btn = button::text("Warm").on_press(Message::SetLevel(2));
                            if state.level == 2 { warm_btn = warm_btn.class(cosmic::theme::Button::Suggested); }

                            let mut strong_btn = button::text("Strong").on_press(Message::SetLevel(3));
                            if state.level == 3 { strong_btn = strong_btn.class(cosmic::theme::Button::Suggested); }

                            let content_list = list_column()
                                .padding(10)
                                .spacing(10)
                                .add(
                                    row()
                                        .spacing(10)
                                        .align_y(Alignment::Center)
                                        .push(text("Elite Night Light").size(18))
                                        .push(horizontal_space())
                                        .push(toggler(state.enabled).on_toggle(Message::Toggle))
                                )
                                .add(
                                    row()
                                        .spacing(10)
                                        .align_y(Alignment::Center)
                                        .push(text("Auto Schedule").size(14))
                                        .push(horizontal_space())
                                        .push(toggler(state.auto_mode).on_toggle(Message::ToggleAuto))
                                )
                                .add(text("Intensity").size(14))
                                .add(
                                    row()
                                        .spacing(10)
                                        .push(soft_btn)
                                        .push(warm_btn)
                                        .push(strong_btn)
                                );
                            Element::from(state.core.applet.popup_container(content_list))
                                .map(cosmic::Action::App)
                        })),
                    ))
                }
            });

        Element::from(self.core.applet.applet_tooltip::<Message>(
            btn,
            "Elite Night Light",
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

impl Window {
    fn sync_from_dbus(&mut self) {
        if let Ok(conn) = zbus::blocking::Connection::session() {
            if let Ok(reply) = conn.call_method(Some(SERVICE), PATH, Some("org.freedesktop.DBus.Properties"), "Get", &(INTERFACE, "Enabled")) {
                if let Ok(val) = reply.body().deserialize::<zvariant::Value>() {
                    if let Ok(b) = bool::try_from(val) {
                        self.enabled = b;
                    }
                }
            }
            if let Ok(reply) = conn.call_method(Some(SERVICE), PATH, Some("org.freedesktop.DBus.Properties"), "Get", &(INTERFACE, "Level")) {
                if let Ok(val) = reply.body().deserialize::<zvariant::Value>() {
                    if let Ok(u) = u8::try_from(val) {
                        self.level = u;
                    }
                }
            }
        }
    }

    fn save_config(&self) {
        let cfg = Config {
            auto_mode: self.auto_mode,
            manual_enabled: self.enabled,
            level: self.level,
            last_scheduled_is_night: self.last_scheduled_is_night,
        };
        if let Ok(json) = serde_json::to_string(&cfg) {
            let _ = fs::write(config_path(), json);
        }
    }
}

fn config_path() -> String {
    format!("{}/.config/elite-night-light.json", std::env::var("HOME").unwrap_or_default())
}

fn main() -> cosmic::iced::Result {
    cosmic::applet::run::<Window>(())
}
