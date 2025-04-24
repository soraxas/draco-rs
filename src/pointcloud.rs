use crate::{
    decode::{Decoder, DecoderBuffer},
    encode::{Encoder, EncoderBuffer},
    prelude::{
        ffi::{self},
        AttrId, DracoStatusType, StatusOr,
    },
};
use autocxx::prelude::*;

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

    pub fn add_point<T, const N: usize, Idx>(
        &mut self,
        attr_id: AttrId,
        point_index: Idx,
        point: &[T; N],
    ) where
        Idx: Into<ffi::draco::PointIndex>,
    {
        unsafe {
            self.0.pin_mut().SetAttributeValueForPoint(
                attr_id.0,
                point_index.into(),
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

    // This function returns the attribute id of the attribute with the given name
    // It stores the value in-place
    pub fn get_point<T, const N: usize, Idx>(
        &mut self,
        attr_id: AttrId,
        point_index: Idx,
        point_container: &mut [T; N],
    ) where
        T: Default + Copy,
        Idx: Into<ffi::draco::PointIndex>,
    {
        let pa_ptr = self.pc.pin_mut().GetAttributeByUniqueId(attr_id.as_u32());
        unsafe {
            (*pa_ptr).GetMappedValue(
                point_index.into(),
                point_container.as_mut_ptr() as *mut c_void,
            );
        };
    }

    // This function allocates a new array of type T and fills it with the point data
    pub fn get_point_alloc<T, const N: usize, Idx>(
        &mut self,
        attr_id: AttrId,
        point_index: Idx,
    ) -> [T; N]
    where
        T: Default + Copy,
        Idx: Into<ffi::draco::PointIndex>,
    {
        let mut point = [T::default(); N];
        self.get_point(attr_id, point_index, &mut point);
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

    pub fn to_buffer(&self, encoder: &mut Encoder) -> DracoStatusType<EncoderBuffer> {
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

    pub fn from_buffer(decoder: &mut Decoder, buffer: &mut DecoderBuffer) -> DracoStatusType<Self> {
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
