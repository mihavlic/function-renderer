#version 450
#extension GL_EXT_scalar_block_layout : enable

layout(push_constant, scalar) uniform PushConstant { 
    mat4 model_matrix;
    mat4 projection_matrix;
    vec3 rect_min;
    vec3 rect_max;
    float time;
} push;

layout(location = 0) in vec3 inWorldPos;
layout(location = 1) in vec3 inViewPos;

layout(location = 0) out vec4 outColor;

vec4 gradient_density(vec4 d, vec3 n);

float worldPosLine(vec3 normal) {
    // do not draw lines for almost-parallel surfaces
    bool x = false; //abs(dot(normal, vec3(1.0, 0.0, 0.0))) > 0.95;
    bool y = false; //abs(dot(normal, vec3(0.0, 1.0, 0.0))) > 0.95;

    vec2 coord = (inWorldPos.xy - vec2(0.7, 2.7)) / 2.0;
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

void main() {
    vec3 view_normal = normalize(cross(dFdx(inViewPos), dFdy(inViewPos)));
    vec3 triangle_world_normal = normalize(cross(dFdx(inWorldPos), dFdy(inWorldPos)));
    float a = dot(view_normal, vec3(0.0, 0.0, 1.0));

#if 1
    vec3 normalized_pos = inWorldPos / 63.0;
    vec3 world_pos = mix(push.rect_min, push.rect_max, normalized_pos);
    vec4 density_input = vec4(world_pos, push.time);

    vec3 world_normal = abs(normalize(gradient_density(density_input, normalized_pos).xyz));
    // for some reson the normals are wrongly swizzled?
    vec3 color = mix(vec3(1.0), world_normal, 0.6);
    color = mix(vec3(0.0), color, min(a + 0.5, 1.0));
#else
    // color mixing code stolen from https://github.com/fstl-app/fstl/blob/master/gl/mesh.frag
    vec3 base3 = vec3(0.99, 0.96, 0.89);
    vec3 base2 = vec3(0.92, 0.91, 0.83);
    vec3 base00 = vec3(0.40, 0.48, 0.51);

    float b = dot(view_normal, vec3(-0.57, -0.57, 0.57));

    vec3 color = (a*base2 + (1-a)*base00)*0.5 + (b*base3 + (1-b)*base00)*0.5;
#endif

    float line = worldPosLine(triangle_world_normal);
    outColor = vec4(color * line, 1.0);
}