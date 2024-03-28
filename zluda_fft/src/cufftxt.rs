use crate::cufft::*;

/* automatically generated by rust-bindgen 0.66.1 */

impl cudaDataType_t {
    pub const CUDA_R_16F: cudaDataType_t = cudaDataType_t(2);
}
impl cudaDataType_t {
    pub const CUDA_C_16F: cudaDataType_t = cudaDataType_t(6);
}
impl cudaDataType_t {
    pub const CUDA_R_16BF: cudaDataType_t = cudaDataType_t(14);
}
impl cudaDataType_t {
    pub const CUDA_C_16BF: cudaDataType_t = cudaDataType_t(15);
}
impl cudaDataType_t {
    pub const CUDA_R_32F: cudaDataType_t = cudaDataType_t(0);
}
impl cudaDataType_t {
    pub const CUDA_C_32F: cudaDataType_t = cudaDataType_t(4);
}
impl cudaDataType_t {
    pub const CUDA_R_64F: cudaDataType_t = cudaDataType_t(1);
}
impl cudaDataType_t {
    pub const CUDA_C_64F: cudaDataType_t = cudaDataType_t(5);
}
impl cudaDataType_t {
    pub const CUDA_R_4I: cudaDataType_t = cudaDataType_t(16);
}
impl cudaDataType_t {
    pub const CUDA_C_4I: cudaDataType_t = cudaDataType_t(17);
}
impl cudaDataType_t {
    pub const CUDA_R_4U: cudaDataType_t = cudaDataType_t(18);
}
impl cudaDataType_t {
    pub const CUDA_C_4U: cudaDataType_t = cudaDataType_t(19);
}
impl cudaDataType_t {
    pub const CUDA_R_8I: cudaDataType_t = cudaDataType_t(3);
}
impl cudaDataType_t {
    pub const CUDA_C_8I: cudaDataType_t = cudaDataType_t(7);
}
impl cudaDataType_t {
    pub const CUDA_R_8U: cudaDataType_t = cudaDataType_t(8);
}
impl cudaDataType_t {
    pub const CUDA_C_8U: cudaDataType_t = cudaDataType_t(9);
}
impl cudaDataType_t {
    pub const CUDA_R_16I: cudaDataType_t = cudaDataType_t(20);
}
impl cudaDataType_t {
    pub const CUDA_C_16I: cudaDataType_t = cudaDataType_t(21);
}
impl cudaDataType_t {
    pub const CUDA_R_16U: cudaDataType_t = cudaDataType_t(22);
}
impl cudaDataType_t {
    pub const CUDA_C_16U: cudaDataType_t = cudaDataType_t(23);
}
impl cudaDataType_t {
    pub const CUDA_R_32I: cudaDataType_t = cudaDataType_t(10);
}
impl cudaDataType_t {
    pub const CUDA_C_32I: cudaDataType_t = cudaDataType_t(11);
}
impl cudaDataType_t {
    pub const CUDA_R_32U: cudaDataType_t = cudaDataType_t(12);
}
impl cudaDataType_t {
    pub const CUDA_C_32U: cudaDataType_t = cudaDataType_t(13);
}
impl cudaDataType_t {
    pub const CUDA_R_64I: cudaDataType_t = cudaDataType_t(24);
}
impl cudaDataType_t {
    pub const CUDA_C_64I: cudaDataType_t = cudaDataType_t(25);
}
impl cudaDataType_t {
    pub const CUDA_R_64U: cudaDataType_t = cudaDataType_t(26);
}
impl cudaDataType_t {
    pub const CUDA_C_64U: cudaDataType_t = cudaDataType_t(27);
}
impl cudaDataType_t {
    pub const CUDA_R_8F_E4M3: cudaDataType_t = cudaDataType_t(28);
}
impl cudaDataType_t {
    pub const CUDA_R_8F_E5M2: cudaDataType_t = cudaDataType_t(29);
}
#[repr(transparent)]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudaDataType_t(pub ::std::os::raw::c_uint);
pub use self::cudaDataType_t as cudaDataType;
impl libFormat_t {
    pub const LIB_FORMAT_CUFFT: libFormat_t = libFormat_t(0);
}
impl libFormat_t {
    pub const LIB_FORMAT_UNDEFINED: libFormat_t = libFormat_t(1);
}
#[repr(transparent)]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct libFormat_t(pub ::std::os::raw::c_uint);
pub use self::libFormat_t as libFormat;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cudaXtDesc_t {
    pub version: ::std::os::raw::c_int,
    pub nGPUs: ::std::os::raw::c_int,
    pub GPUs: [::std::os::raw::c_int; 64usize],
    pub data: [*mut ::std::os::raw::c_void; 64usize],
    pub size: [usize; 64usize],
    pub cudaXtState: *mut ::std::os::raw::c_void,
}
pub type cudaXtDesc = cudaXtDesc_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cudaLibXtDesc_t {
    pub version: ::std::os::raw::c_int,
    pub descriptor: *mut cudaXtDesc,
    pub library: libFormat,
    pub subFormat: ::std::os::raw::c_int,
    pub libDescriptor: *mut ::std::os::raw::c_void,
}
pub type cudaLibXtDesc = cudaLibXtDesc_t;
impl cufftXtSubFormat_t {
    pub const CUFFT_XT_FORMAT_INPUT: cufftXtSubFormat_t = cufftXtSubFormat_t(0);
}
impl cufftXtSubFormat_t {
    pub const CUFFT_XT_FORMAT_OUTPUT: cufftXtSubFormat_t = cufftXtSubFormat_t(1);
}
impl cufftXtSubFormat_t {
    pub const CUFFT_XT_FORMAT_INPLACE: cufftXtSubFormat_t = cufftXtSubFormat_t(2);
}
impl cufftXtSubFormat_t {
    pub const CUFFT_XT_FORMAT_INPLACE_SHUFFLED: cufftXtSubFormat_t = cufftXtSubFormat_t(3);
}
impl cufftXtSubFormat_t {
    pub const CUFFT_XT_FORMAT_1D_INPUT_SHUFFLED: cufftXtSubFormat_t = cufftXtSubFormat_t(4);
}
impl cufftXtSubFormat_t {
    pub const CUFFT_XT_FORMAT_DISTRIBUTED_INPUT: cufftXtSubFormat_t = cufftXtSubFormat_t(5);
}
impl cufftXtSubFormat_t {
    pub const CUFFT_XT_FORMAT_DISTRIBUTED_OUTPUT: cufftXtSubFormat_t = cufftXtSubFormat_t(6);
}
impl cufftXtSubFormat_t {
    pub const CUFFT_FORMAT_UNDEFINED: cufftXtSubFormat_t = cufftXtSubFormat_t(7);
}
#[repr(transparent)]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct cufftXtSubFormat_t(pub ::std::os::raw::c_uint);
pub use self::cufftXtSubFormat_t as cufftXtSubFormat;
impl cufftXtCopyType_t {
    pub const CUFFT_COPY_HOST_TO_DEVICE: cufftXtCopyType_t = cufftXtCopyType_t(0);
}
impl cufftXtCopyType_t {
    pub const CUFFT_COPY_DEVICE_TO_HOST: cufftXtCopyType_t = cufftXtCopyType_t(1);
}
impl cufftXtCopyType_t {
    pub const CUFFT_COPY_DEVICE_TO_DEVICE: cufftXtCopyType_t = cufftXtCopyType_t(2);
}
impl cufftXtCopyType_t {
    pub const CUFFT_COPY_UNDEFINED: cufftXtCopyType_t = cufftXtCopyType_t(3);
}
#[repr(transparent)]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct cufftXtCopyType_t(pub ::std::os::raw::c_uint);
pub use self::cufftXtCopyType_t as cufftXtCopyType;
impl cufftXtQueryType_t {
    pub const CUFFT_QUERY_1D_FACTORS: cufftXtQueryType_t = cufftXtQueryType_t(0);
}
impl cufftXtQueryType_t {
    pub const CUFFT_QUERY_UNDEFINED: cufftXtQueryType_t = cufftXtQueryType_t(1);
}
#[repr(transparent)]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct cufftXtQueryType_t(pub ::std::os::raw::c_uint);
pub use self::cufftXtQueryType_t as cufftXtQueryType;
impl cufftXtWorkAreaPolicy_t {
    pub const CUFFT_WORKAREA_MINIMAL: cufftXtWorkAreaPolicy_t = cufftXtWorkAreaPolicy_t(0);
}
impl cufftXtWorkAreaPolicy_t {
    pub const CUFFT_WORKAREA_USER: cufftXtWorkAreaPolicy_t = cufftXtWorkAreaPolicy_t(1);
}
impl cufftXtWorkAreaPolicy_t {
    pub const CUFFT_WORKAREA_PERFORMANCE: cufftXtWorkAreaPolicy_t = cufftXtWorkAreaPolicy_t(2);
}
#[repr(transparent)]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct cufftXtWorkAreaPolicy_t(pub ::std::os::raw::c_uint);
pub use self::cufftXtWorkAreaPolicy_t as cufftXtWorkAreaPolicy;

