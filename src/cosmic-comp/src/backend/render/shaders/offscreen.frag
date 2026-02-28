#version 100

//_DEFINES_

#if defined(EXTERNAL)
#extension GL_OES_EGL_image_external : require
#endif

precision highp float;
#if defined(EXTERNAL)
uniform samplerExternalOES tex;
#else
uniform sampler2D tex;
#endif

uniform float alpha;
varying vec2 v_coords;

#if defined(DEBUG_FLAGS)
uniform float tint;
#endif

uniform float invert;
uniform float color_mode;

// ELITE NIGHT LIGHT: New uniform
uniform float night_light_level;

void main() {
    vec4 color = texture2D(tex, v_coords);

#if defined(NO_ALPHA)
    color = vec4(color.rgb, 1.0) * alpha;
#else
    color = color * alpha;
#endif

    // un-multiply
    color.rgb /= color.a;

    // First invert then filter

    if (invert == 1.0) {
        color.rgb = 1.0 - color.rgb;
    }

    if (color_mode == 1.0) {        // greyscale
        float value = (color.r + color.g + color.b) / 3.0;
        color = vec4(value, value, value, color.a);
    } else if (color_mode >= 2.0) {
        // ... (existing color blind filters)
    }

    // ELITE NIGHT LIGHT: Apply orange tint at the very end
    // Using 0.5 as threshold to be safe against undefined/noisy values
    if (night_light_level > 0.5) {
        vec3 orange_tint;
        if (night_light_level < 1.5) {
            orange_tint = vec3(1.0, 0.9, 0.8); // Soft (1.0)
        } else if (night_light_level < 2.5) {
            orange_tint = vec3(1.0, 0.8, 0.6); // Warm (2.0)
        } else {
            orange_tint = vec3(1.0, 0.7, 0.4); // Strong (3.0)
        }
        color.rgb *= orange_tint;
    }

    // re-multiply
    color.rgb *= color.a;

    gl_FragColor = color;
}
