#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) in vec3 inWorldPos;
layout(location = 1) in vec3 inWorldNormal;
layout(location = 2) in vec3 inViewNormal;

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
    float intensity = worldPosLine();
    outColor = vec4(abs(inWorldNormal) * intensity, 1.0);
}