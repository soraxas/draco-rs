use crate::prelude::ffi::{self};
use autocxx::prelude::*;

pub struct Encoder {
    pub(crate) encoder: UniquePtr<ffi::draco::Encoder>,
}

impl Encoder {
    pub fn new() -> Self {
        let encoder = ffi::draco::Encoder::new().within_unique_ptr();
        Self { encoder }
    }

    pub fn set_attribute_quantization(
        mut self,
        attr: ffi::draco::GeometryAttribute_Type,
        num_bits: i32,
    ) -> Self {
        self.encoder
            .pin_mut()
            .SetAttributeQuantization(attr, num_bits.into());
        self
    }

    pub fn set_speed_options(mut self, encoding_speed: i32, decoding_speed: i32) -> Self {
        self.encoder
            .pin_mut()
            .SetSpeedOptions(encoding_speed.into(), decoding_speed.into());
        self
    }
}
impl Default for Encoder {
    fn default() -> Self {
        Self::new()
    }
}

pub struct EncoderBuffer {
    pub(crate) buffer: UniquePtr<ffi::draco::EncoderBuffer>,
}

impl EncoderBuffer {
    pub fn new() -> Self {
        let buffer = ffi::draco::EncoderBuffer::new().within_unique_ptr();
        Self { buffer }
    }

    pub fn as_mut_ptr(&mut self) -> *mut ffi::draco::EncoderBuffer {
        self.buffer.as_mut_ptr()
    }
}

impl Default for EncoderBuffer {
    fn default() -> Self {
        Self::new()
    }
}
