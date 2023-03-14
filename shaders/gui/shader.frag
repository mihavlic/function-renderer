#version 450

layout(constant_id = 0) const uint srgb_textures = 0;
layout(constant_id = 1) const uint output_srgb_fragment = 0;
layout(binding = 0, set = 0) uniform sampler2D font_texture;

layout(location = 0) in vec4 v_color_in_gamma;
layout(location = 1) in vec2 v_tex_coords;

layout(location = 0) out vec4 f_color;

// 0-1 sRGB gamma from 0-1 linear
vec3 srgb_gamma_from_linear(vec3 rgb) {
    bvec3 cutoff = lessThan(rgb, vec3(0.0031308));
    vec3 lower = rgb * vec3(12.92);
    vec3 higher = vec3(1.055) * pow(rgb, vec3(1.0 / 2.4)) - vec3(0.055);
    return mix(higher, lower, vec3(cutoff));
}

// 0-1 sRGBA gamma from 0-1 linear
vec4 srgba_gamma_from_linear(vec4 rgba) {
    return vec4(srgb_gamma_from_linear(rgba.rgb), rgba.a);
}

// 0-1 linear from 0-1 sRGBA gamma
// https://gamedev.stackexchange.com/a/194038
vec3 linear_from_srgb_gamma(vec3 rgb) {
    bvec3 cutoff = lessThan(rgb, vec3(0.04045));
    vec3 higher = pow((rgb + vec3(0.055)) / vec3(1.055), vec3(2.4));
    vec3 lower = rgb / vec3(12.92);
    return mix(higher, lower, cutoff);
}

// 0-1 linear from 0-1 sRGBA gamma
// https://gamedev.stackexchange.com/a/194038
vec4 linear_from_srgba_gamma(vec4 rgba) {
    return vec4(linear_from_srgb_gamma(rgba.rgb), rgba.a);
}

void main() {
    vec4 texture_in_gamma = texture(font_texture, v_tex_coords);

    // we need the color in gamma space, however if vulkan knows that the texture is in sRGB
    // it will automatically convert it to linear when sampling, we need to undo this
    if (srgb_textures == 1) {
        texture_in_gamma = srgba_gamma_from_linear(texture_in_gamma);
    }

    // We multiply the colors in gamma space, because that's the only way to get text to look right.
    vec4 color = v_color_in_gamma * texture_in_gamma;

    // if the target we're rendering to is not vk::Format::*_SRGB and the attachment is a swapchain
    // image with vk::ColorSpace::NON_LINEAR, we must must output in sRGB, otherwise in linear
    // https://stackoverflow.com/a/66401423
    if (output_srgb_fragment == 1) {
        f_color = color;
    } else {
        f_color = linear_from_srgba_gamma(color);
    }
}
