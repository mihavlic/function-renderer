struct Extent {
    width: u32,
    height: u32,
}

var<push_constant> size: Extent;
@group(0) @binding(0)
var output: texture_storage_2d<rgba8unorm, write>;

@compute @workgroup_size(8, 8)
fn main(
    @builtin(global_invocation_id) global_id: vec3<u32>
) {
    if global_id.x < size.width && global_id.y < size.height {
        let color = vec4(1.0, 0.0, 0.0, 1.0) * abs(sin(f32(global_id.x + global_id.y) / 20.0));
        textureStore(output, vec2<i32>(global_id.xy), color);
    }
}