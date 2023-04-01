#version 450
#extension GL_EXT_scalar_block_layout : enable

layout(binding = 0, scalar) uniform FunctionData {
    mat4 model_matrix;
    mat4 projection_matrix;
    vec3 rect_min;
    vec3 rect_max;
    float time;
    vec3 camera_dir;
} data;

layout(location = 0) in vec3 inWorldPos;
layout(location = 1) in vec3 inViewPos;

layout(location = 0) out vec4 outColor;

vec4 gradient_density(vec4 d, vec3 n);

// https://madebyevan.com/shaders/grid/
float grid_intensity(vec2 pos, vec2 fwidth, vec2 bias) {
    vec2 grid = abs(fract(pos - 0.5) - 0.5) / fwidth + bias;
    float line = min(grid.x, grid.y);
    float intensity = min(line, 1.0);
    return pow(intensity, 1.0 / 2.2);
}

float complete_grid(vec3 world_pos, vec3 world_normal) {
    // do not draw lines for almost-parallel surfaces
    bool draw_line_x = abs(dot(world_normal, vec3(1.0, 0.0, 0.0))) < 0.998;
    bool draw_line_y = abs(dot(world_normal, vec3(0.0, 1.0, 0.0))) < 0.998;
    vec2 line_bias = vec2(bvec2(!draw_line_x, !draw_line_y));

    vec2 extent = data.rect_min.xy - data.rect_max.xy;
    vec2 fun_pos = world_pos.xy / extent;

    vec2 width = fwidth(16 * fun_pos);

    vec2 line_pos = (inWorldPos.xy / 4.0) - fract(inWorldPos.xy / 4.0) + fract(fun_pos) * 16;
    return grid_intensity(line_pos, width, line_bias);
}

// (possible to use color mixing code from https://github.com/fstl-app/fstl/blob/master/gl/mesh.frag)
void main() {
    vec3 normalized_pos = inWorldPos / 63.0;
    vec3 world_pos = mix(data.rect_min, data.rect_max, normalized_pos);
    vec3 world_normal = normalize(gradient_density(vec4(world_pos, data.time), normalized_pos).xyz);

    vec3 color = mix(vec3(1.0), abs(world_normal), 0.6);
    float cosine = max(abs(dot(data.camera_dir, world_normal)), 0.0);
    float darken = mix(0.3, 1.0, cosine);
    float line = complete_grid(world_pos, world_normal);

    outColor = vec4(color * darken * line, 1.0);
}