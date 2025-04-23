use crate::prelude::{
    ffi::{self},
    StatusOr,
};
use autocxx::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AttrId(pub c_int);

impl AttrId {
    pub fn as_u32(&self) -> u32 {
        self.0 .0 as u32
    }
}

impl From<c_int> for AttrId {
    fn from(value: c_int) -> Self {
        Self(value)
    }
}
impl From<AttrId> for c_int {
    fn from(value: AttrId) -> Self {
        value.0
    }
}

// pub enum DracoStatus<T> {
//     Ok(T),
//     FailedStatus(UniquePtr<ffi::draco::Status>),
// }

pub struct PointCloudBuilder {
    builder: UniquePtr<ffi::draco::PointCloudBuilder>,
}

impl PointCloudBuilder {
    pub fn new(num: u32) -> Self {
        let mut builder = ffi::draco::PointCloudBuilder::new().within_unique_ptr();
        builder.pin_mut().Start(num);
        Self { builder }
    }

    pub fn add_attribute(
        &mut self,
        attribute_type: ffi::draco::GeometryAttribute_Type,
        num_components: i8,
        data_type: ffi::draco::DataType,
    ) -> AttrId {
        self.builder
            .pin_mut()
            .AddAttribute(attribute_type, num_components, data_type)
            .into()
    }

    pub fn add_point(
        &mut self,
        attr_id: AttrId,
        point_index: ffi::draco::PointIndex,
        point: &[f64; 3],
    ) {
        unsafe {
            self.builder.pin_mut().SetAttributeValueForPoint(
                attr_id.0,
                point_index,
                point.as_ptr() as *const autocxx::c_void,
            );
        }
    }
    pub fn build(mut self, deduplicate_points: bool) -> PointCloud {
        PointCloud {
            pc: self.builder.pin_mut().Finalize(deduplicate_points),
        }
    }
}

pub struct PointCloud {
    pc: UniquePtr<ffi::draco::PointCloud>,
}
impl Default for PointCloud {
    fn default() -> Self {
        Self::new()
    }
}

impl PointCloud {
    pub fn new() -> Self {
        let pc = ffi::draco::PointCloud::new().within_unique_ptr();
        Self { pc }
    }

    pub fn get_point(&mut self, attr_id: AttrId, point_index: ffi::draco::PointIndex) -> [f64; 3] {
        let mut point = [0.0; 3];
        let pa_ptr = self.pc.pin_mut().GetAttributeByUniqueId(attr_id.as_u32());
        unsafe {
            (*pa_ptr).GetMappedValue(point_index, point.as_mut_ptr() as *mut autocxx::c_void);
        }
        point
    }

    pub fn num_named_attributes(&self, attr_type: ffi::draco::GeometryAttribute_Type) -> i32 {
        self.pc.NumNamedAttributes(attr_type)
    }

    pub fn num_points(&self) -> u32 {
        self.pc.num_points()
    }

    pub fn len(&self) -> u32 {
        self.pc.num_points()
    }

    pub fn is_empty(&self) -> bool {
        self.pc.num_points() == 0
    }

    pub fn to_buffer(
        &self,
        encoder: &mut Encoder,
    ) -> Result<EncoderBuffer, UniquePtr<ffi::draco::Status>> {
        let mut buffer = EncoderBuffer::new();

        let status = unsafe {
            encoder
                .encoder
                .pin_mut()
                .EncodePointCloudToBuffer(self.pc.as_ref().unwrap(), buffer.as_mut_ptr())
                .within_unique_ptr()
        };

        if status.ok() {
            Ok(buffer)
        } else {
            Err(status)
        }
    }

    pub fn from_buffer(
        decoder: &mut Decoder,
        buffer: &mut DecoderBuffer,
    ) -> Result<Self, UniquePtr<ffi::draco::Status>> {
        let mut status_or = unsafe {
            decoder
                .decoder
                .pin_mut()
                .DecodePointCloudFromBuffer(buffer.buffer.as_mut_ptr())
        };
        if status_or.ok() {
            Ok(Self {
                pc: status_or.pin_mut().value(),
            })
        } else {
            Err(status_or.status().within_unique_ptr())
        }
    }
}

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
                encoder_buffer.buffer.as_ref().unwrap().data() as *const i8,
                encoder_buffer.buffer.size(),
            )
        };
        Self { buffer }
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

    // pub fn decode_point_cloud(
    //     &mut self,
    //     buffer: &mut DecoderBuffer,
    // ) -> Result<PointCloud, UniquePtr<ffi::draco::Status>> {
    //     let pc = unsafe {
    //         self.decoder
    //             .pin_mut()
    //             .DecodePointCloudFromBuffer(buffer.as_mut_ptr())
    //     };
    //     if pc.ok() {
    //         Ok(PointCloud { pc })
    //     } else {
    //         Err(pc)
    //     }
    // }
}
