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

float worldPosLine(vec3 normal) {
    // do not draw lines for almost-parallel surfaces
    bool x = abs(dot(normal, vec3(1.0, 0.0, 0.0))) > 0.998;
    bool y = abs(dot(normal, vec3(0.0, 1.0, 0.0))) > 0.998;

    vec2 coord = inWorldPos.xy;
    // Compute anti-aliased world-space grid lines
    vec2 grid = abs(fract(coord) - 0.5) / fwidth(coord);

    float line = 1.0;
    if (!x) {
        line = min(line, grid.x);
    }
    if (!y) {
        line = min(line, grid.y);
    }
    
    // Just visualize the grid lines directly
    float intensity = min(line, 1.0);
    // Apply gamma correction
    intensity = pow(intensity, 1.0 / 2.2);

    return intensity;
}

// (possible to use color mixing code from https://github.com/fstl-app/fstl/blob/master/gl/mesh.frag)
void main() {
    vec3 normalized_pos = inWorldPos / 63.0;
    vec3 world_pos = mix(data.rect_min, data.rect_max, normalized_pos);
    vec3 world_normal = normalize(gradient_density(vec4(world_pos, data.time), normalized_pos).xyz);

    vec3 color = mix(vec3(1.0), abs(world_normal), 0.6);

    float cosine = max(dot(data.camera_dir, world_normal), 0.0);
    float darken = mix(0.3, 1.0, cosine);

    vec3 triangle_world_normal = normalize(cross(dFdx(inWorldPos), dFdy(inWorldPos)));
    float line = worldPosLine(triangle_world_normal);

    outColor = vec4(color * darken * line, 1.0);
}