#[no_mangle]
pub unsafe extern "system" fn cufftXtSetGPUs(
    handle: cufftHandle,
    nGPUs: ::std::os::raw::c_int,
    whichGPUs: *mut ::std::os::raw::c_int,
) -> cufftResult {
    crate::unsupported()
}

#[no_mangle]
pub unsafe extern "system" fn cufftXtMalloc(
    plan: cufftHandle,
    descriptor: *mut *mut cudaLibXtDesc,
    format: cufftXtSubFormat,
) -> cufftResult {
    crate::unsupported()
}

#[no_mangle]
pub unsafe extern "system" fn cufftXtMemcpy(
    plan: cufftHandle,
    dstPointer: *mut ::std::os::raw::c_void,
    srcPointer: *mut ::std::os::raw::c_void,
    type_: cufftXtCopyType,
) -> cufftResult {
    crate::unsupported()
}

#[no_mangle]
pub unsafe extern "system" fn cufftXtFree(descriptor: *mut cudaLibXtDesc) -> cufftResult {
    crate::unsupported()
}

#[no_mangle]
pub unsafe extern "system" fn cufftXtSetWorkArea(
    plan: cufftHandle,
    workArea: *mut *mut ::std::os::raw::c_void,
) -> cufftResult {
    crate::unsupported()
}

