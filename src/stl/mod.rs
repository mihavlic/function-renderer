/// Writes an indexed triangle list mesh to an stl,
/// header and attribute fields are zero
/// see https://en.wikipedia.org/wiki/STL_(file_format)#Binary_STL
pub fn write_stl(
    mut writer: impl std::io::Write,
    vertices: &[glam::Vec3],
    indices: &[u32],
) -> std::io::Result<()> {
    // round the length down to a multiple of 3
    let len = (indices.len() / 3) * 3;
    let indices = &indices[..len];

    // write 80 bytes of zeroes for the header
    writer.write_all(&[0u8; 80])?;

    let triangles = (indices.len() / 3).try_into().unwrap();
    write_element::<u32>(&mut writer, triangles)?;

    let mut i = 0;
    while i < indices.len() {
        let i1 = unsafe { *indices.get_unchecked(i) };
        let i2 = unsafe { *indices.get_unchecked(i + 1) };
        let i3 = unsafe { *indices.get_unchecked(i + 2) };

        let v1 = vertices[i1 as usize];
        let v2 = vertices[i2 as usize];
        let v3 = vertices[i3 as usize];

        let normal = {
            let s12 = v1 - v2;
            let s13 = v1 - v3;
            s12.cross(s13)
        };

        write_element(&mut writer, [normal, v1, v2, v3])?;
        write_element(&mut writer, 0u16)?;

        i += 3;
    }

    Ok(())
}

#[inline(always)]
fn write_element<T>(mut writer: impl std::io::Write, element: T) -> std::io::Result<()> {
    let ptr = &element as *const T as *const u8;
    let arr = unsafe { std::slice::from_raw_parts(ptr, std::mem::size_of::<T>()) };
    writer.write_all(arr)
}
