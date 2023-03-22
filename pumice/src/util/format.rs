#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct FormatAspectBits {
    pub color: u16,
    pub depth: u16,
    pub stencil: u16,
    pub unused: u16,
}

macro_rules! aspects {
    ($c:literal, $d:literal, $s:literal, $u:literal) => {
        (
            crate::vk10::ImageAspectFlags::empty(),
            FormatAspectBits {
                color: $c,
                depth: $d,
                stencil: $s,
                unused: $u,
            },
        )
    };
    ($c:literal, $d:literal, $s:literal, $u:literal $(, $aspect:ident)+) => {
        (
            $(
                crate::vk10::ImageAspectFlags::$aspect
            )|+,
            FormatAspectBits {
                color: $c,
                depth: $d,
                stencil: $s,
                unused: $u,
            },
        )
    };
}

impl FormatAspectBits {
    pub fn total_bits(self) -> u32 {
        self.color as u32 + self.depth as u32 + self.stencil as u32 + self.unused as u32
    }
    pub fn total_bytes(self) -> u32 {
        let bits = self.total_bits();
        assert!(bits % 8 == 0, "Bits aren't a multiple of 8");
        bits / 8
    }
}
impl crate::vk10::Format {
    pub fn get_format_aspects(
        self,
    ) -> (crate::vk10::ImageAspectFlags, FormatAspectBits) {
        match self {
            Self::UNDEFINED => aspects!(0, 0, 0, 0),
            Self::R4G4_UNORM_PACK8 => aspects!(8, 0, 0, 0, COLOR),
            Self::R4G4B4A4_UNORM_PACK16 => aspects!(16, 0, 0, 0, COLOR),
            Self::B4G4R4A4_UNORM_PACK16 => aspects!(16, 0, 0, 0, COLOR),
            Self::R5G6B5_UNORM_PACK16 => aspects!(16, 0, 0, 0, COLOR),
            Self::B5G6R5_UNORM_PACK16 => aspects!(16, 0, 0, 0, COLOR),
            Self::R5G5B5A1_UNORM_PACK16 => aspects!(16, 0, 0, 0, COLOR),
            Self::B5G5R5A1_UNORM_PACK16 => aspects!(16, 0, 0, 0, COLOR),
            Self::A1R5G5B5_UNORM_PACK16 => aspects!(16, 0, 0, 0, COLOR),
            Self::R8_UNORM => aspects!(8, 0, 0, 0, COLOR),
            Self::R8_SNORM => aspects!(8, 0, 0, 0, COLOR),
            Self::R8_USCALED => aspects!(8, 0, 0, 0, COLOR),
            Self::R8_SSCALED => aspects!(8, 0, 0, 0, COLOR),
            Self::R8_UINT => aspects!(8, 0, 0, 0, COLOR),
            Self::R8_SINT => aspects!(8, 0, 0, 0, COLOR),
            Self::R8_SRGB => aspects!(8, 0, 0, 0, COLOR),
            Self::R8G8_UNORM => aspects!(16, 0, 0, 0, COLOR),
            Self::R8G8_SNORM => aspects!(16, 0, 0, 0, COLOR),
            Self::R8G8_USCALED => aspects!(16, 0, 0, 0, COLOR),
            Self::R8G8_SSCALED => aspects!(16, 0, 0, 0, COLOR),
            Self::R8G8_UINT => aspects!(16, 0, 0, 0, COLOR),
            Self::R8G8_SINT => aspects!(16, 0, 0, 0, COLOR),
            Self::R8G8_SRGB => aspects!(16, 0, 0, 0, COLOR),
            Self::R8G8B8_UNORM => aspects!(24, 0, 0, 0, COLOR),
            Self::R8G8B8_SNORM => aspects!(24, 0, 0, 0, COLOR),
            Self::R8G8B8_USCALED => aspects!(24, 0, 0, 0, COLOR),
            Self::R8G8B8_SSCALED => aspects!(24, 0, 0, 0, COLOR),
            Self::R8G8B8_UINT => aspects!(24, 0, 0, 0, COLOR),
            Self::R8G8B8_SINT => aspects!(24, 0, 0, 0, COLOR),
            Self::R8G8B8_SRGB => aspects!(24, 0, 0, 0, COLOR),
            Self::B8G8R8_UNORM => aspects!(24, 0, 0, 0, COLOR),
            Self::B8G8R8_SNORM => aspects!(24, 0, 0, 0, COLOR),
            Self::B8G8R8_USCALED => aspects!(24, 0, 0, 0, COLOR),
            Self::B8G8R8_SSCALED => aspects!(24, 0, 0, 0, COLOR),
            Self::B8G8R8_UINT => aspects!(24, 0, 0, 0, COLOR),
            Self::B8G8R8_SINT => aspects!(24, 0, 0, 0, COLOR),
            Self::B8G8R8_SRGB => aspects!(24, 0, 0, 0, COLOR),
            Self::R8G8B8A8_UNORM => aspects!(32, 0, 0, 0, COLOR),
            Self::R8G8B8A8_SNORM => aspects!(32, 0, 0, 0, COLOR),
            Self::R8G8B8A8_USCALED => aspects!(32, 0, 0, 0, COLOR),
            Self::R8G8B8A8_SSCALED => aspects!(32, 0, 0, 0, COLOR),
            Self::R8G8B8A8_UINT => aspects!(32, 0, 0, 0, COLOR),
            Self::R8G8B8A8_SINT => aspects!(32, 0, 0, 0, COLOR),
            Self::R8G8B8A8_SRGB => aspects!(32, 0, 0, 0, COLOR),
            Self::B8G8R8A8_UNORM => aspects!(32, 0, 0, 0, COLOR),
            Self::B8G8R8A8_SNORM => aspects!(32, 0, 0, 0, COLOR),
            Self::B8G8R8A8_USCALED => aspects!(32, 0, 0, 0, COLOR),
            Self::B8G8R8A8_SSCALED => aspects!(32, 0, 0, 0, COLOR),
            Self::B8G8R8A8_UINT => aspects!(32, 0, 0, 0, COLOR),
            Self::B8G8R8A8_SINT => aspects!(32, 0, 0, 0, COLOR),
            Self::B8G8R8A8_SRGB => aspects!(32, 0, 0, 0, COLOR),
            Self::A8B8G8R8_UNORM_PACK32 => aspects!(32, 0, 0, 0, COLOR),
            Self::A8B8G8R8_SNORM_PACK32 => aspects!(32, 0, 0, 0, COLOR),
            Self::A8B8G8R8_USCALED_PACK32 => aspects!(32, 0, 0, 0, COLOR),
            Self::A8B8G8R8_SSCALED_PACK32 => aspects!(32, 0, 0, 0, COLOR),
            Self::A8B8G8R8_UINT_PACK32 => aspects!(32, 0, 0, 0, COLOR),
            Self::A8B8G8R8_SINT_PACK32 => aspects!(32, 0, 0, 0, COLOR),
            Self::A8B8G8R8_SRGB_PACK32 => aspects!(32, 0, 0, 0, COLOR),
            Self::A2R10G10B10_UNORM_PACK32 => aspects!(32, 0, 0, 0, COLOR),
            Self::A2R10G10B10_SNORM_PACK32 => aspects!(32, 0, 0, 0, COLOR),
            Self::A2R10G10B10_USCALED_PACK32 => aspects!(32, 0, 0, 0, COLOR),
            Self::A2R10G10B10_SSCALED_PACK32 => aspects!(32, 0, 0, 0, COLOR),
            Self::A2R10G10B10_UINT_PACK32 => aspects!(32, 0, 0, 0, COLOR),
            Self::A2R10G10B10_SINT_PACK32 => aspects!(32, 0, 0, 0, COLOR),
            Self::A2B10G10R10_UNORM_PACK32 => aspects!(32, 0, 0, 0, COLOR),
            Self::A2B10G10R10_SNORM_PACK32 => aspects!(32, 0, 0, 0, COLOR),
            Self::A2B10G10R10_USCALED_PACK32 => aspects!(32, 0, 0, 0, COLOR),
            Self::A2B10G10R10_SSCALED_PACK32 => aspects!(32, 0, 0, 0, COLOR),
            Self::A2B10G10R10_UINT_PACK32 => aspects!(32, 0, 0, 0, COLOR),
            Self::A2B10G10R10_SINT_PACK32 => aspects!(32, 0, 0, 0, COLOR),
            Self::R16_UNORM => aspects!(16, 0, 0, 0, COLOR),
            Self::R16_SNORM => aspects!(16, 0, 0, 0, COLOR),
            Self::R16_USCALED => aspects!(16, 0, 0, 0, COLOR),
            Self::R16_SSCALED => aspects!(16, 0, 0, 0, COLOR),
            Self::R16_UINT => aspects!(16, 0, 0, 0, COLOR),
            Self::R16_SINT => aspects!(16, 0, 0, 0, COLOR),
            Self::R16_SFLOAT => aspects!(16, 0, 0, 0, COLOR),
            Self::R16G16_UNORM => aspects!(32, 0, 0, 0, COLOR),
            Self::R16G16_SNORM => aspects!(32, 0, 0, 0, COLOR),
            Self::R16G16_USCALED => aspects!(32, 0, 0, 0, COLOR),
            Self::R16G16_SSCALED => aspects!(32, 0, 0, 0, COLOR),
            Self::R16G16_UINT => aspects!(32, 0, 0, 0, COLOR),
            Self::R16G16_SINT => aspects!(32, 0, 0, 0, COLOR),
            Self::R16G16_SFLOAT => aspects!(32, 0, 0, 0, COLOR),
            Self::R16G16B16_UNORM => aspects!(48, 0, 0, 0, COLOR),
            Self::R16G16B16_SNORM => aspects!(48, 0, 0, 0, COLOR),
            Self::R16G16B16_USCALED => aspects!(48, 0, 0, 0, COLOR),
            Self::R16G16B16_SSCALED => aspects!(48, 0, 0, 0, COLOR),
            Self::R16G16B16_UINT => aspects!(48, 0, 0, 0, COLOR),
            Self::R16G16B16_SINT => aspects!(48, 0, 0, 0, COLOR),
            Self::R16G16B16_SFLOAT => aspects!(48, 0, 0, 0, COLOR),
            Self::R16G16B16A16_UNORM => aspects!(64, 0, 0, 0, COLOR),
            Self::R16G16B16A16_SNORM => aspects!(64, 0, 0, 0, COLOR),
            Self::R16G16B16A16_USCALED => aspects!(64, 0, 0, 0, COLOR),
            Self::R16G16B16A16_SSCALED => aspects!(64, 0, 0, 0, COLOR),
            Self::R16G16B16A16_UINT => aspects!(64, 0, 0, 0, COLOR),
            Self::R16G16B16A16_SINT => aspects!(64, 0, 0, 0, COLOR),
            Self::R16G16B16A16_SFLOAT => aspects!(64, 0, 0, 0, COLOR),
            Self::R32_UINT => aspects!(32, 0, 0, 0, COLOR),
            Self::R32_SINT => aspects!(32, 0, 0, 0, COLOR),
            Self::R32_SFLOAT => aspects!(32, 0, 0, 0, COLOR),
            Self::R32G32_UINT => aspects!(64, 0, 0, 0, COLOR),
            Self::R32G32_SINT => aspects!(64, 0, 0, 0, COLOR),
            Self::R32G32_SFLOAT => aspects!(64, 0, 0, 0, COLOR),
            Self::R32G32B32_UINT => aspects!(96, 0, 0, 0, COLOR),
            Self::R32G32B32_SINT => aspects!(96, 0, 0, 0, COLOR),
            Self::R32G32B32_SFLOAT => aspects!(96, 0, 0, 0, COLOR),
            Self::R32G32B32A32_UINT => aspects!(128, 0, 0, 0, COLOR),
            Self::R32G32B32A32_SINT => aspects!(128, 0, 0, 0, COLOR),
            Self::R32G32B32A32_SFLOAT => aspects!(128, 0, 0, 0, COLOR),
            Self::R64_UINT => aspects!(64, 0, 0, 0, COLOR),
            Self::R64_SINT => aspects!(64, 0, 0, 0, COLOR),
            Self::R64_SFLOAT => aspects!(64, 0, 0, 0, COLOR),
            Self::R64G64_UINT => aspects!(128, 0, 0, 0, COLOR),
            Self::R64G64_SINT => aspects!(128, 0, 0, 0, COLOR),
            Self::R64G64_SFLOAT => aspects!(128, 0, 0, 0, COLOR),
            Self::R64G64B64_UINT => aspects!(192, 0, 0, 0, COLOR),
            Self::R64G64B64_SINT => aspects!(192, 0, 0, 0, COLOR),
            Self::R64G64B64_SFLOAT => aspects!(192, 0, 0, 0, COLOR),
            Self::R64G64B64A64_UINT => aspects!(256, 0, 0, 0, COLOR),
            Self::R64G64B64A64_SINT => aspects!(256, 0, 0, 0, COLOR),
            Self::R64G64B64A64_SFLOAT => aspects!(256, 0, 0, 0, COLOR),
            Self::B10G11R11_UFLOAT_PACK32 => aspects!(32, 0, 0, 0, COLOR),
            Self::E5B9G9R9_UFLOAT_PACK32 => aspects!(27, 0, 0, 0, COLOR),
            Self::D16_UNORM => aspects!(0, 16, 0, 0, DEPTH),
            Self::X8_D24_UNORM_PACK32 => aspects!(0, 24, 0, 8, DEPTH),
            Self::D32_SFLOAT => aspects!(0, 32, 0, 0, DEPTH),
            Self::S8_UINT => aspects!(0, 0, 8, 0, STENCIL),
            Self::D16_UNORM_S8_UINT => aspects!(0, 16, 8, 0, DEPTH, STENCIL),
            Self::D24_UNORM_S8_UINT => aspects!(0, 24, 8, 0, DEPTH, STENCIL),
            Self::D32_SFLOAT_S8_UINT => aspects!(0, 32, 8, 0, DEPTH, STENCIL),
            Self::BC1_RGB_UNORM_BLOCK => aspects!(0, 0, 0, 0, COLOR),
            Self::BC1_RGB_SRGB_BLOCK => aspects!(0, 0, 0, 0, COLOR),
            Self::BC1_RGBA_UNORM_BLOCK => aspects!(0, 0, 0, 0, COLOR),
            Self::BC1_RGBA_SRGB_BLOCK => aspects!(0, 0, 0, 0, COLOR),
            Self::BC2_UNORM_BLOCK => aspects!(0, 0, 0, 0, COLOR),
            Self::BC2_SRGB_BLOCK => aspects!(0, 0, 0, 0, COLOR),
            Self::BC3_UNORM_BLOCK => aspects!(0, 0, 0, 0, COLOR),
            Self::BC3_SRGB_BLOCK => aspects!(0, 0, 0, 0, COLOR),
            Self::BC4_UNORM_BLOCK => aspects!(0, 0, 0, 0, COLOR),
            Self::BC4_SNORM_BLOCK => aspects!(0, 0, 0, 0, COLOR),
            Self::BC5_UNORM_BLOCK => aspects!(0, 0, 0, 0, COLOR),
            Self::BC5_SNORM_BLOCK => aspects!(0, 0, 0, 0, COLOR),
            Self::BC6H_UFLOAT_BLOCK => aspects!(0, 0, 0, 0, COLOR),
            Self::BC6H_SFLOAT_BLOCK => aspects!(0, 0, 0, 0, COLOR),
            Self::BC7_UNORM_BLOCK => aspects!(0, 0, 0, 0, COLOR),
            Self::BC7_SRGB_BLOCK => aspects!(0, 0, 0, 0, COLOR),
            Self::ETC2_R8G8B8_UNORM_BLOCK => aspects!(24, 0, 0, 0, COLOR),
            Self::ETC2_R8G8B8_SRGB_BLOCK => aspects!(24, 0, 0, 0, COLOR),
            Self::ETC2_R8G8B8A1_UNORM_BLOCK => aspects!(25, 0, 0, 0, COLOR),
            Self::ETC2_R8G8B8A1_SRGB_BLOCK => aspects!(25, 0, 0, 0, COLOR),
            Self::ETC2_R8G8B8A8_UNORM_BLOCK => aspects!(32, 0, 0, 0, COLOR),
            Self::ETC2_R8G8B8A8_SRGB_BLOCK => aspects!(32, 0, 0, 0, COLOR),
            Self::EAC_R11_UNORM_BLOCK => aspects!(11, 0, 0, 0, COLOR),
            Self::EAC_R11_SNORM_BLOCK => aspects!(11, 0, 0, 0, COLOR),
            Self::EAC_R11G11_UNORM_BLOCK => aspects!(22, 0, 0, 0, COLOR),
            Self::EAC_R11G11_SNORM_BLOCK => aspects!(22, 0, 0, 0, COLOR),
            Self::ASTC_4x4_UNORM_BLOCK => aspects!(0, 0, 0, 0, COLOR),
            Self::ASTC_4x4_SRGB_BLOCK => aspects!(0, 0, 0, 0, COLOR),
            Self::ASTC_5x4_UNORM_BLOCK => aspects!(0, 0, 0, 0, COLOR),
            Self::ASTC_5x4_SRGB_BLOCK => aspects!(0, 0, 0, 0, COLOR),
            Self::ASTC_5x5_UNORM_BLOCK => aspects!(0, 0, 0, 0, COLOR),
            Self::ASTC_5x5_SRGB_BLOCK => aspects!(0, 0, 0, 0, COLOR),
            Self::ASTC_6x5_UNORM_BLOCK => aspects!(0, 0, 0, 0, COLOR),
            Self::ASTC_6x5_SRGB_BLOCK => aspects!(0, 0, 0, 0, COLOR),
            Self::ASTC_6x6_UNORM_BLOCK => aspects!(0, 0, 0, 0, COLOR),
            Self::ASTC_6x6_SRGB_BLOCK => aspects!(0, 0, 0, 0, COLOR),
            Self::ASTC_8x5_UNORM_BLOCK => aspects!(0, 0, 0, 0, COLOR),
            Self::ASTC_8x5_SRGB_BLOCK => aspects!(0, 0, 0, 0, COLOR),
            Self::ASTC_8x6_UNORM_BLOCK => aspects!(0, 0, 0, 0, COLOR),
            Self::ASTC_8x6_SRGB_BLOCK => aspects!(0, 0, 0, 0, COLOR),
            Self::ASTC_8x8_UNORM_BLOCK => aspects!(0, 0, 0, 0, COLOR),
            Self::ASTC_8x8_SRGB_BLOCK => aspects!(0, 0, 0, 0, COLOR),
            Self::ASTC_10x5_UNORM_BLOCK => aspects!(0, 0, 0, 0, COLOR),
            Self::ASTC_10x5_SRGB_BLOCK => aspects!(0, 0, 0, 0, COLOR),
            Self::ASTC_10x6_UNORM_BLOCK => aspects!(0, 0, 0, 0, COLOR),
            Self::ASTC_10x6_SRGB_BLOCK => aspects!(0, 0, 0, 0, COLOR),
            Self::ASTC_10x8_UNORM_BLOCK => aspects!(0, 0, 0, 0, COLOR),
            Self::ASTC_10x8_SRGB_BLOCK => aspects!(0, 0, 0, 0, COLOR),
            Self::ASTC_10x10_UNORM_BLOCK => aspects!(0, 0, 0, 0, COLOR),
            Self::ASTC_10x10_SRGB_BLOCK => aspects!(0, 0, 0, 0, COLOR),
            Self::ASTC_12x10_UNORM_BLOCK => aspects!(0, 0, 0, 0, COLOR),
            Self::ASTC_12x10_SRGB_BLOCK => aspects!(0, 0, 0, 0, COLOR),
            Self::ASTC_12x12_UNORM_BLOCK => aspects!(0, 0, 0, 0, COLOR),
            Self::ASTC_12x12_SRGB_BLOCK => aspects!(0, 0, 0, 0, COLOR),
            _ => panic!("Unknown format {}", self.0),
        }
    }
}