#[no_mangle]
pub unsafe extern "system" fn cufftXtExecDescriptorC2C(
    plan: cufftHandle,
    input: *mut cudaLibXtDesc,
    output: *mut cudaLibXtDesc,
    direction: ::std::os::raw::c_int,
) -> cufftResult {
    crate::unsupported()
}

#[no_mangle]
pub unsafe extern "system" fn cufftXtExecDescriptorR2C(
    plan: cufftHandle,
    input: *mut cudaLibXtDesc,
    output: *mut cudaLibXtDesc,
) -> cufftResult {
    crate::unsupported()
}

#[no_mangle]
pub unsafe extern "system" fn cufftXtExecDescriptorC2R(
    plan: cufftHandle,
    input: *mut cudaLibXtDesc,
    output: *mut cudaLibXtDesc,
) -> cufftResult {
    crate::unsupported()
}

#[no_mangle]
pub unsafe extern "system" fn cufftXtExecDescriptorZ2Z(
    plan: cufftHandle,
    input: *mut cudaLibXtDesc,
    output: *mut cudaLibXtDesc,
    direction: ::std::os::raw::c_int,
) -> cufftResult {
    crate::unsupported()
}

#[no_mangle]
pub unsafe extern "system" fn cufftXtExecDescriptorD2Z(
    plan: cufftHandle,
    input: *mut cudaLibXtDesc,
    output: *mut cudaLibXtDesc,
) -> cufftResult {
    crate::unsupported()
}

#[no_mangle]
pub unsafe extern "system" fn cufftXtExecDescriptorZ2D(
    plan: cufftHandle,
    input: *mut cudaLibXtDesc,
    output: *mut cudaLibXtDesc,
) -> cufftResult {
    crate::unsupported()
}

#[no_mangle]
pub unsafe extern "system" fn cufftXtQueryPlan(
    plan: cufftHandle,
    queryStruct: *mut ::std::os::raw::c_void,
    queryType: cufftXtQueryType,
) -> cufftResult {
    crate::unsupported()
}
impl cufftXtCallbackType_t {
    pub const CUFFT_CB_LD_COMPLEX: cufftXtCallbackType_t = cufftXtCallbackType_t(0);
}
impl cufftXtCallbackType_t {
    pub const CUFFT_CB_LD_COMPLEX_DOUBLE: cufftXtCallbackType_t = cufftXtCallbackType_t(1);
}
impl cufftXtCallbackType_t {
    pub const CUFFT_CB_LD_REAL: cufftXtCallbackType_t = cufftXtCallbackType_t(2);
}
impl cufftXtCallbackType_t {
    pub const CUFFT_CB_LD_REAL_DOUBLE: cufftXtCallbackType_t = cufftXtCallbackType_t(3);
}
impl cufftXtCallbackType_t {
    pub const CUFFT_CB_ST_COMPLEX: cufftXtCallbackType_t = cufftXtCallbackType_t(4);
}
impl cufftXtCallbackType_t {
    pub const CUFFT_CB_ST_COMPLEX_DOUBLE: cufftXtCallbackType_t = cufftXtCallbackType_t(5);
}
impl cufftXtCallbackType_t {
    pub const CUFFT_CB_ST_REAL: cufftXtCallbackType_t = cufftXtCallbackType_t(6);
}
impl cufftXtCallbackType_t {
    pub const CUFFT_CB_ST_REAL_DOUBLE: cufftXtCallbackType_t = cufftXtCallbackType_t(7);
}
impl cufftXtCallbackType_t {
    pub const CUFFT_CB_UNDEFINED: cufftXtCallbackType_t = cufftXtCallbackType_t(8);
}
#[repr(transparent)]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct cufftXtCallbackType_t(pub ::std::os::raw::c_uint);
pub use self::cufftXtCallbackType_t as cufftXtCallbackType;

