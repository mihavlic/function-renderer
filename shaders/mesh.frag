#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) in vec3 inWorldPos;
layout(location = 1) in vec3 inViewPos;

layout(location = 0) out vec4 outColor;

float worldPosLine() {
    vec2 coord = inWorldPos.xy / 2.0;
    // Compute anti-aliased world-space grid lines
    vec2 grid = abs(fract(coord - 0.5) - 0.5) / fwidth(coord);
    float line = min(grid.x, grid.y);
    // Just visualize the grid lines directly
    float intensity = min(line, 1.0);
    // Apply gamma correction
    intensity = pow(intensity, 1.0 / 2.2);

    return intensity;
}

void main() {
    vec3 world_normal = abs(normalize(cross(dFdx(inWorldPos), dFdy(inWorldPos))));
    vec3 view_normal = normalize(cross(dFdx(inViewPos), dFdy(inViewPos)));

    float a = dot(view_normal, vec3(0.0, 0.0, 1.0));

#if 1
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

    float line = worldPosLine();
    outColor = vec4(color * line, 1.0);
}