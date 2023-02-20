#version 450
#extension GL_ARB_separate_shader_objects : enable
#extension GL_EXT_scalar_block_layout : enable

layout(push_constant, scalar) uniform PushConstant { 
    mat4 model_matrix;
    mat4 projection_matrix;
} push;

layout(location = 0) in vec3 inPosition;
layout(location = 1) in vec4 inNormal;

layout(location = 0) out vec3 outWorldPos;
layout(location = 1) out vec3 outWorldNormal;
layout(location = 2) out vec3 outViewNormal;

vec3 uint_to_norm_vec3(uint packed) {
    const uint I10 = (1 << 11) - 1;
    const uint I11 = (1 << 12) - 1;
    const float F10 = float(I10);
    const float F11 = float(I11);
    
    float r = float(packed & I11) / F11;
    float g = float((packed >> 11) & I11) / F11;
    float b = float((packed >> 22) & I10) / F10;
    
    return vec3(r, g, b) * 2.0 - 1.0;
}

void main() {
    vec4 pos = push.projection_matrix * push.model_matrix * vec4(inPosition, 1.0);

    outWorldPos = inPosition;
    outWorldNormal = inNormal.xyz; // uint_to_norm_vec3(inNormal);
    outViewNormal = (push.model_matrix * vec4(outWorldNormal, 1.0) ).xyz;
    gl_Position = pos;
}