#[no_mangle]
pub unsafe extern "system" fn cufftXtSetCallback(
    plan: cufftHandle,
    callback_routine: *mut *mut ::std::os::raw::c_void,
    cbType: cufftXtCallbackType,
    caller_info: *mut *mut ::std::os::raw::c_void,
) -> cufftResult {
    crate::unsupported()
}

#[no_mangle]
pub unsafe extern "system" fn cufftXtClearCallback(
    plan: cufftHandle,
    cbType: cufftXtCallbackType,
) -> cufftResult {
    crate::unsupported()
}

#[no_mangle]
pub unsafe extern "system" fn cufftXtSetCallbackSharedSize(
    plan: cufftHandle,
    cbType: cufftXtCallbackType,
    sharedSize: usize,
) -> cufftResult {
    crate::unsupported()
}

#[no_mangle]
pub unsafe extern "system" fn cufftXtMakePlanMany(
    plan: cufftHandle,
    rank: ::std::os::raw::c_int,
    n: *mut ::std::os::raw::c_longlong,
    inembed: *mut ::std::os::raw::c_longlong,
    istride: ::std::os::raw::c_longlong,
    idist: ::std::os::raw::c_longlong,
    inputtype: cudaDataType,
    onembed: *mut ::std::os::raw::c_longlong,
    ostride: ::std::os::raw::c_longlong,
    odist: ::std::os::raw::c_longlong,
    outputtype: cudaDataType,
    batch: ::std::os::raw::c_longlong,
    workSize: *mut usize,
    executiontype: cudaDataType,
) -> cufftResult {
    crate::unsupported()
}

#[no_mangle]
pub unsafe extern "system" fn cufftXtGetSizeMany(
    plan: cufftHandle,
    rank: ::std::os::raw::c_int,
    n: *mut ::std::os::raw::c_longlong,
    inembed: *mut ::std::os::raw::c_longlong,
    istride: ::std::os::raw::c_longlong,
    idist: ::std::os::raw::c_longlong,
    inputtype: cudaDataType,
    onembed: *mut ::std::os::raw::c_longlong,
    ostride: ::std::os::raw::c_longlong,
    odist: ::std::os::raw::c_longlong,
    outputtype: cudaDataType,
    batch: ::std::os::raw::c_longlong,
    workSize: *mut usize,
    executiontype: cudaDataType,
) -> cufftResult {
    crate::unsupported()
}

#[no_mangle]
pub unsafe extern "system" fn cufftXtExec(
    plan: cufftHandle,
    input: *mut ::std::os::raw::c_void,
    output: *mut ::std::os::raw::c_void,
    direction: ::std::os::raw::c_int,
) -> cufftResult {
    crate::unsupported()
}

#[no_mangle]
pub unsafe extern "system" fn cufftXtExecDescriptor(
    plan: cufftHandle,
    input: *mut cudaLibXtDesc,
    output: *mut cudaLibXtDesc,
    direction: ::std::os::raw::c_int,
) -> cufftResult {
    crate::unsupported()
}

#[no_mangle]
pub unsafe extern "system" fn cufftXtSetWorkAreaPolicy(
    plan: cufftHandle,
    policy: cufftXtWorkAreaPolicy,
    workSize: *mut usize,
) -> cufftResult {
    crate::unsupported()
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cufftBox3d_t {
    pub lower: [usize; 3usize],
    pub upper: [usize; 3usize],
    pub strides: [usize; 3usize],
}
pub type cufftBox3d = cufftBox3d_t;

#[no_mangle]
pub unsafe extern "system" fn cufftXtSetDistribution(
    plan: cufftHandle,
    box_in: *const cufftBox3d,
    box_out: *const cufftBox3d,
) -> cufftResult {
    crate::unsupported()
}