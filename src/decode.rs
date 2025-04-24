use crate::{
    encode::EncoderBuffer,
    prelude::ffi::{self},
};
use autocxx::prelude::*;

pub struct DecoderBuffer {
    pub(crate) buffer: UniquePtr<ffi::draco::DecoderBuffer>,
}

impl DecoderBuffer {
    pub fn new() -> Self {
        let buffer = ffi::draco::DecoderBuffer::new().within_unique_ptr();
        Self { buffer }
    }

    pub fn from_encoder_buffer(encoder_buffer: &mut EncoderBuffer) -> Self {
        let mut buffer = ffi::draco::DecoderBuffer::new().within_unique_ptr();
        unsafe {
            buffer.pin_mut().Init(
                encoder_buffer.buffer.as_ref().unwrap().data(),
                encoder_buffer.buffer.size(),
            )
        };
        Self { buffer }
    }
}

impl Default for DecoderBuffer {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Decoder {
    pub(crate) decoder: UniquePtr<ffi::draco::Decoder>,
}

impl Decoder {
    pub fn new() -> Self {
        let decoder = ffi::draco::Decoder::new().within_unique_ptr();
        Self { decoder }
    }
}
impl Default for Decoder {
    fn default() -> Self {
        Self::new()
    }
}
