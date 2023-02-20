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
    // float intensity = dot(inNormal, normalize(vec3(1.0, 1.0, 5.0)));
    // vec3 a = inNormal * (max(0.0, intensity) + 0.1);
    
    // float a = (acos(inNormal.x) / 6.28) + 0.5;
    // float b = (acos(inNormal.y) / 6.28) + 0.5;
    // float z = (acos(inNormal.z) / 6.28) + 0.5;

    // float intensity = 1.0 - dot(normalize(inViewNormal), vec3(0.0, 1.0, 0.0));
    // float adjusted = intensity * 0.9 + 0.1;

    float intensity = worldPosLine();
    outColor = vec4(abs(inWorldNormal) * intensity, 1.0);

    // color = vec4(normalize(inWorldNormal) + 0.2, 1.);
}