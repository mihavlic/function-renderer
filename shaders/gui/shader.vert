#version 450

layout(push_constant) uniform PushConstants {
    vec2 screen_size;
} constants;

layout(location = 0) in vec2 position;
layout(location = 1) in vec2 tex_coords;
layout(location = 2) in vec4 color;

// the attribute is stored in memory as sRGB
layout(location = 0) out vec4 v_color_in_gamma;
layout(location = 1) out vec2 v_tex_coords;

void main() {
    gl_Position = vec4(
        2.0 * position.x / constants.screen_size.x - 1.0,
        2.0 * position.y / constants.screen_size.y - 1.0,
        0.0,
        1.0
    );
    
    v_tex_coords = tex_coords;
    v_color_in_gamma = color;
}