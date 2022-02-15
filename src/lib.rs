extern crate spirq;

use std::ffi::c_void;
use std::os::raw::c_char;

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SqBool32 {
    False = 0,
    True = 1,
}

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SqResult {
    Success,
    ErrorCorruptedSpirv = -1,
    ErrorUnsupportedSpirv = -2,
}

#[repr(C)]
pub struct SqReflectConfig {
    pub spirv: *const u32,
    pub spirv_size: usize,
    pub ref_all_rscs: SqBool32,
    pub combine_img_samplers: SqBool32,
}

#[repr(u32)]
pub enum SqAccessType {
    /// The variable can be accessed by read.
    ReadOnly = 1,
    /// The variable can be accessed by write.
    WriteOnly = 2,
    /// The variable can be accessed by read or by write.
    ReadWrite = 3,
}

#[repr(C)]
pub enum SqDescriptorType {
    /// `VK_DESCRIPTOR_TYPE_SAMPLER`
    Sampler(),
    /// `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`
    CombinedImageSampler(),
    /// `VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE`
    SampledImage(),
    /// `VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`
    StorageImage(SqAccessType),
    /// `VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER`.
    UniformTexelBuffer(),
    /// `VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER`.
    StorageTexelBuffer(SqAccessType),
    /// `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER` or
    /// `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC` depending on how you gonna
    /// use it.
    UniformBuffer(),
    /// `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER` or
    /// `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC` depending on how you gonna
    /// use it.
    StorageBuffer(SqAccessType),
    /// `VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT` and its input attachment index.
    InputAttachment(u32),
    /// `VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_KHR`
    AccelStruct(),
}

#[repr(u32)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub enum SqScalarRepr {
    Boolean,
    SignedInt,
    UnsignedInt,
    SignedFloat,
}

#[repr(C)]
pub struct SqScalarType {
    repr: SqScalarRepr,
    nbyte: u32,
}
#[repr(u32)]
pub enum SqMatrixAxisOrder {
    ColumnMajor,
    RowMajor,
}
#[repr(C)]
pub struct SqMatrixType {
    pub vec_ty: SqScalarType,
    pub nvec: u32,
    pub stride: usize,
    pub major: SqMatrixAxisOrder,
}

#[repr(C)]
pub struct SqStructMember {
    pub name: *const c_char,
    pub offset: usize,
    pub ty: *const SqType,
}

#[repr(u32)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub enum SqImageFormat {
    Rgba32f,
    Rgba16f,
    R32f,
    Rgba8,
    Rgba8Snorm,
    Rg32f,
    Rg16f,
    R11fG11fB10f,
    R16f,
    Rgba16,
    Rgb10A2,
    Rg16,
    Rg8,
    R16,
    R8,
    Rgba16Snorm,
    Rg16Snorm,
    Rg8Snorm,
    R16Snorm,
    R8Snorm,
    Rgba32i,
    Rgba16i,
    Rgba8i,
    R32i,
    Rg32i,
    Rg16i,
    Rg8i,
    R16i,
    R8i,
    Rgba32ui,
    Rgba16ui,
    Rgba8ui,
    R32ui,
    Rgb10a2ui,
    Rg32ui,
    Rg16ui,
    Rg8ui,
    R16ui,
    R8ui,
}
#[repr(u32)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub enum SqImageArrangement {
    Image1D,
    Image2D,
    Image2DMS,
    Image3D,
    CubeMap,
    Image1DArray,
    Image2DArray,
    Image2DMSArray,
    CubeMapArray,
    Image2DRect,
    ImageBuffer,
}
#[repr(u32)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub enum SqSubpassDataArrangement {
    SubpassData,
    SubpassDataMS,
}

#[repr(C)]
pub enum SqType {
    Void,
    Numeric {
        scalar_ty: SqScalarType,
        nlane: u32,
    },
    Matrix {
        scalar_ty: SqScalarType,
        nrow: u32,
        ncolumn: u32,
    },
    Image {
        scalar_ty: SqScalarType,
        fmt: SqImageFormat,
        arng: SqImageArrangement,
    },
    SampledImage {
        scalar_ty: SqScalarType,
        is_depth: SqBool32,
        arng: SqImageArrangement,
    },
    Sampler,
    SubpassData {
        arng: SqSubpassDataArrangement,
    },
    Array {
        proto_ty: *const SqType,
        nelem: u32,
        stride: usize,
    },
    Struct {
        name: *const c_char,
        members: *const SqStructMember,
    },
    AccelStruct,
}

#[repr(C)]
pub enum SqVariable {
    Input {
        name: *const c_char,
        location: u32,
        component: u32,
        ty: *const SqType,
    },
    Output {
        name: *const c_char,
        location: u32,
        component: u32,
        ty: *const SqType,
    },
    Descriptor {
        name: *const c_char,
        binding: u32,
        set: u32,
        desc_ty: SqDescriptorType,
        ty: *const SqType,
        nbind: u32,
    },
    PushConstant {
        name: *const c_char,
        ty: *const SqType,
    },
    SpecConstant {
        name: *const c_char,
        spec_id: u32,
        ty: *const SqType,
    }
}


#[repr(C)]
pub struct SqEntryPoint {
    name: *const c_char,
    vars: *const SqVariable,
    base_size: usize,
    dyn_stride: usize,
}

#[repr(C)]
pub struct SqSpirvQuery {
    entry_points: *const SqEntryPoint,
    nentry_point: u32,
}

#[no_mangle]
pub unsafe extern "C" fn sqCreateReflectQuery(
    cfg: &SqReflectConfig,
    query: &mut SqSpirvQuery
) -> SqResult {
    let entry_points = spirq::ReflectConfig::new()
        .spv(std::slice::from_raw_parts(cfg.spirv, cfg.spirv_size))
        .combine_img_samplers(cfg.combine_img_samplers != SqBool32::False)
        .ref_all_rscs(cfg.ref_all_rscs != SqBool32::False)
        .reflect();

    let entry_points = match entry_points {
        Err(spirq::Error::CorruptedSpirv(_)) => {
            return SqResult::ErrorCorruptedSpirv;
        },
        Err(spirq::Error::UnsupportedSpirv(_)) => {
            return SqResult::ErrorUnsupportedSpirv;
        },
        Ok(x) => x,
    };

    *query = SqSpirvQuery {
        entry_points: Box::new(entry_points).leak() as *mut _,
    };
    SqResult::Success
}

pub unsafe extern "C" fn sqEnumerateSpirvEntryPoints(
    query: &mut SqSpirvQuery,
    entry_points: u32,
    nentry_point: u32,
) -> SqResult {
    SqResult::Success
}
pub unsafe extern "C" fn sqEnumerateSpirvEntryPointVariables(
    que
)

#[no_mangle]
pub unsafe extern "C" fn sqDestroyReflectQuery(query: &mut SqSpirvQuery) {
    drop(query);
}
