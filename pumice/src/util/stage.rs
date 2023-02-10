impl crate::extensions::khr_synchronization2::PipelineStageFlags2KHR {
    pub const UTIL_SPECIAL_FLAGS: Self = Self(
        Self::TOP_OF_PIPE.0 | Self::BOTTOM_OF_PIPE.0 | Self::VERTEX_INPUT.0
            | Self::PRE_RASTERIZATION_SHADERS.0 | Self::ALL_TRANSFER.0
            | Self::ALL_GRAPHICS.0,
    );
    pub const UTIL_ALL_COMMANDS_EQUIVALENT: Self = Self(
        Self::all().0 & !Self::UTIL_SPECIAL_FLAGS.0,
    );
    pub const UTIL_VERTEX_INPUT_EQUIVALENT: Self = Self(
        Self::INDEX_INPUT.0 | Self::VERTEX_ATTRIBUTE_INPUT.0,
    );
    pub const UTIL_PRE_RASTERIZATION_SHADERS_EQUIVALENT: Self = Self(
        Self::VERTEX_SHADER.0 | Self::TESSELLATION_CONTROL_SHADER.0
            | Self::TESSELLATION_EVALUATION_SHADER.0 | Self::GEOMETRY_SHADER.0,
    );
    pub const UTIL_ALL_TRANSFER_EQUIVALENT: Self = Self(
        Self::COPY.0 | Self::BLIT.0 | Self::RESOLVE.0 | Self::CLEAR.0,
    );
    pub const UTIL_ALL_GRAPHICS_EQUIVALENT: Self = Self(
        Self::DRAW_INDIRECT.0 | Self::VERTEX_INPUT.0 | Self::VERTEX_SHADER.0
            | Self::TESSELLATION_CONTROL_SHADER.0
            | Self::TESSELLATION_EVALUATION_SHADER.0 | Self::GEOMETRY_SHADER.0
            | Self::FRAGMENT_SHADER.0 | Self::EARLY_FRAGMENT_TESTS.0
            | Self::LATE_FRAGMENT_TESTS.0 | Self::COLOR_ATTACHMENT_OUTPUT.0,
    );
    pub const fn translate_special_bits(self) -> Self {
        let mut out = self.0 & Self::UTIL_ALL_COMMANDS_EQUIVALENT.0;
        if self.contains(Self::ALL_COMMANDS) {
            return Self::UTIL_ALL_COMMANDS_EQUIVALENT;
        }
        if self.contains(Self::VERTEX_INPUT) {
            out |= Self::UTIL_VERTEX_INPUT_EQUIVALENT.0;
        }
        if self.contains(Self::PRE_RASTERIZATION_SHADERS) {
            out |= Self::UTIL_PRE_RASTERIZATION_SHADERS_EQUIVALENT.0;
        }
        if self.contains(Self::ALL_TRANSFER) {
            out |= Self::UTIL_ALL_TRANSFER_EQUIVALENT.0;
        }
        if self.contains(Self::ALL_GRAPHICS) {
            out |= Self::UTIL_ALL_GRAPHICS_EQUIVALENT.0;
        }
        Self(out)
    }
}
