// LICENSE: It's not clear to me whether this should be licensed under the Batelle BSD license (like the parasail header file it's derived from), or under the MIT license that the remainder of this crate is licensed under. Consider it dual licensed unless there's a good legal reason not to, in which case please tell me!

extern crate libc;

use libc::{c_int, c_double, c_void, c_char};

#[repr(C)]
#[derive(Copy)]
pub struct Struct_parasail_result {
    pub saturated: c_int,
    pub score: c_int,
    pub matches: c_int,
    pub similar: c_int,
    pub length: c_int,
    pub end_query: c_int,
    pub end_ref: c_int,
    pub score_table: *mut c_int,
    pub matches_table: *mut c_int,
    pub similar_table: *mut c_int,
    pub length_table: *mut c_int,
    pub score_row: *mut c_int,
    pub matches_row: *mut c_int,
    pub similar_row: *mut c_int,
    pub length_row: *mut c_int,
    pub score_col: *mut c_int,
    pub matches_col: *mut c_int,
    pub similar_col: *mut c_int,
    pub length_col: *mut c_int,
}
impl ::std::clone::Clone for Struct_parasail_result {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct_parasail_result {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type ParasailResult = Struct_parasail_result;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_parasail_matrix {
    pub name: *const u8,
    pub matrix: *const c_int,
    pub mapper: *const c_int,
    pub size: c_int,
    pub max: c_int,
    pub min: c_int,
    pub need_free: c_int,
}
impl ::std::clone::Clone for Struct_parasail_matrix {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct_parasail_matrix {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type ParasailMatrix = Struct_parasail_matrix;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_parasail_profile_data {
    pub score: *mut c_void,
    pub matches: *mut c_void,
    pub similar: *mut c_void,
}
impl ::std::clone::Clone for Struct_parasail_profile_data {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct_parasail_profile_data {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type ParasailProfileData = Struct_parasail_profile_data;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_parasail_profile {
    pub s1: *const u8,
    pub s1_len: c_int,
    pub matrix: *const ParasailMatrix,
    pub profile8: Struct_parasail_profile_data,
    pub profile16: Struct_parasail_profile_data,
    pub profile32: Struct_parasail_profile_data,
    pub profile64: Struct_parasail_profile_data,
    pub free: ::std::option::Option<unsafe extern "C" fn(profile: *mut c_void)>,
    pub stop: c_int,
}
impl ::std::clone::Clone for Struct_parasail_profile {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct_parasail_profile {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type ParasailProfile = Struct_parasail_profile;
pub type ParasailFunction = unsafe extern "C" fn(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_parasail_function_info {
    pub pointer: *mut ::std::option::Option<extern "C" fn() -> *mut ParasailResult>,
    pub name: *const u8,
    pub alg: *const u8,
    pub _type: *const u8,
    pub isa: *const u8,
    pub bits: *const u8,
    pub width: *const u8,
    pub lanes: c_int,
    pub is_table: u8,
    pub is_rowcol: u8,
    pub is_stats: u8,
    pub is_ref: u8,
}
impl ::std::clone::Clone for Struct_parasail_function_info {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct_parasail_function_info {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type ParasailFunctionInfo = Struct_parasail_function_info;
pub type ParasailProfileFunction = unsafe extern "C" fn(profile: *const ParasailProfile,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int)
                                                        -> *mut ParasailResult;
pub type ParasailProfileCreator = unsafe extern "C" fn(s1: *const u8,
                                                       s1Len: c_int,
                                                       matrix: *const ParasailMatrix)
                                                       -> *mut ParasailProfile;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_parasail_pfunction_info {
    pub pointer: *mut ::std::option::Option<extern "C" fn() -> *mut ParasailResult>,
    pub creator: *mut ::std::option::Option<extern "C" fn() -> *mut ParasailProfile>,
    pub name: *const u8,
    pub alg: *const u8,
    pub _type: *const u8,
    pub isa: *const u8,
    pub bits: *const u8,
    pub width: *const u8,
    pub lanes: c_int,
    pub is_table: u8,
    pub is_rowcol: u8,
    pub is_stats: u8,
    pub is_ref: u8,
}
impl ::std::clone::Clone for Struct_parasail_pfunction_info {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct_parasail_pfunction_info {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type ParasailProfileFunctionInfo = Struct_parasail_pfunction_info;
extern "C" {
    pub fn parasail_profile_free(profile: *mut ParasailProfile);
    pub fn parasail_version(major: *mut c_int, minor: *mut c_int, patch: *mut c_int);
    pub fn parasail_result_free(result: *mut ParasailResult);
    pub fn parasail_lookup_function(funcname: *const u8)
     ->
         *mut ::std::option::Option<unsafe extern "C" fn(funcname:
                                                             *const u8)
                                        -> *mut ParasailResult>;
    pub fn parasail_lookup_pfunction(funcname: *const u8)
     ->
         *mut ::std::option::Option<unsafe extern "C" fn(funcname:
                                                             *const u8)
                                        -> *mut ParasailResult>;
    pub fn parasail_lookup_pcreator(funcname: *const u8)
     ->
         *mut ::std::option::Option<unsafe extern "C" fn(funcname:
                                                             *const u8)
                                        -> *mut ParasailProfile>;
    pub fn parasail_lookup_function_info(funcname: *const u8) -> *const ParasailFunctionInfo;
    pub fn parasail_lookup_pfunction_info(funcname: *const u8) -> *const ParasailProfileFunctionInfo;
    pub fn parasail_time() -> c_double;
    pub fn parasail_matrix_lookup(matrixname: *const c_char) -> *const ParasailMatrix;
    pub fn parasail_matrix_create(alphabet: *const c_char,
                                  _match: c_int,
                                  mismatch: c_int)
                                  -> *mut ParasailMatrix;
    pub fn parasail_matrix_free(matrix: *mut ParasailMatrix);
    pub fn parasail_nw(s1: *const u8,
                       s1Len: c_int,
                       s2: *const u8,
                       s2Len: c_int,
                       open: c_int,
                       gap: c_int,
                       matrix: *const ParasailMatrix)
                       -> *mut ParasailResult;
    pub fn parasail_nw_table(s1: *const u8,
                             s1Len: c_int,
                             s2: *const u8,
                             s2Len: c_int,
                             open: c_int,
                             gap: c_int,
                             matrix: *const ParasailMatrix)
                             -> *mut ParasailResult;
    pub fn parasail_nw_rowcol(s1: *const u8,
                              s1Len: c_int,
                              s2: *const u8,
                              s2Len: c_int,
                              open: c_int,
                              gap: c_int,
                              matrix: *const ParasailMatrix)
                              -> *mut ParasailResult;
    pub fn parasail_nw_stats(s1: *const u8,
                             s1Len: c_int,
                             s2: *const u8,
                             s2Len: c_int,
                             open: c_int,
                             gap: c_int,
                             matrix: *const ParasailMatrix)
                             -> *mut ParasailResult;
    pub fn parasail_nw_stats_table(s1: *const u8,
                                   s1Len: c_int,
                                   s2: *const u8,
                                   s2Len: c_int,
                                   open: c_int,
                                   gap: c_int,
                                   matrix: *const ParasailMatrix)
                                   -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol(s1: *const u8,
                                    s1Len: c_int,
                                    s2: *const u8,
                                    s2Len: c_int,
                                    open: c_int,
                                    gap: c_int,
                                    matrix: *const ParasailMatrix)
                                    -> *mut ParasailResult;
    pub fn parasail_sg(s1: *const u8,
                       s1Len: c_int,
                       s2: *const u8,
                       s2Len: c_int,
                       open: c_int,
                       gap: c_int,
                       matrix: *const ParasailMatrix)
                       -> *mut ParasailResult;
    pub fn parasail_sg_table(s1: *const u8,
                             s1Len: c_int,
                             s2: *const u8,
                             s2Len: c_int,
                             open: c_int,
                             gap: c_int,
                             matrix: *const ParasailMatrix)
                             -> *mut ParasailResult;
    pub fn parasail_sg_rowcol(s1: *const u8,
                              s1Len: c_int,
                              s2: *const u8,
                              s2Len: c_int,
                              open: c_int,
                              gap: c_int,
                              matrix: *const ParasailMatrix)
                              -> *mut ParasailResult;
    pub fn parasail_sg_stats(s1: *const u8,
                             s1Len: c_int,
                             s2: *const u8,
                             s2Len: c_int,
                             open: c_int,
                             gap: c_int,
                             matrix: *const ParasailMatrix)
                             -> *mut ParasailResult;
    pub fn parasail_sg_stats_table(s1: *const u8,
                                   s1Len: c_int,
                                   s2: *const u8,
                                   s2Len: c_int,
                                   open: c_int,
                                   gap: c_int,
                                   matrix: *const ParasailMatrix)
                                   -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol(s1: *const u8,
                                    s1Len: c_int,
                                    s2: *const u8,
                                    s2Len: c_int,
                                    open: c_int,
                                    gap: c_int,
                                    matrix: *const ParasailMatrix)
                                    -> *mut ParasailResult;
    pub fn parasail_sw(s1: *const u8,
                       s1Len: c_int,
                       s2: *const u8,
                       s2Len: c_int,
                       open: c_int,
                       gap: c_int,
                       matrix: *const ParasailMatrix)
                       -> *mut ParasailResult;
    pub fn parasail_sw_table(s1: *const u8,
                             s1Len: c_int,
                             s2: *const u8,
                             s2Len: c_int,
                             open: c_int,
                             gap: c_int,
                             matrix: *const ParasailMatrix)
                             -> *mut ParasailResult;
    pub fn parasail_sw_rowcol(s1: *const u8,
                              s1Len: c_int,
                              s2: *const u8,
                              s2Len: c_int,
                              open: c_int,
                              gap: c_int,
                              matrix: *const ParasailMatrix)
                              -> *mut ParasailResult;
    pub fn parasail_sw_stats(s1: *const u8,
                             s1Len: c_int,
                             s2: *const u8,
                             s2Len: c_int,
                             open: c_int,
                             gap: c_int,
                             matrix: *const ParasailMatrix)
                             -> *mut ParasailResult;
    pub fn parasail_sw_stats_table(s1: *const u8,
                                   s1Len: c_int,
                                   s2: *const u8,
                                   s2Len: c_int,
                                   open: c_int,
                                   gap: c_int,
                                   matrix: *const ParasailMatrix)
                                   -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol(s1: *const u8,
                                    s1Len: c_int,
                                    s2: *const u8,
                                    s2Len: c_int,
                                    open: c_int,
                                    gap: c_int,
                                    matrix: *const ParasailMatrix)
                                    -> *mut ParasailResult;
    pub fn parasail_nw_scan(s1: *const u8,
                            s1Len: c_int,
                            s2: *const u8,
                            s2Len: c_int,
                            open: c_int,
                            gap: c_int,
                            matrix: *const ParasailMatrix)
                            -> *mut ParasailResult;
    pub fn parasail_nw_table_scan(s1: *const u8,
                                  s1Len: c_int,
                                  s2: *const u8,
                                  s2Len: c_int,
                                  open: c_int,
                                  gap: c_int,
                                  matrix: *const ParasailMatrix)
                                  -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_scan(s1: *const u8,
                                   s1Len: c_int,
                                   s2: *const u8,
                                   s2Len: c_int,
                                   open: c_int,
                                   gap: c_int,
                                   matrix: *const ParasailMatrix)
                                   -> *mut ParasailResult;
    pub fn parasail_nw_stats_scan(s1: *const u8,
                                  s1Len: c_int,
                                  s2: *const u8,
                                  s2Len: c_int,
                                  open: c_int,
                                  gap: c_int,
                                  matrix: *const ParasailMatrix)
                                  -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_scan(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_scan(s1: *const u8,
                                         s1Len: c_int,
                                         s2: *const u8,
                                         s2Len: c_int,
                                         open: c_int,
                                         gap: c_int,
                                         matrix: *const ParasailMatrix)
                                         -> *mut ParasailResult;
    pub fn parasail_sg_scan(s1: *const u8,
                            s1Len: c_int,
                            s2: *const u8,
                            s2Len: c_int,
                            open: c_int,
                            gap: c_int,
                            matrix: *const ParasailMatrix)
                            -> *mut ParasailResult;
    pub fn parasail_sg_table_scan(s1: *const u8,
                                  s1Len: c_int,
                                  s2: *const u8,
                                  s2Len: c_int,
                                  open: c_int,
                                  gap: c_int,
                                  matrix: *const ParasailMatrix)
                                  -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_scan(s1: *const u8,
                                   s1Len: c_int,
                                   s2: *const u8,
                                   s2Len: c_int,
                                   open: c_int,
                                   gap: c_int,
                                   matrix: *const ParasailMatrix)
                                   -> *mut ParasailResult;
    pub fn parasail_sg_stats_scan(s1: *const u8,
                                  s1Len: c_int,
                                  s2: *const u8,
                                  s2Len: c_int,
                                  open: c_int,
                                  gap: c_int,
                                  matrix: *const ParasailMatrix)
                                  -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_scan(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_scan(s1: *const u8,
                                         s1Len: c_int,
                                         s2: *const u8,
                                         s2Len: c_int,
                                         open: c_int,
                                         gap: c_int,
                                         matrix: *const ParasailMatrix)
                                         -> *mut ParasailResult;
    pub fn parasail_sw_scan(s1: *const u8,
                            s1Len: c_int,
                            s2: *const u8,
                            s2Len: c_int,
                            open: c_int,
                            gap: c_int,
                            matrix: *const ParasailMatrix)
                            -> *mut ParasailResult;
    pub fn parasail_sw_table_scan(s1: *const u8,
                                  s1Len: c_int,
                                  s2: *const u8,
                                  s2Len: c_int,
                                  open: c_int,
                                  gap: c_int,
                                  matrix: *const ParasailMatrix)
                                  -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_scan(s1: *const u8,
                                   s1Len: c_int,
                                   s2: *const u8,
                                   s2Len: c_int,
                                   open: c_int,
                                   gap: c_int,
                                   matrix: *const ParasailMatrix)
                                   -> *mut ParasailResult;
    pub fn parasail_sw_stats_scan(s1: *const u8,
                                  s1Len: c_int,
                                  s2: *const u8,
                                  s2Len: c_int,
                                  open: c_int,
                                  gap: c_int,
                                  matrix: *const ParasailMatrix)
                                  -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_scan(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_scan(s1: *const u8,
                                         s1Len: c_int,
                                         s2: *const u8,
                                         s2Len: c_int,
                                         open: c_int,
                                         gap: c_int,
                                         matrix: *const ParasailMatrix)
                                         -> *mut ParasailResult;
    pub fn parasail_nw_scan_sse2_128_64(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_nw_scan_sse2_128_32(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_nw_scan_sse2_128_16(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_nw_scan_sse2_128_8(s1: *const u8,
                                       s1Len: c_int,
                                       s2: *const u8,
                                       s2Len: c_int,
                                       open: c_int,
                                       gap: c_int,
                                       matrix: *const ParasailMatrix)
                                       -> *mut ParasailResult;
    pub fn parasail_nw_scan_sse2_128_sat(s1: *const u8,
                                         s1Len: c_int,
                                         s2: *const u8,
                                         s2Len: c_int,
                                         open: c_int,
                                         gap: c_int,
                                         matrix: *const ParasailMatrix)
                                         -> *mut ParasailResult;
    pub fn parasail_nw_scan_sse41_128_64(s1: *const u8,
                                         s1Len: c_int,
                                         s2: *const u8,
                                         s2Len: c_int,
                                         open: c_int,
                                         gap: c_int,
                                         matrix: *const ParasailMatrix)
                                         -> *mut ParasailResult;
    pub fn parasail_nw_scan_sse41_128_32(s1: *const u8,
                                         s1Len: c_int,
                                         s2: *const u8,
                                         s2Len: c_int,
                                         open: c_int,
                                         gap: c_int,
                                         matrix: *const ParasailMatrix)
                                         -> *mut ParasailResult;
    pub fn parasail_nw_scan_sse41_128_16(s1: *const u8,
                                         s1Len: c_int,
                                         s2: *const u8,
                                         s2Len: c_int,
                                         open: c_int,
                                         gap: c_int,
                                         matrix: *const ParasailMatrix)
                                         -> *mut ParasailResult;
    pub fn parasail_nw_scan_sse41_128_8(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_nw_scan_sse41_128_sat(s1: *const u8,
                                          s1Len: c_int,
                                          s2: *const u8,
                                          s2Len: c_int,
                                          open: c_int,
                                          gap: c_int,
                                          matrix: *const ParasailMatrix)
                                          -> *mut ParasailResult;
    pub fn parasail_nw_scan_avx2_256_64(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_nw_scan_avx2_256_32(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_nw_scan_avx2_256_16(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_nw_scan_avx2_256_8(s1: *const u8,
                                       s1Len: c_int,
                                       s2: *const u8,
                                       s2Len: c_int,
                                       open: c_int,
                                       gap: c_int,
                                       matrix: *const ParasailMatrix)
                                       -> *mut ParasailResult;
    pub fn parasail_nw_scan_avx2_256_sat(s1: *const u8,
                                         s1Len: c_int,
                                         s2: *const u8,
                                         s2Len: c_int,
                                         open: c_int,
                                         gap: c_int,
                                         matrix: *const ParasailMatrix)
                                         -> *mut ParasailResult;
    pub fn parasail_nw_scan_knc_512_32(s1: *const u8,
                                       s1Len: c_int,
                                       s2: *const u8,
                                       s2Len: c_int,
                                       open: c_int,
                                       gap: c_int,
                                       matrix: *const ParasailMatrix)
                                       -> *mut ParasailResult;
    pub fn parasail_nw_striped_sse2_128_64(s1: *const u8,
                                           s1Len: c_int,
                                           s2: *const u8,
                                           s2Len: c_int,
                                           open: c_int,
                                           gap: c_int,
                                           matrix: *const ParasailMatrix)
                                           -> *mut ParasailResult;
    pub fn parasail_nw_striped_sse2_128_32(s1: *const u8,
                                           s1Len: c_int,
                                           s2: *const u8,
                                           s2Len: c_int,
                                           open: c_int,
                                           gap: c_int,
                                           matrix: *const ParasailMatrix)
                                           -> *mut ParasailResult;
    pub fn parasail_nw_striped_sse2_128_16(s1: *const u8,
                                           s1Len: c_int,
                                           s2: *const u8,
                                           s2Len: c_int,
                                           open: c_int,
                                           gap: c_int,
                                           matrix: *const ParasailMatrix)
                                           -> *mut ParasailResult;
    pub fn parasail_nw_striped_sse2_128_8(s1: *const u8,
                                          s1Len: c_int,
                                          s2: *const u8,
                                          s2Len: c_int,
                                          open: c_int,
                                          gap: c_int,
                                          matrix: *const ParasailMatrix)
                                          -> *mut ParasailResult;
    pub fn parasail_nw_striped_sse2_128_sat(s1: *const u8,
                                            s1Len: c_int,
                                            s2: *const u8,
                                            s2Len: c_int,
                                            open: c_int,
                                            gap: c_int,
                                            matrix: *const ParasailMatrix)
                                            -> *mut ParasailResult;
    pub fn parasail_nw_striped_sse41_128_64(s1: *const u8,
                                            s1Len: c_int,
                                            s2: *const u8,
                                            s2Len: c_int,
                                            open: c_int,
                                            gap: c_int,
                                            matrix: *const ParasailMatrix)
                                            -> *mut ParasailResult;
    pub fn parasail_nw_striped_sse41_128_32(s1: *const u8,
                                            s1Len: c_int,
                                            s2: *const u8,
                                            s2Len: c_int,
                                            open: c_int,
                                            gap: c_int,
                                            matrix: *const ParasailMatrix)
                                            -> *mut ParasailResult;
    pub fn parasail_nw_striped_sse41_128_16(s1: *const u8,
                                            s1Len: c_int,
                                            s2: *const u8,
                                            s2Len: c_int,
                                            open: c_int,
                                            gap: c_int,
                                            matrix: *const ParasailMatrix)
                                            -> *mut ParasailResult;
    pub fn parasail_nw_striped_sse41_128_8(s1: *const u8,
                                           s1Len: c_int,
                                           s2: *const u8,
                                           s2Len: c_int,
                                           open: c_int,
                                           gap: c_int,
                                           matrix: *const ParasailMatrix)
                                           -> *mut ParasailResult;
    pub fn parasail_nw_striped_sse41_128_sat(s1: *const u8,
                                             s1Len: c_int,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int,
                                             matrix: *const ParasailMatrix)
                                             -> *mut ParasailResult;
    pub fn parasail_nw_striped_avx2_256_64(s1: *const u8,
                                           s1Len: c_int,
                                           s2: *const u8,
                                           s2Len: c_int,
                                           open: c_int,
                                           gap: c_int,
                                           matrix: *const ParasailMatrix)
                                           -> *mut ParasailResult;
    pub fn parasail_nw_striped_avx2_256_32(s1: *const u8,
                                           s1Len: c_int,
                                           s2: *const u8,
                                           s2Len: c_int,
                                           open: c_int,
                                           gap: c_int,
                                           matrix: *const ParasailMatrix)
                                           -> *mut ParasailResult;
    pub fn parasail_nw_striped_avx2_256_16(s1: *const u8,
                                           s1Len: c_int,
                                           s2: *const u8,
                                           s2Len: c_int,
                                           open: c_int,
                                           gap: c_int,
                                           matrix: *const ParasailMatrix)
                                           -> *mut ParasailResult;
    pub fn parasail_nw_striped_avx2_256_8(s1: *const u8,
                                          s1Len: c_int,
                                          s2: *const u8,
                                          s2Len: c_int,
                                          open: c_int,
                                          gap: c_int,
                                          matrix: *const ParasailMatrix)
                                          -> *mut ParasailResult;
    pub fn parasail_nw_striped_avx2_256_sat(s1: *const u8,
                                            s1Len: c_int,
                                            s2: *const u8,
                                            s2Len: c_int,
                                            open: c_int,
                                            gap: c_int,
                                            matrix: *const ParasailMatrix)
                                            -> *mut ParasailResult;
    pub fn parasail_nw_striped_knc_512_32(s1: *const u8,
                                          s1Len: c_int,
                                          s2: *const u8,
                                          s2Len: c_int,
                                          open: c_int,
                                          gap: c_int,
                                          matrix: *const ParasailMatrix)
                                          -> *mut ParasailResult;
    pub fn parasail_nw_diag_sse2_128_64(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_nw_diag_sse2_128_32(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_nw_diag_sse2_128_16(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_nw_diag_sse2_128_8(s1: *const u8,
                                       s1Len: c_int,
                                       s2: *const u8,
                                       s2Len: c_int,
                                       open: c_int,
                                       gap: c_int,
                                       matrix: *const ParasailMatrix)
                                       -> *mut ParasailResult;
    pub fn parasail_nw_diag_sse2_128_sat(s1: *const u8,
                                         s1Len: c_int,
                                         s2: *const u8,
                                         s2Len: c_int,
                                         open: c_int,
                                         gap: c_int,
                                         matrix: *const ParasailMatrix)
                                         -> *mut ParasailResult;
    pub fn parasail_nw_diag_sse41_128_64(s1: *const u8,
                                         s1Len: c_int,
                                         s2: *const u8,
                                         s2Len: c_int,
                                         open: c_int,
                                         gap: c_int,
                                         matrix: *const ParasailMatrix)
                                         -> *mut ParasailResult;
    pub fn parasail_nw_diag_sse41_128_32(s1: *const u8,
                                         s1Len: c_int,
                                         s2: *const u8,
                                         s2Len: c_int,
                                         open: c_int,
                                         gap: c_int,
                                         matrix: *const ParasailMatrix)
                                         -> *mut ParasailResult;
    pub fn parasail_nw_diag_sse41_128_16(s1: *const u8,
                                         s1Len: c_int,
                                         s2: *const u8,
                                         s2Len: c_int,
                                         open: c_int,
                                         gap: c_int,
                                         matrix: *const ParasailMatrix)
                                         -> *mut ParasailResult;
    pub fn parasail_nw_diag_sse41_128_8(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_nw_diag_sse41_128_sat(s1: *const u8,
                                          s1Len: c_int,
                                          s2: *const u8,
                                          s2Len: c_int,
                                          open: c_int,
                                          gap: c_int,
                                          matrix: *const ParasailMatrix)
                                          -> *mut ParasailResult;
    pub fn parasail_nw_diag_avx2_256_64(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_nw_diag_avx2_256_32(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_nw_diag_avx2_256_16(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_nw_diag_avx2_256_8(s1: *const u8,
                                       s1Len: c_int,
                                       s2: *const u8,
                                       s2Len: c_int,
                                       open: c_int,
                                       gap: c_int,
                                       matrix: *const ParasailMatrix)
                                       -> *mut ParasailResult;
    pub fn parasail_nw_diag_avx2_256_sat(s1: *const u8,
                                         s1Len: c_int,
                                         s2: *const u8,
                                         s2Len: c_int,
                                         open: c_int,
                                         gap: c_int,
                                         matrix: *const ParasailMatrix)
                                         -> *mut ParasailResult;
    pub fn parasail_nw_diag_knc_512_32(s1: *const u8,
                                       s1Len: c_int,
                                       s2: *const u8,
                                       s2Len: c_int,
                                       open: c_int,
                                       gap: c_int,
                                       matrix: *const ParasailMatrix)
                                       -> *mut ParasailResult;
    pub fn parasail_nw_table_scan_sse2_128_64(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_nw_table_scan_sse2_128_32(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_nw_table_scan_sse2_128_16(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_nw_table_scan_sse2_128_8(s1: *const u8,
                                             s1Len: c_int,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int,
                                             matrix: *const ParasailMatrix)
                                             -> *mut ParasailResult;
    pub fn parasail_nw_table_scan_sse2_128_sat(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_nw_table_scan_sse41_128_64(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_nw_table_scan_sse41_128_32(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_nw_table_scan_sse41_128_16(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_nw_table_scan_sse41_128_8(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_nw_table_scan_sse41_128_sat(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_nw_table_scan_avx2_256_64(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_nw_table_scan_avx2_256_32(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_nw_table_scan_avx2_256_16(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_nw_table_scan_avx2_256_8(s1: *const u8,
                                             s1Len: c_int,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int,
                                             matrix: *const ParasailMatrix)
                                             -> *mut ParasailResult;
    pub fn parasail_nw_table_scan_avx2_256_sat(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_nw_table_scan_knc_512_32(s1: *const u8,
                                             s1Len: c_int,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int,
                                             matrix: *const ParasailMatrix)
                                             -> *mut ParasailResult;
    pub fn parasail_nw_table_striped_sse2_128_64(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
    pub fn parasail_nw_table_striped_sse2_128_32(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
    pub fn parasail_nw_table_striped_sse2_128_16(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
    pub fn parasail_nw_table_striped_sse2_128_8(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_nw_table_striped_sse2_128_sat(s1: *const u8,
                                                  s1Len: c_int,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int,
                                                  matrix: *const ParasailMatrix)
                                                  -> *mut ParasailResult;
    pub fn parasail_nw_table_striped_sse41_128_64(s1: *const u8,
                                                  s1Len: c_int,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int,
                                                  matrix: *const ParasailMatrix)
                                                  -> *mut ParasailResult;
    pub fn parasail_nw_table_striped_sse41_128_32(s1: *const u8,
                                                  s1Len: c_int,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int,
                                                  matrix: *const ParasailMatrix)
                                                  -> *mut ParasailResult;
    pub fn parasail_nw_table_striped_sse41_128_16(s1: *const u8,
                                                  s1Len: c_int,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int,
                                                  matrix: *const ParasailMatrix)
                                                  -> *mut ParasailResult;
    pub fn parasail_nw_table_striped_sse41_128_8(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
    pub fn parasail_nw_table_striped_sse41_128_sat(s1: *const u8,
                                                   s1Len: c_int,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int,
                                                   matrix: *const ParasailMatrix)
                                                   -> *mut ParasailResult;
    pub fn parasail_nw_table_striped_avx2_256_64(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
    pub fn parasail_nw_table_striped_avx2_256_32(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
    pub fn parasail_nw_table_striped_avx2_256_16(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
    pub fn parasail_nw_table_striped_avx2_256_8(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_nw_table_striped_avx2_256_sat(s1: *const u8,
                                                  s1Len: c_int,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int,
                                                  matrix: *const ParasailMatrix)
                                                  -> *mut ParasailResult;
    pub fn parasail_nw_table_striped_knc_512_32(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_nw_table_diag_sse2_128_64(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_nw_table_diag_sse2_128_32(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_nw_table_diag_sse2_128_16(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_nw_table_diag_sse2_128_8(s1: *const u8,
                                             s1Len: c_int,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int,
                                             matrix: *const ParasailMatrix)
                                             -> *mut ParasailResult;
    pub fn parasail_nw_table_diag_sse2_128_sat(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_nw_table_diag_sse41_128_64(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_nw_table_diag_sse41_128_32(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_nw_table_diag_sse41_128_16(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_nw_table_diag_sse41_128_8(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_nw_table_diag_sse41_128_sat(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_nw_table_diag_avx2_256_64(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_nw_table_diag_avx2_256_32(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_nw_table_diag_avx2_256_16(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_nw_table_diag_avx2_256_8(s1: *const u8,
                                             s1Len: c_int,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int,
                                             matrix: *const ParasailMatrix)
                                             -> *mut ParasailResult;
    pub fn parasail_nw_table_diag_avx2_256_sat(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_nw_table_diag_knc_512_32(s1: *const u8,
                                             s1Len: c_int,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int,
                                             matrix: *const ParasailMatrix)
                                             -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_scan_sse2_128_64(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_scan_sse2_128_32(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_scan_sse2_128_16(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_scan_sse2_128_8(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_scan_sse2_128_sat(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_scan_sse41_128_64(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_scan_sse41_128_32(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_scan_sse41_128_16(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_scan_sse41_128_8(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_scan_sse41_128_sat(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_scan_avx2_256_64(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_scan_avx2_256_32(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_scan_avx2_256_16(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_scan_avx2_256_8(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_scan_avx2_256_sat(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_scan_knc_512_32(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_striped_sse2_128_64(s1: *const u8,
                                                  s1Len: c_int,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int,
                                                  matrix: *const ParasailMatrix)
                                                  -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_striped_sse2_128_32(s1: *const u8,
                                                  s1Len: c_int,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int,
                                                  matrix: *const ParasailMatrix)
                                                  -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_striped_sse2_128_16(s1: *const u8,
                                                  s1Len: c_int,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int,
                                                  matrix: *const ParasailMatrix)
                                                  -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_striped_sse2_128_8(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_striped_sse2_128_sat(s1: *const u8,
                                                   s1Len: c_int,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int,
                                                   matrix: *const ParasailMatrix)
                                                   -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_striped_sse41_128_64(s1: *const u8,
                                                   s1Len: c_int,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int,
                                                   matrix: *const ParasailMatrix)
                                                   -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_striped_sse41_128_32(s1: *const u8,
                                                   s1Len: c_int,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int,
                                                   matrix: *const ParasailMatrix)
                                                   -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_striped_sse41_128_16(s1: *const u8,
                                                   s1Len: c_int,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int,
                                                   matrix: *const ParasailMatrix)
                                                   -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_striped_sse41_128_8(s1: *const u8,
                                                  s1Len: c_int,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int,
                                                  matrix: *const ParasailMatrix)
                                                  -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_striped_sse41_128_sat(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_striped_avx2_256_64(s1: *const u8,
                                                  s1Len: c_int,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int,
                                                  matrix: *const ParasailMatrix)
                                                  -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_striped_avx2_256_32(s1: *const u8,
                                                  s1Len: c_int,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int,
                                                  matrix: *const ParasailMatrix)
                                                  -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_striped_avx2_256_16(s1: *const u8,
                                                  s1Len: c_int,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int,
                                                  matrix: *const ParasailMatrix)
                                                  -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_striped_avx2_256_8(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_striped_avx2_256_sat(s1: *const u8,
                                                   s1Len: c_int,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int,
                                                   matrix: *const ParasailMatrix)
                                                   -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_striped_knc_512_32(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_diag_sse2_128_64(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_diag_sse2_128_32(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_diag_sse2_128_16(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_diag_sse2_128_8(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_diag_sse2_128_sat(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_diag_sse41_128_64(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_diag_sse41_128_32(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_diag_sse41_128_16(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_diag_sse41_128_8(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_diag_sse41_128_sat(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_diag_avx2_256_64(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_diag_avx2_256_32(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_diag_avx2_256_16(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_diag_avx2_256_8(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_diag_avx2_256_sat(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_diag_knc_512_32(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_nw_stats_scan_sse2_128_64(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_nw_stats_scan_sse2_128_32(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_nw_stats_scan_sse2_128_16(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_nw_stats_scan_sse2_128_8(s1: *const u8,
                                             s1Len: c_int,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int,
                                             matrix: *const ParasailMatrix)
                                             -> *mut ParasailResult;
    pub fn parasail_nw_stats_scan_sse2_128_sat(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_nw_stats_scan_sse41_128_64(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_nw_stats_scan_sse41_128_32(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_nw_stats_scan_sse41_128_16(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_nw_stats_scan_sse41_128_8(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_nw_stats_scan_sse41_128_sat(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_nw_stats_scan_avx2_256_64(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_nw_stats_scan_avx2_256_32(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_nw_stats_scan_avx2_256_16(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_nw_stats_scan_avx2_256_8(s1: *const u8,
                                             s1Len: c_int,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int,
                                             matrix: *const ParasailMatrix)
                                             -> *mut ParasailResult;
    pub fn parasail_nw_stats_scan_avx2_256_sat(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_nw_stats_scan_knc_512_32(s1: *const u8,
                                             s1Len: c_int,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int,
                                             matrix: *const ParasailMatrix)
                                             -> *mut ParasailResult;
    pub fn parasail_nw_stats_striped_sse2_128_64(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
    pub fn parasail_nw_stats_striped_sse2_128_32(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
    pub fn parasail_nw_stats_striped_sse2_128_16(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
    pub fn parasail_nw_stats_striped_sse2_128_8(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_nw_stats_striped_sse2_128_sat(s1: *const u8,
                                                  s1Len: c_int,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int,
                                                  matrix: *const ParasailMatrix)
                                                  -> *mut ParasailResult;
    pub fn parasail_nw_stats_striped_sse41_128_64(s1: *const u8,
                                                  s1Len: c_int,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int,
                                                  matrix: *const ParasailMatrix)
                                                  -> *mut ParasailResult;
    pub fn parasail_nw_stats_striped_sse41_128_32(s1: *const u8,
                                                  s1Len: c_int,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int,
                                                  matrix: *const ParasailMatrix)
                                                  -> *mut ParasailResult;
    pub fn parasail_nw_stats_striped_sse41_128_16(s1: *const u8,
                                                  s1Len: c_int,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int,
                                                  matrix: *const ParasailMatrix)
                                                  -> *mut ParasailResult;
    pub fn parasail_nw_stats_striped_sse41_128_8(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
    pub fn parasail_nw_stats_striped_sse41_128_sat(s1: *const u8,
                                                   s1Len: c_int,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int,
                                                   matrix: *const ParasailMatrix)
                                                   -> *mut ParasailResult;
    pub fn parasail_nw_stats_striped_avx2_256_64(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
    pub fn parasail_nw_stats_striped_avx2_256_32(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
    pub fn parasail_nw_stats_striped_avx2_256_16(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
    pub fn parasail_nw_stats_striped_avx2_256_8(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_nw_stats_striped_avx2_256_sat(s1: *const u8,
                                                  s1Len: c_int,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int,
                                                  matrix: *const ParasailMatrix)
                                                  -> *mut ParasailResult;
    pub fn parasail_nw_stats_striped_knc_512_32(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_nw_stats_diag_sse2_128_64(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_nw_stats_diag_sse2_128_32(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_nw_stats_diag_sse2_128_16(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_nw_stats_diag_sse2_128_8(s1: *const u8,
                                             s1Len: c_int,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int,
                                             matrix: *const ParasailMatrix)
                                             -> *mut ParasailResult;
    pub fn parasail_nw_stats_diag_sse2_128_sat(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_nw_stats_diag_sse41_128_64(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_nw_stats_diag_sse41_128_32(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_nw_stats_diag_sse41_128_16(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_nw_stats_diag_sse41_128_8(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_nw_stats_diag_sse41_128_sat(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_nw_stats_diag_avx2_256_64(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_nw_stats_diag_avx2_256_32(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_nw_stats_diag_avx2_256_16(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_nw_stats_diag_avx2_256_8(s1: *const u8,
                                             s1Len: c_int,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int,
                                             matrix: *const ParasailMatrix)
                                             -> *mut ParasailResult;
    pub fn parasail_nw_stats_diag_avx2_256_sat(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_nw_stats_diag_knc_512_32(s1: *const u8,
                                             s1Len: c_int,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int,
                                             matrix: *const ParasailMatrix)
                                             -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_scan_sse2_128_64(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_scan_sse2_128_32(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_scan_sse2_128_16(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_scan_sse2_128_8(s1: *const u8,
                                                   s1Len: c_int,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int,
                                                   matrix: *const ParasailMatrix)
                                                   -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_scan_sse2_128_sat(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_scan_sse41_128_64(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_scan_sse41_128_32(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_scan_sse41_128_16(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_scan_sse41_128_8(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_scan_sse41_128_sat(s1: *const u8,
                                                      s1Len: c_int,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int,
                                                      matrix: *const ParasailMatrix)
                                                      -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_scan_avx2_256_64(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_scan_avx2_256_32(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_scan_avx2_256_16(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_scan_avx2_256_8(s1: *const u8,
                                                   s1Len: c_int,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int,
                                                   matrix: *const ParasailMatrix)
                                                   -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_scan_avx2_256_sat(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_scan_knc_512_32(s1: *const u8,
                                                   s1Len: c_int,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int,
                                                   matrix: *const ParasailMatrix)
                                                   -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_striped_sse2_128_64(s1: *const u8,
                                                       s1Len: c_int,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int,
                                                       matrix: *const ParasailMatrix)
                                                       -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_striped_sse2_128_32(s1: *const u8,
                                                       s1Len: c_int,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int,
                                                       matrix: *const ParasailMatrix)
                                                       -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_striped_sse2_128_16(s1: *const u8,
                                                       s1Len: c_int,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int,
                                                       matrix: *const ParasailMatrix)
                                                       -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_striped_sse2_128_8(s1: *const u8,
                                                      s1Len: c_int,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int,
                                                      matrix: *const ParasailMatrix)
                                                      -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_striped_sse2_128_sat(s1: *const u8,
                                                        s1Len: c_int,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int,
                                                        matrix: *const ParasailMatrix)
                                                        -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_striped_sse41_128_64(s1: *const u8,
                                                        s1Len: c_int,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int,
                                                        matrix: *const ParasailMatrix)
                                                        -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_striped_sse41_128_32(s1: *const u8,
                                                        s1Len: c_int,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int,
                                                        matrix: *const ParasailMatrix)
                                                        -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_striped_sse41_128_16(s1: *const u8,
                                                        s1Len: c_int,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int,
                                                        matrix: *const ParasailMatrix)
                                                        -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_striped_sse41_128_8(s1: *const u8,
                                                       s1Len: c_int,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int,
                                                       matrix: *const ParasailMatrix)
                                                       -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_striped_sse41_128_sat(s1: *const u8,
                                                         s1Len: c_int,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int,
                                                         matrix: *const ParasailMatrix)
                                                         -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_striped_avx2_256_64(s1: *const u8,
                                                       s1Len: c_int,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int,
                                                       matrix: *const ParasailMatrix)
                                                       -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_striped_avx2_256_32(s1: *const u8,
                                                       s1Len: c_int,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int,
                                                       matrix: *const ParasailMatrix)
                                                       -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_striped_avx2_256_16(s1: *const u8,
                                                       s1Len: c_int,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int,
                                                       matrix: *const ParasailMatrix)
                                                       -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_striped_avx2_256_8(s1: *const u8,
                                                      s1Len: c_int,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int,
                                                      matrix: *const ParasailMatrix)
                                                      -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_striped_avx2_256_sat(s1: *const u8,
                                                        s1Len: c_int,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int,
                                                        matrix: *const ParasailMatrix)
                                                        -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_striped_knc_512_32(s1: *const u8,
                                                      s1Len: c_int,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int,
                                                      matrix: *const ParasailMatrix)
                                                      -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_diag_sse2_128_64(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_diag_sse2_128_32(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_diag_sse2_128_16(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_diag_sse2_128_8(s1: *const u8,
                                                   s1Len: c_int,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int,
                                                   matrix: *const ParasailMatrix)
                                                   -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_diag_sse2_128_sat(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_diag_sse41_128_64(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_diag_sse41_128_32(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_diag_sse41_128_16(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_diag_sse41_128_8(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_diag_sse41_128_sat(s1: *const u8,
                                                      s1Len: c_int,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int,
                                                      matrix: *const ParasailMatrix)
                                                      -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_diag_avx2_256_64(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_diag_avx2_256_32(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_diag_avx2_256_16(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_diag_avx2_256_8(s1: *const u8,
                                                   s1Len: c_int,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int,
                                                   matrix: *const ParasailMatrix)
                                                   -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_diag_avx2_256_sat(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_diag_knc_512_32(s1: *const u8,
                                                   s1Len: c_int,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int,
                                                   matrix: *const ParasailMatrix)
                                                   -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_scan_sse2_128_64(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_scan_sse2_128_32(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_scan_sse2_128_16(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_scan_sse2_128_8(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_scan_sse2_128_sat(s1: *const u8,
                                                      s1Len: c_int,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int,
                                                      matrix: *const ParasailMatrix)
                                                      -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_scan_sse41_128_64(s1: *const u8,
                                                      s1Len: c_int,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int,
                                                      matrix: *const ParasailMatrix)
                                                      -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_scan_sse41_128_32(s1: *const u8,
                                                      s1Len: c_int,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int,
                                                      matrix: *const ParasailMatrix)
                                                      -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_scan_sse41_128_16(s1: *const u8,
                                                      s1Len: c_int,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int,
                                                      matrix: *const ParasailMatrix)
                                                      -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_scan_sse41_128_8(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_scan_sse41_128_sat(s1: *const u8,
                                                       s1Len: c_int,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int,
                                                       matrix: *const ParasailMatrix)
                                                       -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_scan_avx2_256_64(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_scan_avx2_256_32(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_scan_avx2_256_16(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_scan_avx2_256_8(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_scan_avx2_256_sat(s1: *const u8,
                                                      s1Len: c_int,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int,
                                                      matrix: *const ParasailMatrix)
                                                      -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_scan_knc_512_32(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_striped_sse2_128_64(s1: *const u8,
                                                        s1Len: c_int,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int,
                                                        matrix: *const ParasailMatrix)
                                                        -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_striped_sse2_128_32(s1: *const u8,
                                                        s1Len: c_int,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int,
                                                        matrix: *const ParasailMatrix)
                                                        -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_striped_sse2_128_16(s1: *const u8,
                                                        s1Len: c_int,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int,
                                                        matrix: *const ParasailMatrix)
                                                        -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_striped_sse2_128_8(s1: *const u8,
                                                       s1Len: c_int,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int,
                                                       matrix: *const ParasailMatrix)
                                                       -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_striped_sse2_128_sat(s1: *const u8,
                                                         s1Len: c_int,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int,
                                                         matrix: *const ParasailMatrix)
                                                         -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_striped_sse41_128_64(s1: *const u8,
                                                         s1Len: c_int,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int,
                                                         matrix: *const ParasailMatrix)
                                                         -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_striped_sse41_128_32(s1: *const u8,
                                                         s1Len: c_int,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int,
                                                         matrix: *const ParasailMatrix)
                                                         -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_striped_sse41_128_16(s1: *const u8,
                                                         s1Len: c_int,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int,
                                                         matrix: *const ParasailMatrix)
                                                         -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_striped_sse41_128_8(s1: *const u8,
                                                        s1Len: c_int,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int,
                                                        matrix: *const ParasailMatrix)
                                                        -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_striped_sse41_128_sat(s1: *const u8,
                                                          s1Len: c_int,
                                                          s2: *const u8,
                                                          s2Len: c_int,
                                                          open: c_int,
                                                          gap: c_int,
                                                          matrix: *const ParasailMatrix)
                                                          -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_striped_avx2_256_64(s1: *const u8,
                                                        s1Len: c_int,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int,
                                                        matrix: *const ParasailMatrix)
                                                        -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_striped_avx2_256_32(s1: *const u8,
                                                        s1Len: c_int,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int,
                                                        matrix: *const ParasailMatrix)
                                                        -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_striped_avx2_256_16(s1: *const u8,
                                                        s1Len: c_int,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int,
                                                        matrix: *const ParasailMatrix)
                                                        -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_striped_avx2_256_8(s1: *const u8,
                                                       s1Len: c_int,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int,
                                                       matrix: *const ParasailMatrix)
                                                       -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_striped_avx2_256_sat(s1: *const u8,
                                                         s1Len: c_int,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int,
                                                         matrix: *const ParasailMatrix)
                                                         -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_striped_knc_512_32(s1: *const u8,
                                                       s1Len: c_int,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int,
                                                       matrix: *const ParasailMatrix)
                                                       -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_diag_sse2_128_64(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_diag_sse2_128_32(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_diag_sse2_128_16(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_diag_sse2_128_8(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_diag_sse2_128_sat(s1: *const u8,
                                                      s1Len: c_int,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int,
                                                      matrix: *const ParasailMatrix)
                                                      -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_diag_sse41_128_64(s1: *const u8,
                                                      s1Len: c_int,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int,
                                                      matrix: *const ParasailMatrix)
                                                      -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_diag_sse41_128_32(s1: *const u8,
                                                      s1Len: c_int,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int,
                                                      matrix: *const ParasailMatrix)
                                                      -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_diag_sse41_128_16(s1: *const u8,
                                                      s1Len: c_int,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int,
                                                      matrix: *const ParasailMatrix)
                                                      -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_diag_sse41_128_8(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_diag_sse41_128_sat(s1: *const u8,
                                                       s1Len: c_int,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int,
                                                       matrix: *const ParasailMatrix)
                                                       -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_diag_avx2_256_64(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_diag_avx2_256_32(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_diag_avx2_256_16(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_diag_avx2_256_8(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_diag_avx2_256_sat(s1: *const u8,
                                                      s1Len: c_int,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int,
                                                      matrix: *const ParasailMatrix)
                                                      -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_diag_knc_512_32(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_sg_scan_sse2_128_64(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_sg_scan_sse2_128_32(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_sg_scan_sse2_128_16(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_sg_scan_sse2_128_8(s1: *const u8,
                                       s1Len: c_int,
                                       s2: *const u8,
                                       s2Len: c_int,
                                       open: c_int,
                                       gap: c_int,
                                       matrix: *const ParasailMatrix)
                                       -> *mut ParasailResult;
    pub fn parasail_sg_scan_sse2_128_sat(s1: *const u8,
                                         s1Len: c_int,
                                         s2: *const u8,
                                         s2Len: c_int,
                                         open: c_int,
                                         gap: c_int,
                                         matrix: *const ParasailMatrix)
                                         -> *mut ParasailResult;
    pub fn parasail_sg_scan_sse41_128_64(s1: *const u8,
                                         s1Len: c_int,
                                         s2: *const u8,
                                         s2Len: c_int,
                                         open: c_int,
                                         gap: c_int,
                                         matrix: *const ParasailMatrix)
                                         -> *mut ParasailResult;
    pub fn parasail_sg_scan_sse41_128_32(s1: *const u8,
                                         s1Len: c_int,
                                         s2: *const u8,
                                         s2Len: c_int,
                                         open: c_int,
                                         gap: c_int,
                                         matrix: *const ParasailMatrix)
                                         -> *mut ParasailResult;
    pub fn parasail_sg_scan_sse41_128_16(s1: *const u8,
                                         s1Len: c_int,
                                         s2: *const u8,
                                         s2Len: c_int,
                                         open: c_int,
                                         gap: c_int,
                                         matrix: *const ParasailMatrix)
                                         -> *mut ParasailResult;
    pub fn parasail_sg_scan_sse41_128_8(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_sg_scan_sse41_128_sat(s1: *const u8,
                                          s1Len: c_int,
                                          s2: *const u8,
                                          s2Len: c_int,
                                          open: c_int,
                                          gap: c_int,
                                          matrix: *const ParasailMatrix)
                                          -> *mut ParasailResult;
    pub fn parasail_sg_scan_avx2_256_64(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_sg_scan_avx2_256_32(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_sg_scan_avx2_256_16(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_sg_scan_avx2_256_8(s1: *const u8,
                                       s1Len: c_int,
                                       s2: *const u8,
                                       s2Len: c_int,
                                       open: c_int,
                                       gap: c_int,
                                       matrix: *const ParasailMatrix)
                                       -> *mut ParasailResult;
    pub fn parasail_sg_scan_avx2_256_sat(s1: *const u8,
                                         s1Len: c_int,
                                         s2: *const u8,
                                         s2Len: c_int,
                                         open: c_int,
                                         gap: c_int,
                                         matrix: *const ParasailMatrix)
                                         -> *mut ParasailResult;
    pub fn parasail_sg_scan_knc_512_32(s1: *const u8,
                                       s1Len: c_int,
                                       s2: *const u8,
                                       s2Len: c_int,
                                       open: c_int,
                                       gap: c_int,
                                       matrix: *const ParasailMatrix)
                                       -> *mut ParasailResult;
    pub fn parasail_sg_striped_sse2_128_64(s1: *const u8,
                                           s1Len: c_int,
                                           s2: *const u8,
                                           s2Len: c_int,
                                           open: c_int,
                                           gap: c_int,
                                           matrix: *const ParasailMatrix)
                                           -> *mut ParasailResult;
    pub fn parasail_sg_striped_sse2_128_32(s1: *const u8,
                                           s1Len: c_int,
                                           s2: *const u8,
                                           s2Len: c_int,
                                           open: c_int,
                                           gap: c_int,
                                           matrix: *const ParasailMatrix)
                                           -> *mut ParasailResult;
    pub fn parasail_sg_striped_sse2_128_16(s1: *const u8,
                                           s1Len: c_int,
                                           s2: *const u8,
                                           s2Len: c_int,
                                           open: c_int,
                                           gap: c_int,
                                           matrix: *const ParasailMatrix)
                                           -> *mut ParasailResult;
    pub fn parasail_sg_striped_sse2_128_8(s1: *const u8,
                                          s1Len: c_int,
                                          s2: *const u8,
                                          s2Len: c_int,
                                          open: c_int,
                                          gap: c_int,
                                          matrix: *const ParasailMatrix)
                                          -> *mut ParasailResult;
    pub fn parasail_sg_striped_sse2_128_sat(s1: *const u8,
                                            s1Len: c_int,
                                            s2: *const u8,
                                            s2Len: c_int,
                                            open: c_int,
                                            gap: c_int,
                                            matrix: *const ParasailMatrix)
                                            -> *mut ParasailResult;
    pub fn parasail_sg_striped_sse41_128_64(s1: *const u8,
                                            s1Len: c_int,
                                            s2: *const u8,
                                            s2Len: c_int,
                                            open: c_int,
                                            gap: c_int,
                                            matrix: *const ParasailMatrix)
                                            -> *mut ParasailResult;
    pub fn parasail_sg_striped_sse41_128_32(s1: *const u8,
                                            s1Len: c_int,
                                            s2: *const u8,
                                            s2Len: c_int,
                                            open: c_int,
                                            gap: c_int,
                                            matrix: *const ParasailMatrix)
                                            -> *mut ParasailResult;
    pub fn parasail_sg_striped_sse41_128_16(s1: *const u8,
                                            s1Len: c_int,
                                            s2: *const u8,
                                            s2Len: c_int,
                                            open: c_int,
                                            gap: c_int,
                                            matrix: *const ParasailMatrix)
                                            -> *mut ParasailResult;
    pub fn parasail_sg_striped_sse41_128_8(s1: *const u8,
                                           s1Len: c_int,
                                           s2: *const u8,
                                           s2Len: c_int,
                                           open: c_int,
                                           gap: c_int,
                                           matrix: *const ParasailMatrix)
                                           -> *mut ParasailResult;
    pub fn parasail_sg_striped_sse41_128_sat(s1: *const u8,
                                             s1Len: c_int,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int,
                                             matrix: *const ParasailMatrix)
                                             -> *mut ParasailResult;
    pub fn parasail_sg_striped_avx2_256_64(s1: *const u8,
                                           s1Len: c_int,
                                           s2: *const u8,
                                           s2Len: c_int,
                                           open: c_int,
                                           gap: c_int,
                                           matrix: *const ParasailMatrix)
                                           -> *mut ParasailResult;
    pub fn parasail_sg_striped_avx2_256_32(s1: *const u8,
                                           s1Len: c_int,
                                           s2: *const u8,
                                           s2Len: c_int,
                                           open: c_int,
                                           gap: c_int,
                                           matrix: *const ParasailMatrix)
                                           -> *mut ParasailResult;
    pub fn parasail_sg_striped_avx2_256_16(s1: *const u8,
                                           s1Len: c_int,
                                           s2: *const u8,
                                           s2Len: c_int,
                                           open: c_int,
                                           gap: c_int,
                                           matrix: *const ParasailMatrix)
                                           -> *mut ParasailResult;
    pub fn parasail_sg_striped_avx2_256_8(s1: *const u8,
                                          s1Len: c_int,
                                          s2: *const u8,
                                          s2Len: c_int,
                                          open: c_int,
                                          gap: c_int,
                                          matrix: *const ParasailMatrix)
                                          -> *mut ParasailResult;
    pub fn parasail_sg_striped_avx2_256_sat(s1: *const u8,
                                            s1Len: c_int,
                                            s2: *const u8,
                                            s2Len: c_int,
                                            open: c_int,
                                            gap: c_int,
                                            matrix: *const ParasailMatrix)
                                            -> *mut ParasailResult;
    pub fn parasail_sg_striped_knc_512_32(s1: *const u8,
                                          s1Len: c_int,
                                          s2: *const u8,
                                          s2Len: c_int,
                                          open: c_int,
                                          gap: c_int,
                                          matrix: *const ParasailMatrix)
                                          -> *mut ParasailResult;
    pub fn parasail_sg_diag_sse2_128_64(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_sg_diag_sse2_128_32(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_sg_diag_sse2_128_16(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_sg_diag_sse2_128_8(s1: *const u8,
                                       s1Len: c_int,
                                       s2: *const u8,
                                       s2Len: c_int,
                                       open: c_int,
                                       gap: c_int,
                                       matrix: *const ParasailMatrix)
                                       -> *mut ParasailResult;
    pub fn parasail_sg_diag_sse2_128_sat(s1: *const u8,
                                         s1Len: c_int,
                                         s2: *const u8,
                                         s2Len: c_int,
                                         open: c_int,
                                         gap: c_int,
                                         matrix: *const ParasailMatrix)
                                         -> *mut ParasailResult;
    pub fn parasail_sg_diag_sse41_128_64(s1: *const u8,
                                         s1Len: c_int,
                                         s2: *const u8,
                                         s2Len: c_int,
                                         open: c_int,
                                         gap: c_int,
                                         matrix: *const ParasailMatrix)
                                         -> *mut ParasailResult;
    pub fn parasail_sg_diag_sse41_128_32(s1: *const u8,
                                         s1Len: c_int,
                                         s2: *const u8,
                                         s2Len: c_int,
                                         open: c_int,
                                         gap: c_int,
                                         matrix: *const ParasailMatrix)
                                         -> *mut ParasailResult;
    pub fn parasail_sg_diag_sse41_128_16(s1: *const u8,
                                         s1Len: c_int,
                                         s2: *const u8,
                                         s2Len: c_int,
                                         open: c_int,
                                         gap: c_int,
                                         matrix: *const ParasailMatrix)
                                         -> *mut ParasailResult;
    pub fn parasail_sg_diag_sse41_128_8(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_sg_diag_sse41_128_sat(s1: *const u8,
                                          s1Len: c_int,
                                          s2: *const u8,
                                          s2Len: c_int,
                                          open: c_int,
                                          gap: c_int,
                                          matrix: *const ParasailMatrix)
                                          -> *mut ParasailResult;
    pub fn parasail_sg_diag_avx2_256_64(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_sg_diag_avx2_256_32(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_sg_diag_avx2_256_16(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_sg_diag_avx2_256_8(s1: *const u8,
                                       s1Len: c_int,
                                       s2: *const u8,
                                       s2Len: c_int,
                                       open: c_int,
                                       gap: c_int,
                                       matrix: *const ParasailMatrix)
                                       -> *mut ParasailResult;
    pub fn parasail_sg_diag_avx2_256_sat(s1: *const u8,
                                         s1Len: c_int,
                                         s2: *const u8,
                                         s2Len: c_int,
                                         open: c_int,
                                         gap: c_int,
                                         matrix: *const ParasailMatrix)
                                         -> *mut ParasailResult;
    pub fn parasail_sg_diag_knc_512_32(s1: *const u8,
                                       s1Len: c_int,
                                       s2: *const u8,
                                       s2Len: c_int,
                                       open: c_int,
                                       gap: c_int,
                                       matrix: *const ParasailMatrix)
                                       -> *mut ParasailResult;
    pub fn parasail_sg_table_scan_sse2_128_64(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sg_table_scan_sse2_128_32(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sg_table_scan_sse2_128_16(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sg_table_scan_sse2_128_8(s1: *const u8,
                                             s1Len: c_int,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int,
                                             matrix: *const ParasailMatrix)
                                             -> *mut ParasailResult;
    pub fn parasail_sg_table_scan_sse2_128_sat(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sg_table_scan_sse41_128_64(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sg_table_scan_sse41_128_32(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sg_table_scan_sse41_128_16(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sg_table_scan_sse41_128_8(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sg_table_scan_sse41_128_sat(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_sg_table_scan_avx2_256_64(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sg_table_scan_avx2_256_32(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sg_table_scan_avx2_256_16(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sg_table_scan_avx2_256_8(s1: *const u8,
                                             s1Len: c_int,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int,
                                             matrix: *const ParasailMatrix)
                                             -> *mut ParasailResult;
    pub fn parasail_sg_table_scan_avx2_256_sat(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sg_table_scan_knc_512_32(s1: *const u8,
                                             s1Len: c_int,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int,
                                             matrix: *const ParasailMatrix)
                                             -> *mut ParasailResult;
    pub fn parasail_sg_table_striped_sse2_128_64(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
    pub fn parasail_sg_table_striped_sse2_128_32(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
    pub fn parasail_sg_table_striped_sse2_128_16(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
    pub fn parasail_sg_table_striped_sse2_128_8(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_sg_table_striped_sse2_128_sat(s1: *const u8,
                                                  s1Len: c_int,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int,
                                                  matrix: *const ParasailMatrix)
                                                  -> *mut ParasailResult;
    pub fn parasail_sg_table_striped_sse41_128_64(s1: *const u8,
                                                  s1Len: c_int,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int,
                                                  matrix: *const ParasailMatrix)
                                                  -> *mut ParasailResult;
    pub fn parasail_sg_table_striped_sse41_128_32(s1: *const u8,
                                                  s1Len: c_int,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int,
                                                  matrix: *const ParasailMatrix)
                                                  -> *mut ParasailResult;
    pub fn parasail_sg_table_striped_sse41_128_16(s1: *const u8,
                                                  s1Len: c_int,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int,
                                                  matrix: *const ParasailMatrix)
                                                  -> *mut ParasailResult;
    pub fn parasail_sg_table_striped_sse41_128_8(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
    pub fn parasail_sg_table_striped_sse41_128_sat(s1: *const u8,
                                                   s1Len: c_int,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int,
                                                   matrix: *const ParasailMatrix)
                                                   -> *mut ParasailResult;
    pub fn parasail_sg_table_striped_avx2_256_64(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
    pub fn parasail_sg_table_striped_avx2_256_32(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
    pub fn parasail_sg_table_striped_avx2_256_16(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
    pub fn parasail_sg_table_striped_avx2_256_8(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_sg_table_striped_avx2_256_sat(s1: *const u8,
                                                  s1Len: c_int,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int,
                                                  matrix: *const ParasailMatrix)
                                                  -> *mut ParasailResult;
    pub fn parasail_sg_table_striped_knc_512_32(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_sg_table_diag_sse2_128_64(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sg_table_diag_sse2_128_32(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sg_table_diag_sse2_128_16(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sg_table_diag_sse2_128_8(s1: *const u8,
                                             s1Len: c_int,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int,
                                             matrix: *const ParasailMatrix)
                                             -> *mut ParasailResult;
    pub fn parasail_sg_table_diag_sse2_128_sat(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sg_table_diag_sse41_128_64(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sg_table_diag_sse41_128_32(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sg_table_diag_sse41_128_16(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sg_table_diag_sse41_128_8(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sg_table_diag_sse41_128_sat(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_sg_table_diag_avx2_256_64(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sg_table_diag_avx2_256_32(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sg_table_diag_avx2_256_16(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sg_table_diag_avx2_256_8(s1: *const u8,
                                             s1Len: c_int,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int,
                                             matrix: *const ParasailMatrix)
                                             -> *mut ParasailResult;
    pub fn parasail_sg_table_diag_avx2_256_sat(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sg_table_diag_knc_512_32(s1: *const u8,
                                             s1Len: c_int,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int,
                                             matrix: *const ParasailMatrix)
                                             -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_scan_sse2_128_64(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_scan_sse2_128_32(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_scan_sse2_128_16(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_scan_sse2_128_8(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_scan_sse2_128_sat(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_scan_sse41_128_64(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_scan_sse41_128_32(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_scan_sse41_128_16(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_scan_sse41_128_8(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_scan_sse41_128_sat(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_scan_avx2_256_64(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_scan_avx2_256_32(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_scan_avx2_256_16(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_scan_avx2_256_8(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_scan_avx2_256_sat(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_scan_knc_512_32(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_striped_sse2_128_64(s1: *const u8,
                                                  s1Len: c_int,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int,
                                                  matrix: *const ParasailMatrix)
                                                  -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_striped_sse2_128_32(s1: *const u8,
                                                  s1Len: c_int,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int,
                                                  matrix: *const ParasailMatrix)
                                                  -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_striped_sse2_128_16(s1: *const u8,
                                                  s1Len: c_int,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int,
                                                  matrix: *const ParasailMatrix)
                                                  -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_striped_sse2_128_8(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_striped_sse2_128_sat(s1: *const u8,
                                                   s1Len: c_int,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int,
                                                   matrix: *const ParasailMatrix)
                                                   -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_striped_sse41_128_64(s1: *const u8,
                                                   s1Len: c_int,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int,
                                                   matrix: *const ParasailMatrix)
                                                   -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_striped_sse41_128_32(s1: *const u8,
                                                   s1Len: c_int,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int,
                                                   matrix: *const ParasailMatrix)
                                                   -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_striped_sse41_128_16(s1: *const u8,
                                                   s1Len: c_int,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int,
                                                   matrix: *const ParasailMatrix)
                                                   -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_striped_sse41_128_8(s1: *const u8,
                                                  s1Len: c_int,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int,
                                                  matrix: *const ParasailMatrix)
                                                  -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_striped_sse41_128_sat(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_striped_avx2_256_64(s1: *const u8,
                                                  s1Len: c_int,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int,
                                                  matrix: *const ParasailMatrix)
                                                  -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_striped_avx2_256_32(s1: *const u8,
                                                  s1Len: c_int,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int,
                                                  matrix: *const ParasailMatrix)
                                                  -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_striped_avx2_256_16(s1: *const u8,
                                                  s1Len: c_int,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int,
                                                  matrix: *const ParasailMatrix)
                                                  -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_striped_avx2_256_8(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_striped_avx2_256_sat(s1: *const u8,
                                                   s1Len: c_int,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int,
                                                   matrix: *const ParasailMatrix)
                                                   -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_striped_knc_512_32(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_diag_sse2_128_64(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_diag_sse2_128_32(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_diag_sse2_128_16(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_diag_sse2_128_8(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_diag_sse2_128_sat(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_diag_sse41_128_64(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_diag_sse41_128_32(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_diag_sse41_128_16(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_diag_sse41_128_8(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_diag_sse41_128_sat(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_diag_avx2_256_64(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_diag_avx2_256_32(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_diag_avx2_256_16(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_diag_avx2_256_8(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_diag_avx2_256_sat(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_diag_knc_512_32(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sg_stats_scan_sse2_128_64(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sg_stats_scan_sse2_128_32(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sg_stats_scan_sse2_128_16(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sg_stats_scan_sse2_128_8(s1: *const u8,
                                             s1Len: c_int,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int,
                                             matrix: *const ParasailMatrix)
                                             -> *mut ParasailResult;
    pub fn parasail_sg_stats_scan_sse2_128_sat(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sg_stats_scan_sse41_128_64(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sg_stats_scan_sse41_128_32(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sg_stats_scan_sse41_128_16(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sg_stats_scan_sse41_128_8(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sg_stats_scan_sse41_128_sat(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_sg_stats_scan_avx2_256_64(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sg_stats_scan_avx2_256_32(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sg_stats_scan_avx2_256_16(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sg_stats_scan_avx2_256_8(s1: *const u8,
                                             s1Len: c_int,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int,
                                             matrix: *const ParasailMatrix)
                                             -> *mut ParasailResult;
    pub fn parasail_sg_stats_scan_avx2_256_sat(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sg_stats_scan_knc_512_32(s1: *const u8,
                                             s1Len: c_int,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int,
                                             matrix: *const ParasailMatrix)
                                             -> *mut ParasailResult;
    pub fn parasail_sg_stats_striped_sse2_128_64(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
    pub fn parasail_sg_stats_striped_sse2_128_32(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
    pub fn parasail_sg_stats_striped_sse2_128_16(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
    pub fn parasail_sg_stats_striped_sse2_128_8(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_sg_stats_striped_sse2_128_sat(s1: *const u8,
                                                  s1Len: c_int,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int,
                                                  matrix: *const ParasailMatrix)
                                                  -> *mut ParasailResult;
    pub fn parasail_sg_stats_striped_sse41_128_64(s1: *const u8,
                                                  s1Len: c_int,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int,
                                                  matrix: *const ParasailMatrix)
                                                  -> *mut ParasailResult;
    pub fn parasail_sg_stats_striped_sse41_128_32(s1: *const u8,
                                                  s1Len: c_int,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int,
                                                  matrix: *const ParasailMatrix)
                                                  -> *mut ParasailResult;
    pub fn parasail_sg_stats_striped_sse41_128_16(s1: *const u8,
                                                  s1Len: c_int,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int,
                                                  matrix: *const ParasailMatrix)
                                                  -> *mut ParasailResult;
    pub fn parasail_sg_stats_striped_sse41_128_8(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
    pub fn parasail_sg_stats_striped_sse41_128_sat(s1: *const u8,
                                                   s1Len: c_int,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int,
                                                   matrix: *const ParasailMatrix)
                                                   -> *mut ParasailResult;
    pub fn parasail_sg_stats_striped_avx2_256_64(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
    pub fn parasail_sg_stats_striped_avx2_256_32(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
    pub fn parasail_sg_stats_striped_avx2_256_16(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
    pub fn parasail_sg_stats_striped_avx2_256_8(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_sg_stats_striped_avx2_256_sat(s1: *const u8,
                                                  s1Len: c_int,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int,
                                                  matrix: *const ParasailMatrix)
                                                  -> *mut ParasailResult;
    pub fn parasail_sg_stats_striped_knc_512_32(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_sg_stats_diag_sse2_128_64(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sg_stats_diag_sse2_128_32(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sg_stats_diag_sse2_128_16(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sg_stats_diag_sse2_128_8(s1: *const u8,
                                             s1Len: c_int,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int,
                                             matrix: *const ParasailMatrix)
                                             -> *mut ParasailResult;
    pub fn parasail_sg_stats_diag_sse2_128_sat(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sg_stats_diag_sse41_128_64(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sg_stats_diag_sse41_128_32(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sg_stats_diag_sse41_128_16(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sg_stats_diag_sse41_128_8(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sg_stats_diag_sse41_128_sat(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_sg_stats_diag_avx2_256_64(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sg_stats_diag_avx2_256_32(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sg_stats_diag_avx2_256_16(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sg_stats_diag_avx2_256_8(s1: *const u8,
                                             s1Len: c_int,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int,
                                             matrix: *const ParasailMatrix)
                                             -> *mut ParasailResult;
    pub fn parasail_sg_stats_diag_avx2_256_sat(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sg_stats_diag_knc_512_32(s1: *const u8,
                                             s1Len: c_int,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int,
                                             matrix: *const ParasailMatrix)
                                             -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_scan_sse2_128_64(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_scan_sse2_128_32(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_scan_sse2_128_16(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_scan_sse2_128_8(s1: *const u8,
                                                   s1Len: c_int,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int,
                                                   matrix: *const ParasailMatrix)
                                                   -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_scan_sse2_128_sat(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_scan_sse41_128_64(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_scan_sse41_128_32(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_scan_sse41_128_16(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_scan_sse41_128_8(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_scan_sse41_128_sat(s1: *const u8,
                                                      s1Len: c_int,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int,
                                                      matrix: *const ParasailMatrix)
                                                      -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_scan_avx2_256_64(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_scan_avx2_256_32(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_scan_avx2_256_16(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_scan_avx2_256_8(s1: *const u8,
                                                   s1Len: c_int,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int,
                                                   matrix: *const ParasailMatrix)
                                                   -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_scan_avx2_256_sat(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_scan_knc_512_32(s1: *const u8,
                                                   s1Len: c_int,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int,
                                                   matrix: *const ParasailMatrix)
                                                   -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_striped_sse2_128_64(s1: *const u8,
                                                       s1Len: c_int,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int,
                                                       matrix: *const ParasailMatrix)
                                                       -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_striped_sse2_128_32(s1: *const u8,
                                                       s1Len: c_int,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int,
                                                       matrix: *const ParasailMatrix)
                                                       -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_striped_sse2_128_16(s1: *const u8,
                                                       s1Len: c_int,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int,
                                                       matrix: *const ParasailMatrix)
                                                       -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_striped_sse2_128_8(s1: *const u8,
                                                      s1Len: c_int,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int,
                                                      matrix: *const ParasailMatrix)
                                                      -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_striped_sse2_128_sat(s1: *const u8,
                                                        s1Len: c_int,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int,
                                                        matrix: *const ParasailMatrix)
                                                        -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_striped_sse41_128_64(s1: *const u8,
                                                        s1Len: c_int,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int,
                                                        matrix: *const ParasailMatrix)
                                                        -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_striped_sse41_128_32(s1: *const u8,
                                                        s1Len: c_int,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int,
                                                        matrix: *const ParasailMatrix)
                                                        -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_striped_sse41_128_16(s1: *const u8,
                                                        s1Len: c_int,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int,
                                                        matrix: *const ParasailMatrix)
                                                        -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_striped_sse41_128_8(s1: *const u8,
                                                       s1Len: c_int,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int,
                                                       matrix: *const ParasailMatrix)
                                                       -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_striped_sse41_128_sat(s1: *const u8,
                                                         s1Len: c_int,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int,
                                                         matrix: *const ParasailMatrix)
                                                         -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_striped_avx2_256_64(s1: *const u8,
                                                       s1Len: c_int,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int,
                                                       matrix: *const ParasailMatrix)
                                                       -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_striped_avx2_256_32(s1: *const u8,
                                                       s1Len: c_int,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int,
                                                       matrix: *const ParasailMatrix)
                                                       -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_striped_avx2_256_16(s1: *const u8,
                                                       s1Len: c_int,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int,
                                                       matrix: *const ParasailMatrix)
                                                       -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_striped_avx2_256_8(s1: *const u8,
                                                      s1Len: c_int,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int,
                                                      matrix: *const ParasailMatrix)
                                                      -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_striped_avx2_256_sat(s1: *const u8,
                                                        s1Len: c_int,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int,
                                                        matrix: *const ParasailMatrix)
                                                        -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_striped_knc_512_32(s1: *const u8,
                                                      s1Len: c_int,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int,
                                                      matrix: *const ParasailMatrix)
                                                      -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_diag_sse2_128_64(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_diag_sse2_128_32(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_diag_sse2_128_16(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_diag_sse2_128_8(s1: *const u8,
                                                   s1Len: c_int,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int,
                                                   matrix: *const ParasailMatrix)
                                                   -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_diag_sse2_128_sat(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_diag_sse41_128_64(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_diag_sse41_128_32(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_diag_sse41_128_16(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_diag_sse41_128_8(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_diag_sse41_128_sat(s1: *const u8,
                                                      s1Len: c_int,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int,
                                                      matrix: *const ParasailMatrix)
                                                      -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_diag_avx2_256_64(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_diag_avx2_256_32(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_diag_avx2_256_16(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_diag_avx2_256_8(s1: *const u8,
                                                   s1Len: c_int,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int,
                                                   matrix: *const ParasailMatrix)
                                                   -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_diag_avx2_256_sat(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_diag_knc_512_32(s1: *const u8,
                                                   s1Len: c_int,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int,
                                                   matrix: *const ParasailMatrix)
                                                   -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_scan_sse2_128_64(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_scan_sse2_128_32(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_scan_sse2_128_16(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_scan_sse2_128_8(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_scan_sse2_128_sat(s1: *const u8,
                                                      s1Len: c_int,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int,
                                                      matrix: *const ParasailMatrix)
                                                      -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_scan_sse41_128_64(s1: *const u8,
                                                      s1Len: c_int,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int,
                                                      matrix: *const ParasailMatrix)
                                                      -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_scan_sse41_128_32(s1: *const u8,
                                                      s1Len: c_int,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int,
                                                      matrix: *const ParasailMatrix)
                                                      -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_scan_sse41_128_16(s1: *const u8,
                                                      s1Len: c_int,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int,
                                                      matrix: *const ParasailMatrix)
                                                      -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_scan_sse41_128_8(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_scan_sse41_128_sat(s1: *const u8,
                                                       s1Len: c_int,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int,
                                                       matrix: *const ParasailMatrix)
                                                       -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_scan_avx2_256_64(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_scan_avx2_256_32(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_scan_avx2_256_16(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_scan_avx2_256_8(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_scan_avx2_256_sat(s1: *const u8,
                                                      s1Len: c_int,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int,
                                                      matrix: *const ParasailMatrix)
                                                      -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_scan_knc_512_32(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_striped_sse2_128_64(s1: *const u8,
                                                        s1Len: c_int,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int,
                                                        matrix: *const ParasailMatrix)
                                                        -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_striped_sse2_128_32(s1: *const u8,
                                                        s1Len: c_int,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int,
                                                        matrix: *const ParasailMatrix)
                                                        -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_striped_sse2_128_16(s1: *const u8,
                                                        s1Len: c_int,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int,
                                                        matrix: *const ParasailMatrix)
                                                        -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_striped_sse2_128_8(s1: *const u8,
                                                       s1Len: c_int,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int,
                                                       matrix: *const ParasailMatrix)
                                                       -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_striped_sse2_128_sat(s1: *const u8,
                                                         s1Len: c_int,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int,
                                                         matrix: *const ParasailMatrix)
                                                         -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_striped_sse41_128_64(s1: *const u8,
                                                         s1Len: c_int,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int,
                                                         matrix: *const ParasailMatrix)
                                                         -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_striped_sse41_128_32(s1: *const u8,
                                                         s1Len: c_int,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int,
                                                         matrix: *const ParasailMatrix)
                                                         -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_striped_sse41_128_16(s1: *const u8,
                                                         s1Len: c_int,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int,
                                                         matrix: *const ParasailMatrix)
                                                         -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_striped_sse41_128_8(s1: *const u8,
                                                        s1Len: c_int,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int,
                                                        matrix: *const ParasailMatrix)
                                                        -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_striped_sse41_128_sat(s1: *const u8,
                                                          s1Len: c_int,
                                                          s2: *const u8,
                                                          s2Len: c_int,
                                                          open: c_int,
                                                          gap: c_int,
                                                          matrix: *const ParasailMatrix)
                                                          -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_striped_avx2_256_64(s1: *const u8,
                                                        s1Len: c_int,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int,
                                                        matrix: *const ParasailMatrix)
                                                        -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_striped_avx2_256_32(s1: *const u8,
                                                        s1Len: c_int,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int,
                                                        matrix: *const ParasailMatrix)
                                                        -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_striped_avx2_256_16(s1: *const u8,
                                                        s1Len: c_int,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int,
                                                        matrix: *const ParasailMatrix)
                                                        -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_striped_avx2_256_8(s1: *const u8,
                                                       s1Len: c_int,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int,
                                                       matrix: *const ParasailMatrix)
                                                       -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_striped_avx2_256_sat(s1: *const u8,
                                                         s1Len: c_int,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int,
                                                         matrix: *const ParasailMatrix)
                                                         -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_striped_knc_512_32(s1: *const u8,
                                                       s1Len: c_int,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int,
                                                       matrix: *const ParasailMatrix)
                                                       -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_diag_sse2_128_64(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_diag_sse2_128_32(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_diag_sse2_128_16(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_diag_sse2_128_8(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_diag_sse2_128_sat(s1: *const u8,
                                                      s1Len: c_int,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int,
                                                      matrix: *const ParasailMatrix)
                                                      -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_diag_sse41_128_64(s1: *const u8,
                                                      s1Len: c_int,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int,
                                                      matrix: *const ParasailMatrix)
                                                      -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_diag_sse41_128_32(s1: *const u8,
                                                      s1Len: c_int,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int,
                                                      matrix: *const ParasailMatrix)
                                                      -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_diag_sse41_128_16(s1: *const u8,
                                                      s1Len: c_int,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int,
                                                      matrix: *const ParasailMatrix)
                                                      -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_diag_sse41_128_8(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_diag_sse41_128_sat(s1: *const u8,
                                                       s1Len: c_int,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int,
                                                       matrix: *const ParasailMatrix)
                                                       -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_diag_avx2_256_64(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_diag_avx2_256_32(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_diag_avx2_256_16(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_diag_avx2_256_8(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_diag_avx2_256_sat(s1: *const u8,
                                                      s1Len: c_int,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int,
                                                      matrix: *const ParasailMatrix)
                                                      -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_diag_knc_512_32(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_sw_scan_sse2_128_64(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_sw_scan_sse2_128_32(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_sw_scan_sse2_128_16(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_sw_scan_sse2_128_8(s1: *const u8,
                                       s1Len: c_int,
                                       s2: *const u8,
                                       s2Len: c_int,
                                       open: c_int,
                                       gap: c_int,
                                       matrix: *const ParasailMatrix)
                                       -> *mut ParasailResult;
    pub fn parasail_sw_scan_sse2_128_sat(s1: *const u8,
                                         s1Len: c_int,
                                         s2: *const u8,
                                         s2Len: c_int,
                                         open: c_int,
                                         gap: c_int,
                                         matrix: *const ParasailMatrix)
                                         -> *mut ParasailResult;
    pub fn parasail_sw_scan_sse41_128_64(s1: *const u8,
                                         s1Len: c_int,
                                         s2: *const u8,
                                         s2Len: c_int,
                                         open: c_int,
                                         gap: c_int,
                                         matrix: *const ParasailMatrix)
                                         -> *mut ParasailResult;
    pub fn parasail_sw_scan_sse41_128_32(s1: *const u8,
                                         s1Len: c_int,
                                         s2: *const u8,
                                         s2Len: c_int,
                                         open: c_int,
                                         gap: c_int,
                                         matrix: *const ParasailMatrix)
                                         -> *mut ParasailResult;
    pub fn parasail_sw_scan_sse41_128_16(s1: *const u8,
                                         s1Len: c_int,
                                         s2: *const u8,
                                         s2Len: c_int,
                                         open: c_int,
                                         gap: c_int,
                                         matrix: *const ParasailMatrix)
                                         -> *mut ParasailResult;
    pub fn parasail_sw_scan_sse41_128_8(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_sw_scan_sse41_128_sat(s1: *const u8,
                                          s1Len: c_int,
                                          s2: *const u8,
                                          s2Len: c_int,
                                          open: c_int,
                                          gap: c_int,
                                          matrix: *const ParasailMatrix)
                                          -> *mut ParasailResult;
    pub fn parasail_sw_scan_avx2_256_64(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_sw_scan_avx2_256_32(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_sw_scan_avx2_256_16(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_sw_scan_avx2_256_8(s1: *const u8,
                                       s1Len: c_int,
                                       s2: *const u8,
                                       s2Len: c_int,
                                       open: c_int,
                                       gap: c_int,
                                       matrix: *const ParasailMatrix)
                                       -> *mut ParasailResult;
    pub fn parasail_sw_scan_avx2_256_sat(s1: *const u8,
                                         s1Len: c_int,
                                         s2: *const u8,
                                         s2Len: c_int,
                                         open: c_int,
                                         gap: c_int,
                                         matrix: *const ParasailMatrix)
                                         -> *mut ParasailResult;
    pub fn parasail_sw_scan_knc_512_32(s1: *const u8,
                                       s1Len: c_int,
                                       s2: *const u8,
                                       s2Len: c_int,
                                       open: c_int,
                                       gap: c_int,
                                       matrix: *const ParasailMatrix)
                                       -> *mut ParasailResult;
    pub fn parasail_sw_striped_sse2_128_64(s1: *const u8,
                                           s1Len: c_int,
                                           s2: *const u8,
                                           s2Len: c_int,
                                           open: c_int,
                                           gap: c_int,
                                           matrix: *const ParasailMatrix)
                                           -> *mut ParasailResult;
    pub fn parasail_sw_striped_sse2_128_32(s1: *const u8,
                                           s1Len: c_int,
                                           s2: *const u8,
                                           s2Len: c_int,
                                           open: c_int,
                                           gap: c_int,
                                           matrix: *const ParasailMatrix)
                                           -> *mut ParasailResult;
    pub fn parasail_sw_striped_sse2_128_16(s1: *const u8,
                                           s1Len: c_int,
                                           s2: *const u8,
                                           s2Len: c_int,
                                           open: c_int,
                                           gap: c_int,
                                           matrix: *const ParasailMatrix)
                                           -> *mut ParasailResult;
    pub fn parasail_sw_striped_sse2_128_8(s1: *const u8,
                                          s1Len: c_int,
                                          s2: *const u8,
                                          s2Len: c_int,
                                          open: c_int,
                                          gap: c_int,
                                          matrix: *const ParasailMatrix)
                                          -> *mut ParasailResult;
    pub fn parasail_sw_striped_sse2_128_sat(s1: *const u8,
                                            s1Len: c_int,
                                            s2: *const u8,
                                            s2Len: c_int,
                                            open: c_int,
                                            gap: c_int,
                                            matrix: *const ParasailMatrix)
                                            -> *mut ParasailResult;
    pub fn parasail_sw_striped_sse41_128_64(s1: *const u8,
                                            s1Len: c_int,
                                            s2: *const u8,
                                            s2Len: c_int,
                                            open: c_int,
                                            gap: c_int,
                                            matrix: *const ParasailMatrix)
                                            -> *mut ParasailResult;
    pub fn parasail_sw_striped_sse41_128_32(s1: *const u8,
                                            s1Len: c_int,
                                            s2: *const u8,
                                            s2Len: c_int,
                                            open: c_int,
                                            gap: c_int,
                                            matrix: *const ParasailMatrix)
                                            -> *mut ParasailResult;
    pub fn parasail_sw_striped_sse41_128_16(s1: *const u8,
                                            s1Len: c_int,
                                            s2: *const u8,
                                            s2Len: c_int,
                                            open: c_int,
                                            gap: c_int,
                                            matrix: *const ParasailMatrix)
                                            -> *mut ParasailResult;
    pub fn parasail_sw_striped_sse41_128_8(s1: *const u8,
                                           s1Len: c_int,
                                           s2: *const u8,
                                           s2Len: c_int,
                                           open: c_int,
                                           gap: c_int,
                                           matrix: *const ParasailMatrix)
                                           -> *mut ParasailResult;
    pub fn parasail_sw_striped_sse41_128_sat(s1: *const u8,
                                             s1Len: c_int,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int,
                                             matrix: *const ParasailMatrix)
                                             -> *mut ParasailResult;
    pub fn parasail_sw_striped_avx2_256_64(s1: *const u8,
                                           s1Len: c_int,
                                           s2: *const u8,
                                           s2Len: c_int,
                                           open: c_int,
                                           gap: c_int,
                                           matrix: *const ParasailMatrix)
                                           -> *mut ParasailResult;
    pub fn parasail_sw_striped_avx2_256_32(s1: *const u8,
                                           s1Len: c_int,
                                           s2: *const u8,
                                           s2Len: c_int,
                                           open: c_int,
                                           gap: c_int,
                                           matrix: *const ParasailMatrix)
                                           -> *mut ParasailResult;
    pub fn parasail_sw_striped_avx2_256_16(s1: *const u8,
                                           s1Len: c_int,
                                           s2: *const u8,
                                           s2Len: c_int,
                                           open: c_int,
                                           gap: c_int,
                                           matrix: *const ParasailMatrix)
                                           -> *mut ParasailResult;
    pub fn parasail_sw_striped_avx2_256_8(s1: *const u8,
                                          s1Len: c_int,
                                          s2: *const u8,
                                          s2Len: c_int,
                                          open: c_int,
                                          gap: c_int,
                                          matrix: *const ParasailMatrix)
                                          -> *mut ParasailResult;
    pub fn parasail_sw_striped_avx2_256_sat(s1: *const u8,
                                            s1Len: c_int,
                                            s2: *const u8,
                                            s2Len: c_int,
                                            open: c_int,
                                            gap: c_int,
                                            matrix: *const ParasailMatrix)
                                            -> *mut ParasailResult;
    pub fn parasail_sw_striped_knc_512_32(s1: *const u8,
                                          s1Len: c_int,
                                          s2: *const u8,
                                          s2Len: c_int,
                                          open: c_int,
                                          gap: c_int,
                                          matrix: *const ParasailMatrix)
                                          -> *mut ParasailResult;
    pub fn parasail_sw_diag_sse2_128_64(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_sw_diag_sse2_128_32(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_sw_diag_sse2_128_16(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_sw_diag_sse2_128_8(s1: *const u8,
                                       s1Len: c_int,
                                       s2: *const u8,
                                       s2Len: c_int,
                                       open: c_int,
                                       gap: c_int,
                                       matrix: *const ParasailMatrix)
                                       -> *mut ParasailResult;
    pub fn parasail_sw_diag_sse2_128_sat(s1: *const u8,
                                         s1Len: c_int,
                                         s2: *const u8,
                                         s2Len: c_int,
                                         open: c_int,
                                         gap: c_int,
                                         matrix: *const ParasailMatrix)
                                         -> *mut ParasailResult;
    pub fn parasail_sw_diag_sse41_128_64(s1: *const u8,
                                         s1Len: c_int,
                                         s2: *const u8,
                                         s2Len: c_int,
                                         open: c_int,
                                         gap: c_int,
                                         matrix: *const ParasailMatrix)
                                         -> *mut ParasailResult;
    pub fn parasail_sw_diag_sse41_128_32(s1: *const u8,
                                         s1Len: c_int,
                                         s2: *const u8,
                                         s2Len: c_int,
                                         open: c_int,
                                         gap: c_int,
                                         matrix: *const ParasailMatrix)
                                         -> *mut ParasailResult;
    pub fn parasail_sw_diag_sse41_128_16(s1: *const u8,
                                         s1Len: c_int,
                                         s2: *const u8,
                                         s2Len: c_int,
                                         open: c_int,
                                         gap: c_int,
                                         matrix: *const ParasailMatrix)
                                         -> *mut ParasailResult;
    pub fn parasail_sw_diag_sse41_128_8(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_sw_diag_sse41_128_sat(s1: *const u8,
                                          s1Len: c_int,
                                          s2: *const u8,
                                          s2Len: c_int,
                                          open: c_int,
                                          gap: c_int,
                                          matrix: *const ParasailMatrix)
                                          -> *mut ParasailResult;
    pub fn parasail_sw_diag_avx2_256_64(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_sw_diag_avx2_256_32(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_sw_diag_avx2_256_16(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_sw_diag_avx2_256_8(s1: *const u8,
                                       s1Len: c_int,
                                       s2: *const u8,
                                       s2Len: c_int,
                                       open: c_int,
                                       gap: c_int,
                                       matrix: *const ParasailMatrix)
                                       -> *mut ParasailResult;
    pub fn parasail_sw_diag_avx2_256_sat(s1: *const u8,
                                         s1Len: c_int,
                                         s2: *const u8,
                                         s2Len: c_int,
                                         open: c_int,
                                         gap: c_int,
                                         matrix: *const ParasailMatrix)
                                         -> *mut ParasailResult;
    pub fn parasail_sw_diag_knc_512_32(s1: *const u8,
                                       s1Len: c_int,
                                       s2: *const u8,
                                       s2Len: c_int,
                                       open: c_int,
                                       gap: c_int,
                                       matrix: *const ParasailMatrix)
                                       -> *mut ParasailResult;
    pub fn parasail_sw_table_scan_sse2_128_64(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sw_table_scan_sse2_128_32(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sw_table_scan_sse2_128_16(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sw_table_scan_sse2_128_8(s1: *const u8,
                                             s1Len: c_int,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int,
                                             matrix: *const ParasailMatrix)
                                             -> *mut ParasailResult;
    pub fn parasail_sw_table_scan_sse2_128_sat(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sw_table_scan_sse41_128_64(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sw_table_scan_sse41_128_32(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sw_table_scan_sse41_128_16(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sw_table_scan_sse41_128_8(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sw_table_scan_sse41_128_sat(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_sw_table_scan_avx2_256_64(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sw_table_scan_avx2_256_32(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sw_table_scan_avx2_256_16(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sw_table_scan_avx2_256_8(s1: *const u8,
                                             s1Len: c_int,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int,
                                             matrix: *const ParasailMatrix)
                                             -> *mut ParasailResult;
    pub fn parasail_sw_table_scan_avx2_256_sat(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sw_table_scan_knc_512_32(s1: *const u8,
                                             s1Len: c_int,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int,
                                             matrix: *const ParasailMatrix)
                                             -> *mut ParasailResult;
    pub fn parasail_sw_table_striped_sse2_128_64(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
    pub fn parasail_sw_table_striped_sse2_128_32(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
    pub fn parasail_sw_table_striped_sse2_128_16(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
    pub fn parasail_sw_table_striped_sse2_128_8(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_sw_table_striped_sse2_128_sat(s1: *const u8,
                                                  s1Len: c_int,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int,
                                                  matrix: *const ParasailMatrix)
                                                  -> *mut ParasailResult;
    pub fn parasail_sw_table_striped_sse41_128_64(s1: *const u8,
                                                  s1Len: c_int,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int,
                                                  matrix: *const ParasailMatrix)
                                                  -> *mut ParasailResult;
    pub fn parasail_sw_table_striped_sse41_128_32(s1: *const u8,
                                                  s1Len: c_int,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int,
                                                  matrix: *const ParasailMatrix)
                                                  -> *mut ParasailResult;
    pub fn parasail_sw_table_striped_sse41_128_16(s1: *const u8,
                                                  s1Len: c_int,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int,
                                                  matrix: *const ParasailMatrix)
                                                  -> *mut ParasailResult;
    pub fn parasail_sw_table_striped_sse41_128_8(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
    pub fn parasail_sw_table_striped_sse41_128_sat(s1: *const u8,
                                                   s1Len: c_int,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int,
                                                   matrix: *const ParasailMatrix)
                                                   -> *mut ParasailResult;
    pub fn parasail_sw_table_striped_avx2_256_64(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
    pub fn parasail_sw_table_striped_avx2_256_32(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
    pub fn parasail_sw_table_striped_avx2_256_16(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
    pub fn parasail_sw_table_striped_avx2_256_8(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_sw_table_striped_avx2_256_sat(s1: *const u8,
                                                  s1Len: c_int,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int,
                                                  matrix: *const ParasailMatrix)
                                                  -> *mut ParasailResult;
    pub fn parasail_sw_table_striped_knc_512_32(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_sw_table_diag_sse2_128_64(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sw_table_diag_sse2_128_32(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sw_table_diag_sse2_128_16(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sw_table_diag_sse2_128_8(s1: *const u8,
                                             s1Len: c_int,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int,
                                             matrix: *const ParasailMatrix)
                                             -> *mut ParasailResult;
    pub fn parasail_sw_table_diag_sse2_128_sat(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sw_table_diag_sse41_128_64(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sw_table_diag_sse41_128_32(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sw_table_diag_sse41_128_16(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sw_table_diag_sse41_128_8(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sw_table_diag_sse41_128_sat(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_sw_table_diag_avx2_256_64(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sw_table_diag_avx2_256_32(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sw_table_diag_avx2_256_16(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sw_table_diag_avx2_256_8(s1: *const u8,
                                             s1Len: c_int,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int,
                                             matrix: *const ParasailMatrix)
                                             -> *mut ParasailResult;
    pub fn parasail_sw_table_diag_avx2_256_sat(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sw_table_diag_knc_512_32(s1: *const u8,
                                             s1Len: c_int,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int,
                                             matrix: *const ParasailMatrix)
                                             -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_scan_sse2_128_64(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_scan_sse2_128_32(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_scan_sse2_128_16(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_scan_sse2_128_8(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_scan_sse2_128_sat(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_scan_sse41_128_64(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_scan_sse41_128_32(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_scan_sse41_128_16(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_scan_sse41_128_8(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_scan_sse41_128_sat(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_scan_avx2_256_64(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_scan_avx2_256_32(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_scan_avx2_256_16(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_scan_avx2_256_8(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_scan_avx2_256_sat(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_scan_knc_512_32(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_striped_sse2_128_64(s1: *const u8,
                                                  s1Len: c_int,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int,
                                                  matrix: *const ParasailMatrix)
                                                  -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_striped_sse2_128_32(s1: *const u8,
                                                  s1Len: c_int,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int,
                                                  matrix: *const ParasailMatrix)
                                                  -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_striped_sse2_128_16(s1: *const u8,
                                                  s1Len: c_int,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int,
                                                  matrix: *const ParasailMatrix)
                                                  -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_striped_sse2_128_8(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_striped_sse2_128_sat(s1: *const u8,
                                                   s1Len: c_int,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int,
                                                   matrix: *const ParasailMatrix)
                                                   -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_striped_sse41_128_64(s1: *const u8,
                                                   s1Len: c_int,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int,
                                                   matrix: *const ParasailMatrix)
                                                   -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_striped_sse41_128_32(s1: *const u8,
                                                   s1Len: c_int,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int,
                                                   matrix: *const ParasailMatrix)
                                                   -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_striped_sse41_128_16(s1: *const u8,
                                                   s1Len: c_int,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int,
                                                   matrix: *const ParasailMatrix)
                                                   -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_striped_sse41_128_8(s1: *const u8,
                                                  s1Len: c_int,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int,
                                                  matrix: *const ParasailMatrix)
                                                  -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_striped_sse41_128_sat(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_striped_avx2_256_64(s1: *const u8,
                                                  s1Len: c_int,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int,
                                                  matrix: *const ParasailMatrix)
                                                  -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_striped_avx2_256_32(s1: *const u8,
                                                  s1Len: c_int,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int,
                                                  matrix: *const ParasailMatrix)
                                                  -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_striped_avx2_256_16(s1: *const u8,
                                                  s1Len: c_int,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int,
                                                  matrix: *const ParasailMatrix)
                                                  -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_striped_avx2_256_8(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_striped_avx2_256_sat(s1: *const u8,
                                                   s1Len: c_int,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int,
                                                   matrix: *const ParasailMatrix)
                                                   -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_striped_knc_512_32(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_diag_sse2_128_64(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_diag_sse2_128_32(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_diag_sse2_128_16(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_diag_sse2_128_8(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_diag_sse2_128_sat(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_diag_sse41_128_64(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_diag_sse41_128_32(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_diag_sse41_128_16(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_diag_sse41_128_8(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_diag_sse41_128_sat(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_diag_avx2_256_64(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_diag_avx2_256_32(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_diag_avx2_256_16(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_diag_avx2_256_8(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_diag_avx2_256_sat(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_diag_knc_512_32(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sw_stats_scan_sse2_128_64(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sw_stats_scan_sse2_128_32(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sw_stats_scan_sse2_128_16(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sw_stats_scan_sse2_128_8(s1: *const u8,
                                             s1Len: c_int,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int,
                                             matrix: *const ParasailMatrix)
                                             -> *mut ParasailResult;
    pub fn parasail_sw_stats_scan_sse2_128_sat(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sw_stats_scan_sse41_128_64(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sw_stats_scan_sse41_128_32(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sw_stats_scan_sse41_128_16(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sw_stats_scan_sse41_128_8(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sw_stats_scan_sse41_128_sat(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_sw_stats_scan_avx2_256_64(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sw_stats_scan_avx2_256_32(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sw_stats_scan_avx2_256_16(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sw_stats_scan_avx2_256_8(s1: *const u8,
                                             s1Len: c_int,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int,
                                             matrix: *const ParasailMatrix)
                                             -> *mut ParasailResult;
    pub fn parasail_sw_stats_scan_avx2_256_sat(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sw_stats_scan_knc_512_32(s1: *const u8,
                                             s1Len: c_int,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int,
                                             matrix: *const ParasailMatrix)
                                             -> *mut ParasailResult;
    pub fn parasail_sw_stats_striped_sse2_128_64(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
    pub fn parasail_sw_stats_striped_sse2_128_32(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
    pub fn parasail_sw_stats_striped_sse2_128_16(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
    pub fn parasail_sw_stats_striped_sse2_128_8(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_sw_stats_striped_sse2_128_sat(s1: *const u8,
                                                  s1Len: c_int,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int,
                                                  matrix: *const ParasailMatrix)
                                                  -> *mut ParasailResult;
    pub fn parasail_sw_stats_striped_sse41_128_64(s1: *const u8,
                                                  s1Len: c_int,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int,
                                                  matrix: *const ParasailMatrix)
                                                  -> *mut ParasailResult;
    pub fn parasail_sw_stats_striped_sse41_128_32(s1: *const u8,
                                                  s1Len: c_int,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int,
                                                  matrix: *const ParasailMatrix)
                                                  -> *mut ParasailResult;
    pub fn parasail_sw_stats_striped_sse41_128_16(s1: *const u8,
                                                  s1Len: c_int,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int,
                                                  matrix: *const ParasailMatrix)
                                                  -> *mut ParasailResult;
    pub fn parasail_sw_stats_striped_sse41_128_8(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
    pub fn parasail_sw_stats_striped_sse41_128_sat(s1: *const u8,
                                                   s1Len: c_int,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int,
                                                   matrix: *const ParasailMatrix)
                                                   -> *mut ParasailResult;
    pub fn parasail_sw_stats_striped_avx2_256_64(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
    pub fn parasail_sw_stats_striped_avx2_256_32(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
    pub fn parasail_sw_stats_striped_avx2_256_16(s1: *const u8,
                                                 s1Len: c_int,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int,
                                                 matrix: *const ParasailMatrix)
                                                 -> *mut ParasailResult;
    pub fn parasail_sw_stats_striped_avx2_256_8(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_sw_stats_striped_avx2_256_sat(s1: *const u8,
                                                  s1Len: c_int,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int,
                                                  matrix: *const ParasailMatrix)
                                                  -> *mut ParasailResult;
    pub fn parasail_sw_stats_striped_knc_512_32(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_sw_stats_diag_sse2_128_64(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sw_stats_diag_sse2_128_32(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sw_stats_diag_sse2_128_16(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sw_stats_diag_sse2_128_8(s1: *const u8,
                                             s1Len: c_int,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int,
                                             matrix: *const ParasailMatrix)
                                             -> *mut ParasailResult;
    pub fn parasail_sw_stats_diag_sse2_128_sat(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sw_stats_diag_sse41_128_64(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sw_stats_diag_sse41_128_32(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sw_stats_diag_sse41_128_16(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sw_stats_diag_sse41_128_8(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sw_stats_diag_sse41_128_sat(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_sw_stats_diag_avx2_256_64(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sw_stats_diag_avx2_256_32(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sw_stats_diag_avx2_256_16(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sw_stats_diag_avx2_256_8(s1: *const u8,
                                             s1Len: c_int,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int,
                                             matrix: *const ParasailMatrix)
                                             -> *mut ParasailResult;
    pub fn parasail_sw_stats_diag_avx2_256_sat(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sw_stats_diag_knc_512_32(s1: *const u8,
                                             s1Len: c_int,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int,
                                             matrix: *const ParasailMatrix)
                                             -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_scan_sse2_128_64(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_scan_sse2_128_32(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_scan_sse2_128_16(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_scan_sse2_128_8(s1: *const u8,
                                                   s1Len: c_int,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int,
                                                   matrix: *const ParasailMatrix)
                                                   -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_scan_sse2_128_sat(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_scan_sse41_128_64(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_scan_sse41_128_32(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_scan_sse41_128_16(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_scan_sse41_128_8(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_scan_sse41_128_sat(s1: *const u8,
                                                      s1Len: c_int,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int,
                                                      matrix: *const ParasailMatrix)
                                                      -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_scan_avx2_256_64(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_scan_avx2_256_32(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_scan_avx2_256_16(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_scan_avx2_256_8(s1: *const u8,
                                                   s1Len: c_int,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int,
                                                   matrix: *const ParasailMatrix)
                                                   -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_scan_avx2_256_sat(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_scan_knc_512_32(s1: *const u8,
                                                   s1Len: c_int,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int,
                                                   matrix: *const ParasailMatrix)
                                                   -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_striped_sse2_128_64(s1: *const u8,
                                                       s1Len: c_int,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int,
                                                       matrix: *const ParasailMatrix)
                                                       -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_striped_sse2_128_32(s1: *const u8,
                                                       s1Len: c_int,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int,
                                                       matrix: *const ParasailMatrix)
                                                       -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_striped_sse2_128_16(s1: *const u8,
                                                       s1Len: c_int,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int,
                                                       matrix: *const ParasailMatrix)
                                                       -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_striped_sse2_128_8(s1: *const u8,
                                                      s1Len: c_int,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int,
                                                      matrix: *const ParasailMatrix)
                                                      -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_striped_sse2_128_sat(s1: *const u8,
                                                        s1Len: c_int,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int,
                                                        matrix: *const ParasailMatrix)
                                                        -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_striped_sse41_128_64(s1: *const u8,
                                                        s1Len: c_int,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int,
                                                        matrix: *const ParasailMatrix)
                                                        -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_striped_sse41_128_32(s1: *const u8,
                                                        s1Len: c_int,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int,
                                                        matrix: *const ParasailMatrix)
                                                        -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_striped_sse41_128_16(s1: *const u8,
                                                        s1Len: c_int,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int,
                                                        matrix: *const ParasailMatrix)
                                                        -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_striped_sse41_128_8(s1: *const u8,
                                                       s1Len: c_int,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int,
                                                       matrix: *const ParasailMatrix)
                                                       -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_striped_sse41_128_sat(s1: *const u8,
                                                         s1Len: c_int,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int,
                                                         matrix: *const ParasailMatrix)
                                                         -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_striped_avx2_256_64(s1: *const u8,
                                                       s1Len: c_int,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int,
                                                       matrix: *const ParasailMatrix)
                                                       -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_striped_avx2_256_32(s1: *const u8,
                                                       s1Len: c_int,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int,
                                                       matrix: *const ParasailMatrix)
                                                       -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_striped_avx2_256_16(s1: *const u8,
                                                       s1Len: c_int,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int,
                                                       matrix: *const ParasailMatrix)
                                                       -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_striped_avx2_256_8(s1: *const u8,
                                                      s1Len: c_int,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int,
                                                      matrix: *const ParasailMatrix)
                                                      -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_striped_avx2_256_sat(s1: *const u8,
                                                        s1Len: c_int,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int,
                                                        matrix: *const ParasailMatrix)
                                                        -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_striped_knc_512_32(s1: *const u8,
                                                      s1Len: c_int,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int,
                                                      matrix: *const ParasailMatrix)
                                                      -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_diag_sse2_128_64(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_diag_sse2_128_32(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_diag_sse2_128_16(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_diag_sse2_128_8(s1: *const u8,
                                                   s1Len: c_int,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int,
                                                   matrix: *const ParasailMatrix)
                                                   -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_diag_sse2_128_sat(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_diag_sse41_128_64(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_diag_sse41_128_32(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_diag_sse41_128_16(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_diag_sse41_128_8(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_diag_sse41_128_sat(s1: *const u8,
                                                      s1Len: c_int,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int,
                                                      matrix: *const ParasailMatrix)
                                                      -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_diag_avx2_256_64(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_diag_avx2_256_32(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_diag_avx2_256_16(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_diag_avx2_256_8(s1: *const u8,
                                                   s1Len: c_int,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int,
                                                   matrix: *const ParasailMatrix)
                                                   -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_diag_avx2_256_sat(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_diag_knc_512_32(s1: *const u8,
                                                   s1Len: c_int,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int,
                                                   matrix: *const ParasailMatrix)
                                                   -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_scan_sse2_128_64(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_scan_sse2_128_32(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_scan_sse2_128_16(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_scan_sse2_128_8(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_scan_sse2_128_sat(s1: *const u8,
                                                      s1Len: c_int,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int,
                                                      matrix: *const ParasailMatrix)
                                                      -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_scan_sse41_128_64(s1: *const u8,
                                                      s1Len: c_int,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int,
                                                      matrix: *const ParasailMatrix)
                                                      -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_scan_sse41_128_32(s1: *const u8,
                                                      s1Len: c_int,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int,
                                                      matrix: *const ParasailMatrix)
                                                      -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_scan_sse41_128_16(s1: *const u8,
                                                      s1Len: c_int,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int,
                                                      matrix: *const ParasailMatrix)
                                                      -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_scan_sse41_128_8(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_scan_sse41_128_sat(s1: *const u8,
                                                       s1Len: c_int,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int,
                                                       matrix: *const ParasailMatrix)
                                                       -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_scan_avx2_256_64(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_scan_avx2_256_32(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_scan_avx2_256_16(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_scan_avx2_256_8(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_scan_avx2_256_sat(s1: *const u8,
                                                      s1Len: c_int,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int,
                                                      matrix: *const ParasailMatrix)
                                                      -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_scan_knc_512_32(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_striped_sse2_128_64(s1: *const u8,
                                                        s1Len: c_int,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int,
                                                        matrix: *const ParasailMatrix)
                                                        -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_striped_sse2_128_32(s1: *const u8,
                                                        s1Len: c_int,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int,
                                                        matrix: *const ParasailMatrix)
                                                        -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_striped_sse2_128_16(s1: *const u8,
                                                        s1Len: c_int,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int,
                                                        matrix: *const ParasailMatrix)
                                                        -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_striped_sse2_128_8(s1: *const u8,
                                                       s1Len: c_int,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int,
                                                       matrix: *const ParasailMatrix)
                                                       -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_striped_sse2_128_sat(s1: *const u8,
                                                         s1Len: c_int,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int,
                                                         matrix: *const ParasailMatrix)
                                                         -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_striped_sse41_128_64(s1: *const u8,
                                                         s1Len: c_int,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int,
                                                         matrix: *const ParasailMatrix)
                                                         -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_striped_sse41_128_32(s1: *const u8,
                                                         s1Len: c_int,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int,
                                                         matrix: *const ParasailMatrix)
                                                         -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_striped_sse41_128_16(s1: *const u8,
                                                         s1Len: c_int,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int,
                                                         matrix: *const ParasailMatrix)
                                                         -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_striped_sse41_128_8(s1: *const u8,
                                                        s1Len: c_int,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int,
                                                        matrix: *const ParasailMatrix)
                                                        -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_striped_sse41_128_sat(s1: *const u8,
                                                          s1Len: c_int,
                                                          s2: *const u8,
                                                          s2Len: c_int,
                                                          open: c_int,
                                                          gap: c_int,
                                                          matrix: *const ParasailMatrix)
                                                          -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_striped_avx2_256_64(s1: *const u8,
                                                        s1Len: c_int,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int,
                                                        matrix: *const ParasailMatrix)
                                                        -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_striped_avx2_256_32(s1: *const u8,
                                                        s1Len: c_int,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int,
                                                        matrix: *const ParasailMatrix)
                                                        -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_striped_avx2_256_16(s1: *const u8,
                                                        s1Len: c_int,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int,
                                                        matrix: *const ParasailMatrix)
                                                        -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_striped_avx2_256_8(s1: *const u8,
                                                       s1Len: c_int,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int,
                                                       matrix: *const ParasailMatrix)
                                                       -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_striped_avx2_256_sat(s1: *const u8,
                                                         s1Len: c_int,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int,
                                                         matrix: *const ParasailMatrix)
                                                         -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_striped_knc_512_32(s1: *const u8,
                                                       s1Len: c_int,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int,
                                                       matrix: *const ParasailMatrix)
                                                       -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_diag_sse2_128_64(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_diag_sse2_128_32(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_diag_sse2_128_16(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_diag_sse2_128_8(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_diag_sse2_128_sat(s1: *const u8,
                                                      s1Len: c_int,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int,
                                                      matrix: *const ParasailMatrix)
                                                      -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_diag_sse41_128_64(s1: *const u8,
                                                      s1Len: c_int,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int,
                                                      matrix: *const ParasailMatrix)
                                                      -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_diag_sse41_128_32(s1: *const u8,
                                                      s1Len: c_int,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int,
                                                      matrix: *const ParasailMatrix)
                                                      -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_diag_sse41_128_16(s1: *const u8,
                                                      s1Len: c_int,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int,
                                                      matrix: *const ParasailMatrix)
                                                      -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_diag_sse41_128_8(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_diag_sse41_128_sat(s1: *const u8,
                                                       s1Len: c_int,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int,
                                                       matrix: *const ParasailMatrix)
                                                       -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_diag_avx2_256_64(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_diag_avx2_256_32(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_diag_avx2_256_16(s1: *const u8,
                                                     s1Len: c_int,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_diag_avx2_256_8(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_diag_avx2_256_sat(s1: *const u8,
                                                      s1Len: c_int,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int,
                                                      matrix: *const ParasailMatrix)
                                                      -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_diag_knc_512_32(s1: *const u8,
                                                    s1Len: c_int,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailResult;
    pub fn parasail_nw_scan_profile_sse2_128_64(profile: *const ParasailProfile,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int)
                                                -> *mut ParasailResult;
    pub fn parasail_nw_scan_profile_sse2_128_32(profile: *const ParasailProfile,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int)
                                                -> *mut ParasailResult;
    pub fn parasail_nw_scan_profile_sse2_128_16(profile: *const ParasailProfile,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int)
                                                -> *mut ParasailResult;
    pub fn parasail_nw_scan_profile_sse2_128_8(profile: *const ParasailProfile,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int)
                                               -> *mut ParasailResult;
    pub fn parasail_nw_scan_profile_sse2_128_sat(profile: *const ParasailProfile,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int)
                                                 -> *mut ParasailResult;
    pub fn parasail_nw_scan_profile_sse41_128_64(profile: *const ParasailProfile,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int)
                                                 -> *mut ParasailResult;
    pub fn parasail_nw_scan_profile_sse41_128_32(profile: *const ParasailProfile,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int)
                                                 -> *mut ParasailResult;
    pub fn parasail_nw_scan_profile_sse41_128_16(profile: *const ParasailProfile,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int)
                                                 -> *mut ParasailResult;
    pub fn parasail_nw_scan_profile_sse41_128_8(profile: *const ParasailProfile,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int)
                                                -> *mut ParasailResult;
    pub fn parasail_nw_scan_profile_sse41_128_sat(profile: *const ParasailProfile,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int)
                                                  -> *mut ParasailResult;
    pub fn parasail_nw_scan_profile_avx2_256_64(profile: *const ParasailProfile,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int)
                                                -> *mut ParasailResult;
    pub fn parasail_nw_scan_profile_avx2_256_32(profile: *const ParasailProfile,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int)
                                                -> *mut ParasailResult;
    pub fn parasail_nw_scan_profile_avx2_256_16(profile: *const ParasailProfile,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int)
                                                -> *mut ParasailResult;
    pub fn parasail_nw_scan_profile_avx2_256_8(profile: *const ParasailProfile,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int)
                                               -> *mut ParasailResult;
    pub fn parasail_nw_scan_profile_avx2_256_sat(profile: *const ParasailProfile,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int)
                                                 -> *mut ParasailResult;
    pub fn parasail_nw_scan_profile_knc_512_32(profile: *const ParasailProfile,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int)
                                               -> *mut ParasailResult;
    pub fn parasail_nw_striped_profile_sse2_128_64(profile: *const ParasailProfile,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int)
                                                   -> *mut ParasailResult;
    pub fn parasail_nw_striped_profile_sse2_128_32(profile: *const ParasailProfile,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int)
                                                   -> *mut ParasailResult;
    pub fn parasail_nw_striped_profile_sse2_128_16(profile: *const ParasailProfile,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int)
                                                   -> *mut ParasailResult;
    pub fn parasail_nw_striped_profile_sse2_128_8(profile: *const ParasailProfile,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int)
                                                  -> *mut ParasailResult;
    pub fn parasail_nw_striped_profile_sse2_128_sat(profile: *const ParasailProfile,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int)
                                                    -> *mut ParasailResult;
    pub fn parasail_nw_striped_profile_sse41_128_64(profile: *const ParasailProfile,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int)
                                                    -> *mut ParasailResult;
    pub fn parasail_nw_striped_profile_sse41_128_32(profile: *const ParasailProfile,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int)
                                                    -> *mut ParasailResult;
    pub fn parasail_nw_striped_profile_sse41_128_16(profile: *const ParasailProfile,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int)
                                                    -> *mut ParasailResult;
    pub fn parasail_nw_striped_profile_sse41_128_8(profile: *const ParasailProfile,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int)
                                                   -> *mut ParasailResult;
    pub fn parasail_nw_striped_profile_sse41_128_sat(profile: *const ParasailProfile,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int)
                                                     -> *mut ParasailResult;
    pub fn parasail_nw_striped_profile_avx2_256_64(profile: *const ParasailProfile,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int)
                                                   -> *mut ParasailResult;
    pub fn parasail_nw_striped_profile_avx2_256_32(profile: *const ParasailProfile,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int)
                                                   -> *mut ParasailResult;
    pub fn parasail_nw_striped_profile_avx2_256_16(profile: *const ParasailProfile,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int)
                                                   -> *mut ParasailResult;
    pub fn parasail_nw_striped_profile_avx2_256_8(profile: *const ParasailProfile,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int)
                                                  -> *mut ParasailResult;
    pub fn parasail_nw_striped_profile_avx2_256_sat(profile: *const ParasailProfile,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int)
                                                    -> *mut ParasailResult;
    pub fn parasail_nw_striped_profile_knc_512_32(profile: *const ParasailProfile,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int)
                                                  -> *mut ParasailResult;
    pub fn parasail_nw_table_scan_profile_sse2_128_64(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_nw_table_scan_profile_sse2_128_32(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_nw_table_scan_profile_sse2_128_16(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_nw_table_scan_profile_sse2_128_8(profile: *const ParasailProfile,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int)
                                                     -> *mut ParasailResult;
    pub fn parasail_nw_table_scan_profile_sse2_128_sat(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_nw_table_scan_profile_sse41_128_64(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_nw_table_scan_profile_sse41_128_32(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_nw_table_scan_profile_sse41_128_16(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_nw_table_scan_profile_sse41_128_8(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_nw_table_scan_profile_sse41_128_sat(profile: *const ParasailProfile,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int)
                                                        -> *mut ParasailResult;
    pub fn parasail_nw_table_scan_profile_avx2_256_64(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_nw_table_scan_profile_avx2_256_32(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_nw_table_scan_profile_avx2_256_16(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_nw_table_scan_profile_avx2_256_8(profile: *const ParasailProfile,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int)
                                                     -> *mut ParasailResult;
    pub fn parasail_nw_table_scan_profile_avx2_256_sat(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_nw_table_scan_profile_knc_512_32(profile: *const ParasailProfile,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int)
                                                     -> *mut ParasailResult;
    pub fn parasail_nw_table_striped_profile_sse2_128_64(profile: *const ParasailProfile,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int)
                                                         -> *mut ParasailResult;
    pub fn parasail_nw_table_striped_profile_sse2_128_32(profile: *const ParasailProfile,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int)
                                                         -> *mut ParasailResult;
    pub fn parasail_nw_table_striped_profile_sse2_128_16(profile: *const ParasailProfile,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int)
                                                         -> *mut ParasailResult;
    pub fn parasail_nw_table_striped_profile_sse2_128_8(profile: *const ParasailProfile,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int)
                                                        -> *mut ParasailResult;
    pub fn parasail_nw_table_striped_profile_sse2_128_sat(profile: *const ParasailProfile,
                                                          s2: *const u8,
                                                          s2Len: c_int,
                                                          open: c_int,
                                                          gap: c_int)
                                                          -> *mut ParasailResult;
    pub fn parasail_nw_table_striped_profile_sse41_128_64(profile: *const ParasailProfile,
                                                          s2: *const u8,
                                                          s2Len: c_int,
                                                          open: c_int,
                                                          gap: c_int)
                                                          -> *mut ParasailResult;
    pub fn parasail_nw_table_striped_profile_sse41_128_32(profile: *const ParasailProfile,
                                                          s2: *const u8,
                                                          s2Len: c_int,
                                                          open: c_int,
                                                          gap: c_int)
                                                          -> *mut ParasailResult;
    pub fn parasail_nw_table_striped_profile_sse41_128_16(profile: *const ParasailProfile,
                                                          s2: *const u8,
                                                          s2Len: c_int,
                                                          open: c_int,
                                                          gap: c_int)
                                                          -> *mut ParasailResult;
    pub fn parasail_nw_table_striped_profile_sse41_128_8(profile: *const ParasailProfile,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int)
                                                         -> *mut ParasailResult;
    pub fn parasail_nw_table_striped_profile_sse41_128_sat(profile: *const ParasailProfile,
                                                           s2: *const u8,
                                                           s2Len: c_int,
                                                           open: c_int,
                                                           gap: c_int)
                                                           -> *mut ParasailResult;
    pub fn parasail_nw_table_striped_profile_avx2_256_64(profile: *const ParasailProfile,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int)
                                                         -> *mut ParasailResult;
    pub fn parasail_nw_table_striped_profile_avx2_256_32(profile: *const ParasailProfile,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int)
                                                         -> *mut ParasailResult;
    pub fn parasail_nw_table_striped_profile_avx2_256_16(profile: *const ParasailProfile,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int)
                                                         -> *mut ParasailResult;
    pub fn parasail_nw_table_striped_profile_avx2_256_8(profile: *const ParasailProfile,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int)
                                                        -> *mut ParasailResult;
    pub fn parasail_nw_table_striped_profile_avx2_256_sat(profile: *const ParasailProfile,
                                                          s2: *const u8,
                                                          s2Len: c_int,
                                                          open: c_int,
                                                          gap: c_int)
                                                          -> *mut ParasailResult;
    pub fn parasail_nw_table_striped_profile_knc_512_32(profile: *const ParasailProfile,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int)
                                                        -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_scan_profile_sse2_128_64(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_scan_profile_sse2_128_32(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_scan_profile_sse2_128_16(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_scan_profile_sse2_128_8(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_scan_profile_sse2_128_sat(profile: *const ParasailProfile,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int)
                                                        -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_scan_profile_sse41_128_64(profile: *const ParasailProfile,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int)
                                                        -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_scan_profile_sse41_128_32(profile: *const ParasailProfile,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int)
                                                        -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_scan_profile_sse41_128_16(profile: *const ParasailProfile,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int)
                                                        -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_scan_profile_sse41_128_8(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_scan_profile_sse41_128_sat(profile: *const ParasailProfile,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int)
                                                         -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_scan_profile_avx2_256_64(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_scan_profile_avx2_256_32(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_scan_profile_avx2_256_16(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_scan_profile_avx2_256_8(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_scan_profile_avx2_256_sat(profile: *const ParasailProfile,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int)
                                                        -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_scan_profile_knc_512_32(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_striped_profile_sse2_128_64(profile: *const ParasailProfile,
                                                          s2: *const u8,
                                                          s2Len: c_int,
                                                          open: c_int,
                                                          gap: c_int)
                                                          -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_striped_profile_sse2_128_32(profile: *const ParasailProfile,
                                                          s2: *const u8,
                                                          s2Len: c_int,
                                                          open: c_int,
                                                          gap: c_int)
                                                          -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_striped_profile_sse2_128_16(profile: *const ParasailProfile,
                                                          s2: *const u8,
                                                          s2Len: c_int,
                                                          open: c_int,
                                                          gap: c_int)
                                                          -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_striped_profile_sse2_128_8(profile: *const ParasailProfile,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int)
                                                         -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_striped_profile_sse2_128_sat(profile: *const ParasailProfile,
                                                           s2: *const u8,
                                                           s2Len: c_int,
                                                           open: c_int,
                                                           gap: c_int)
                                                           -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_striped_profile_sse41_128_64(profile: *const ParasailProfile,
                                                           s2: *const u8,
                                                           s2Len: c_int,
                                                           open: c_int,
                                                           gap: c_int)
                                                           -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_striped_profile_sse41_128_32(profile: *const ParasailProfile,
                                                           s2: *const u8,
                                                           s2Len: c_int,
                                                           open: c_int,
                                                           gap: c_int)
                                                           -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_striped_profile_sse41_128_16(profile: *const ParasailProfile,
                                                           s2: *const u8,
                                                           s2Len: c_int,
                                                           open: c_int,
                                                           gap: c_int)
                                                           -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_striped_profile_sse41_128_8(profile: *const ParasailProfile,
                                                          s2: *const u8,
                                                          s2Len: c_int,
                                                          open: c_int,
                                                          gap: c_int)
                                                          -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_striped_profile_sse41_128_sat(profile: *const ParasailProfile,
                                                            s2: *const u8,
                                                            s2Len: c_int,
                                                            open: c_int,
                                                            gap: c_int)
                                                            -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_striped_profile_avx2_256_64(profile: *const ParasailProfile,
                                                          s2: *const u8,
                                                          s2Len: c_int,
                                                          open: c_int,
                                                          gap: c_int)
                                                          -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_striped_profile_avx2_256_32(profile: *const ParasailProfile,
                                                          s2: *const u8,
                                                          s2Len: c_int,
                                                          open: c_int,
                                                          gap: c_int)
                                                          -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_striped_profile_avx2_256_16(profile: *const ParasailProfile,
                                                          s2: *const u8,
                                                          s2Len: c_int,
                                                          open: c_int,
                                                          gap: c_int)
                                                          -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_striped_profile_avx2_256_8(profile: *const ParasailProfile,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int)
                                                         -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_striped_profile_avx2_256_sat(profile: *const ParasailProfile,
                                                           s2: *const u8,
                                                           s2Len: c_int,
                                                           open: c_int,
                                                           gap: c_int)
                                                           -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_striped_profile_knc_512_32(profile: *const ParasailProfile,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int)
                                                         -> *mut ParasailResult;
    pub fn parasail_nw_stats_scan_profile_sse2_128_64(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_nw_stats_scan_profile_sse2_128_32(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_nw_stats_scan_profile_sse2_128_16(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_nw_stats_scan_profile_sse2_128_8(profile: *const ParasailProfile,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int)
                                                     -> *mut ParasailResult;
    pub fn parasail_nw_stats_scan_profile_sse2_128_sat(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_nw_stats_scan_profile_sse41_128_64(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_nw_stats_scan_profile_sse41_128_32(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_nw_stats_scan_profile_sse41_128_16(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_nw_stats_scan_profile_sse41_128_8(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_nw_stats_scan_profile_sse41_128_sat(profile: *const ParasailProfile,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int)
                                                        -> *mut ParasailResult;
    pub fn parasail_nw_stats_scan_profile_avx2_256_64(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_nw_stats_scan_profile_avx2_256_32(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_nw_stats_scan_profile_avx2_256_16(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_nw_stats_scan_profile_avx2_256_8(profile: *const ParasailProfile,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int)
                                                     -> *mut ParasailResult;
    pub fn parasail_nw_stats_scan_profile_avx2_256_sat(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_nw_stats_scan_profile_knc_512_32(profile: *const ParasailProfile,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int)
                                                     -> *mut ParasailResult;
    pub fn parasail_nw_stats_striped_profile_sse2_128_64(profile: *const ParasailProfile,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int)
                                                         -> *mut ParasailResult;
    pub fn parasail_nw_stats_striped_profile_sse2_128_32(profile: *const ParasailProfile,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int)
                                                         -> *mut ParasailResult;
    pub fn parasail_nw_stats_striped_profile_sse2_128_16(profile: *const ParasailProfile,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int)
                                                         -> *mut ParasailResult;
    pub fn parasail_nw_stats_striped_profile_sse2_128_8(profile: *const ParasailProfile,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int)
                                                        -> *mut ParasailResult;
    pub fn parasail_nw_stats_striped_profile_sse2_128_sat(profile: *const ParasailProfile,
                                                          s2: *const u8,
                                                          s2Len: c_int,
                                                          open: c_int,
                                                          gap: c_int)
                                                          -> *mut ParasailResult;
    pub fn parasail_nw_stats_striped_profile_sse41_128_64(profile: *const ParasailProfile,
                                                          s2: *const u8,
                                                          s2Len: c_int,
                                                          open: c_int,
                                                          gap: c_int)
                                                          -> *mut ParasailResult;
    pub fn parasail_nw_stats_striped_profile_sse41_128_32(profile: *const ParasailProfile,
                                                          s2: *const u8,
                                                          s2Len: c_int,
                                                          open: c_int,
                                                          gap: c_int)
                                                          -> *mut ParasailResult;
    pub fn parasail_nw_stats_striped_profile_sse41_128_16(profile: *const ParasailProfile,
                                                          s2: *const u8,
                                                          s2Len: c_int,
                                                          open: c_int,
                                                          gap: c_int)
                                                          -> *mut ParasailResult;
    pub fn parasail_nw_stats_striped_profile_sse41_128_8(profile: *const ParasailProfile,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int)
                                                         -> *mut ParasailResult;
    pub fn parasail_nw_stats_striped_profile_sse41_128_sat(profile: *const ParasailProfile,
                                                           s2: *const u8,
                                                           s2Len: c_int,
                                                           open: c_int,
                                                           gap: c_int)
                                                           -> *mut ParasailResult;
    pub fn parasail_nw_stats_striped_profile_avx2_256_64(profile: *const ParasailProfile,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int)
                                                         -> *mut ParasailResult;
    pub fn parasail_nw_stats_striped_profile_avx2_256_32(profile: *const ParasailProfile,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int)
                                                         -> *mut ParasailResult;
    pub fn parasail_nw_stats_striped_profile_avx2_256_16(profile: *const ParasailProfile,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int)
                                                         -> *mut ParasailResult;
    pub fn parasail_nw_stats_striped_profile_avx2_256_8(profile: *const ParasailProfile,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int)
                                                        -> *mut ParasailResult;
    pub fn parasail_nw_stats_striped_profile_avx2_256_sat(profile: *const ParasailProfile,
                                                          s2: *const u8,
                                                          s2Len: c_int,
                                                          open: c_int,
                                                          gap: c_int)
                                                          -> *mut ParasailResult;
    pub fn parasail_nw_stats_striped_profile_knc_512_32(profile: *const ParasailProfile,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int)
                                                        -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_scan_profile_sse2_128_64(profile: *const ParasailProfile,
                                                            s2: *const u8,
                                                            s2Len: c_int,
                                                            open: c_int,
                                                            gap: c_int)
                                                            -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_scan_profile_sse2_128_32(profile: *const ParasailProfile,
                                                            s2: *const u8,
                                                            s2Len: c_int,
                                                            open: c_int,
                                                            gap: c_int)
                                                            -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_scan_profile_sse2_128_16(profile: *const ParasailProfile,
                                                            s2: *const u8,
                                                            s2Len: c_int,
                                                            open: c_int,
                                                            gap: c_int)
                                                            -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_scan_profile_sse2_128_8(profile: *const ParasailProfile,
                                                           s2: *const u8,
                                                           s2Len: c_int,
                                                           open: c_int,
                                                           gap: c_int)
                                                           -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_scan_profile_sse2_128_sat(profile: *const ParasailProfile,
                                                             s2: *const u8,
                                                             s2Len: c_int,
                                                             open: c_int,
                                                             gap: c_int)
                                                             -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_scan_profile_sse41_128_64(profile: *const ParasailProfile,
                                                             s2: *const u8,
                                                             s2Len: c_int,
                                                             open: c_int,
                                                             gap: c_int)
                                                             -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_scan_profile_sse41_128_32(profile: *const ParasailProfile,
                                                             s2: *const u8,
                                                             s2Len: c_int,
                                                             open: c_int,
                                                             gap: c_int)
                                                             -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_scan_profile_sse41_128_16(profile: *const ParasailProfile,
                                                             s2: *const u8,
                                                             s2Len: c_int,
                                                             open: c_int,
                                                             gap: c_int)
                                                             -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_scan_profile_sse41_128_8(profile: *const ParasailProfile,
                                                            s2: *const u8,
                                                            s2Len: c_int,
                                                            open: c_int,
                                                            gap: c_int)
                                                            -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_scan_profile_sse41_128_sat(profile: *const ParasailProfile,
                                                              s2: *const u8,
                                                              s2Len: c_int,
                                                              open: c_int,
                                                              gap: c_int)
                                                              -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_scan_profile_avx2_256_64(profile: *const ParasailProfile,
                                                            s2: *const u8,
                                                            s2Len: c_int,
                                                            open: c_int,
                                                            gap: c_int)
                                                            -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_scan_profile_avx2_256_32(profile: *const ParasailProfile,
                                                            s2: *const u8,
                                                            s2Len: c_int,
                                                            open: c_int,
                                                            gap: c_int)
                                                            -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_scan_profile_avx2_256_16(profile: *const ParasailProfile,
                                                            s2: *const u8,
                                                            s2Len: c_int,
                                                            open: c_int,
                                                            gap: c_int)
                                                            -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_scan_profile_avx2_256_8(profile: *const ParasailProfile,
                                                           s2: *const u8,
                                                           s2Len: c_int,
                                                           open: c_int,
                                                           gap: c_int)
                                                           -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_scan_profile_avx2_256_sat(profile: *const ParasailProfile,
                                                             s2: *const u8,
                                                             s2Len: c_int,
                                                             open: c_int,
                                                             gap: c_int)
                                                             -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_scan_profile_knc_512_32(profile: *const ParasailProfile,
                                                           s2: *const u8,
                                                           s2Len: c_int,
                                                           open: c_int,
                                                           gap: c_int)
                                                           -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_striped_profile_sse2_128_64(profile: *const ParasailProfile,
                                                               s2: *const u8,
                                                               s2Len: c_int,
                                                               open: c_int,
                                                               gap: c_int)
                                                               -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_striped_profile_sse2_128_32(profile: *const ParasailProfile,
                                                               s2: *const u8,
                                                               s2Len: c_int,
                                                               open: c_int,
                                                               gap: c_int)
                                                               -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_striped_profile_sse2_128_16(profile: *const ParasailProfile,
                                                               s2: *const u8,
                                                               s2Len: c_int,
                                                               open: c_int,
                                                               gap: c_int)
                                                               -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_striped_profile_sse2_128_8(profile: *const ParasailProfile,
                                                              s2: *const u8,
                                                              s2Len: c_int,
                                                              open: c_int,
                                                              gap: c_int)
                                                              -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_striped_profile_sse2_128_sat(profile: *const ParasailProfile,
                                                                s2: *const u8,
                                                                s2Len: c_int,
                                                                open: c_int,
                                                                gap: c_int)
                                                                -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_striped_profile_sse41_128_64(profile: *const ParasailProfile,
                                                                s2: *const u8,
                                                                s2Len: c_int,
                                                                open: c_int,
                                                                gap: c_int)
                                                                -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_striped_profile_sse41_128_32(profile: *const ParasailProfile,
                                                                s2: *const u8,
                                                                s2Len: c_int,
                                                                open: c_int,
                                                                gap: c_int)
                                                                -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_striped_profile_sse41_128_16(profile: *const ParasailProfile,
                                                                s2: *const u8,
                                                                s2Len: c_int,
                                                                open: c_int,
                                                                gap: c_int)
                                                                -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_striped_profile_sse41_128_8(profile: *const ParasailProfile,
                                                               s2: *const u8,
                                                               s2Len: c_int,
                                                               open: c_int,
                                                               gap: c_int)
                                                               -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_striped_profile_sse41_128_sat(profile: *const ParasailProfile,
                                                                 s2: *const u8,
                                                                 s2Len: c_int,
                                                                 open: c_int,
                                                                 gap: c_int)
                                                                 -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_striped_profile_avx2_256_64(profile: *const ParasailProfile,
                                                               s2: *const u8,
                                                               s2Len: c_int,
                                                               open: c_int,
                                                               gap: c_int)
                                                               -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_striped_profile_avx2_256_32(profile: *const ParasailProfile,
                                                               s2: *const u8,
                                                               s2Len: c_int,
                                                               open: c_int,
                                                               gap: c_int)
                                                               -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_striped_profile_avx2_256_16(profile: *const ParasailProfile,
                                                               s2: *const u8,
                                                               s2Len: c_int,
                                                               open: c_int,
                                                               gap: c_int)
                                                               -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_striped_profile_avx2_256_8(profile: *const ParasailProfile,
                                                              s2: *const u8,
                                                              s2Len: c_int,
                                                              open: c_int,
                                                              gap: c_int)
                                                              -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_striped_profile_avx2_256_sat(profile: *const ParasailProfile,
                                                                s2: *const u8,
                                                                s2Len: c_int,
                                                                open: c_int,
                                                                gap: c_int)
                                                                -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_striped_profile_knc_512_32(profile: *const ParasailProfile,
                                                              s2: *const u8,
                                                              s2Len: c_int,
                                                              open: c_int,
                                                              gap: c_int)
                                                              -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_scan_profile_sse2_128_64(profile: *const ParasailProfile,
                                                             s2: *const u8,
                                                             s2Len: c_int,
                                                             open: c_int,
                                                             gap: c_int)
                                                             -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_scan_profile_sse2_128_32(profile: *const ParasailProfile,
                                                             s2: *const u8,
                                                             s2Len: c_int,
                                                             open: c_int,
                                                             gap: c_int)
                                                             -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_scan_profile_sse2_128_16(profile: *const ParasailProfile,
                                                             s2: *const u8,
                                                             s2Len: c_int,
                                                             open: c_int,
                                                             gap: c_int)
                                                             -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_scan_profile_sse2_128_8(profile: *const ParasailProfile,
                                                            s2: *const u8,
                                                            s2Len: c_int,
                                                            open: c_int,
                                                            gap: c_int)
                                                            -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_scan_profile_sse2_128_sat(profile: *const ParasailProfile,
                                                              s2: *const u8,
                                                              s2Len: c_int,
                                                              open: c_int,
                                                              gap: c_int)
                                                              -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_scan_profile_sse41_128_64(profile: *const ParasailProfile,
                                                              s2: *const u8,
                                                              s2Len: c_int,
                                                              open: c_int,
                                                              gap: c_int)
                                                              -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_scan_profile_sse41_128_32(profile: *const ParasailProfile,
                                                              s2: *const u8,
                                                              s2Len: c_int,
                                                              open: c_int,
                                                              gap: c_int)
                                                              -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_scan_profile_sse41_128_16(profile: *const ParasailProfile,
                                                              s2: *const u8,
                                                              s2Len: c_int,
                                                              open: c_int,
                                                              gap: c_int)
                                                              -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_scan_profile_sse41_128_8(profile: *const ParasailProfile,
                                                             s2: *const u8,
                                                             s2Len: c_int,
                                                             open: c_int,
                                                             gap: c_int)
                                                             -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_scan_profile_sse41_128_sat(profile: *const ParasailProfile,
                                                               s2: *const u8,
                                                               s2Len: c_int,
                                                               open: c_int,
                                                               gap: c_int)
                                                               -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_scan_profile_avx2_256_64(profile: *const ParasailProfile,
                                                             s2: *const u8,
                                                             s2Len: c_int,
                                                             open: c_int,
                                                             gap: c_int)
                                                             -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_scan_profile_avx2_256_32(profile: *const ParasailProfile,
                                                             s2: *const u8,
                                                             s2Len: c_int,
                                                             open: c_int,
                                                             gap: c_int)
                                                             -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_scan_profile_avx2_256_16(profile: *const ParasailProfile,
                                                             s2: *const u8,
                                                             s2Len: c_int,
                                                             open: c_int,
                                                             gap: c_int)
                                                             -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_scan_profile_avx2_256_8(profile: *const ParasailProfile,
                                                            s2: *const u8,
                                                            s2Len: c_int,
                                                            open: c_int,
                                                            gap: c_int)
                                                            -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_scan_profile_avx2_256_sat(profile: *const ParasailProfile,
                                                              s2: *const u8,
                                                              s2Len: c_int,
                                                              open: c_int,
                                                              gap: c_int)
                                                              -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_scan_profile_knc_512_32(profile: *const ParasailProfile,
                                                            s2: *const u8,
                                                            s2Len: c_int,
                                                            open: c_int,
                                                            gap: c_int)
                                                            -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_striped_profile_sse2_128_64(profile: *const ParasailProfile,
                                                                s2: *const u8,
                                                                s2Len: c_int,
                                                                open: c_int,
                                                                gap: c_int)
                                                                -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_striped_profile_sse2_128_32(profile: *const ParasailProfile,
                                                                s2: *const u8,
                                                                s2Len: c_int,
                                                                open: c_int,
                                                                gap: c_int)
                                                                -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_striped_profile_sse2_128_16(profile: *const ParasailProfile,
                                                                s2: *const u8,
                                                                s2Len: c_int,
                                                                open: c_int,
                                                                gap: c_int)
                                                                -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_striped_profile_sse2_128_8(profile: *const ParasailProfile,
                                                               s2: *const u8,
                                                               s2Len: c_int,
                                                               open: c_int,
                                                               gap: c_int)
                                                               -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_striped_profile_sse2_128_sat(profile: *const ParasailProfile,
                                                                 s2: *const u8,
                                                                 s2Len: c_int,
                                                                 open: c_int,
                                                                 gap: c_int)
                                                                 -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_striped_profile_sse41_128_64(profile: *const ParasailProfile,
                                                                 s2: *const u8,
                                                                 s2Len: c_int,
                                                                 open: c_int,
                                                                 gap: c_int)
                                                                 -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_striped_profile_sse41_128_32(profile: *const ParasailProfile,
                                                                 s2: *const u8,
                                                                 s2Len: c_int,
                                                                 open: c_int,
                                                                 gap: c_int)
                                                                 -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_striped_profile_sse41_128_16(profile: *const ParasailProfile,
                                                                 s2: *const u8,
                                                                 s2Len: c_int,
                                                                 open: c_int,
                                                                 gap: c_int)
                                                                 -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_striped_profile_sse41_128_8(profile: *const ParasailProfile,
                                                                s2: *const u8,
                                                                s2Len: c_int,
                                                                open: c_int,
                                                                gap: c_int)
                                                                -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_striped_profile_sse41_128_sat(profile: *const ParasailProfile,
                                                                  s2: *const u8,
                                                                  s2Len: c_int,
                                                                  open: c_int,
                                                                  gap: c_int)
                                                                  -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_striped_profile_avx2_256_64(profile: *const ParasailProfile,
                                                                s2: *const u8,
                                                                s2Len: c_int,
                                                                open: c_int,
                                                                gap: c_int)
                                                                -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_striped_profile_avx2_256_32(profile: *const ParasailProfile,
                                                                s2: *const u8,
                                                                s2Len: c_int,
                                                                open: c_int,
                                                                gap: c_int)
                                                                -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_striped_profile_avx2_256_16(profile: *const ParasailProfile,
                                                                s2: *const u8,
                                                                s2Len: c_int,
                                                                open: c_int,
                                                                gap: c_int)
                                                                -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_striped_profile_avx2_256_8(profile: *const ParasailProfile,
                                                               s2: *const u8,
                                                               s2Len: c_int,
                                                               open: c_int,
                                                               gap: c_int)
                                                               -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_striped_profile_avx2_256_sat(profile: *const ParasailProfile,
                                                                 s2: *const u8,
                                                                 s2Len: c_int,
                                                                 open: c_int,
                                                                 gap: c_int)
                                                                 -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_striped_profile_knc_512_32(profile: *const ParasailProfile,
                                                               s2: *const u8,
                                                               s2Len: c_int,
                                                               open: c_int,
                                                               gap: c_int)
                                                               -> *mut ParasailResult;
    pub fn parasail_sg_scan_profile_sse2_128_64(profile: *const ParasailProfile,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int)
                                                -> *mut ParasailResult;
    pub fn parasail_sg_scan_profile_sse2_128_32(profile: *const ParasailProfile,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int)
                                                -> *mut ParasailResult;
    pub fn parasail_sg_scan_profile_sse2_128_16(profile: *const ParasailProfile,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int)
                                                -> *mut ParasailResult;
    pub fn parasail_sg_scan_profile_sse2_128_8(profile: *const ParasailProfile,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int)
                                               -> *mut ParasailResult;
    pub fn parasail_sg_scan_profile_sse2_128_sat(profile: *const ParasailProfile,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int)
                                                 -> *mut ParasailResult;
    pub fn parasail_sg_scan_profile_sse41_128_64(profile: *const ParasailProfile,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int)
                                                 -> *mut ParasailResult;
    pub fn parasail_sg_scan_profile_sse41_128_32(profile: *const ParasailProfile,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int)
                                                 -> *mut ParasailResult;
    pub fn parasail_sg_scan_profile_sse41_128_16(profile: *const ParasailProfile,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int)
                                                 -> *mut ParasailResult;
    pub fn parasail_sg_scan_profile_sse41_128_8(profile: *const ParasailProfile,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int)
                                                -> *mut ParasailResult;
    pub fn parasail_sg_scan_profile_sse41_128_sat(profile: *const ParasailProfile,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int)
                                                  -> *mut ParasailResult;
    pub fn parasail_sg_scan_profile_avx2_256_64(profile: *const ParasailProfile,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int)
                                                -> *mut ParasailResult;
    pub fn parasail_sg_scan_profile_avx2_256_32(profile: *const ParasailProfile,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int)
                                                -> *mut ParasailResult;
    pub fn parasail_sg_scan_profile_avx2_256_16(profile: *const ParasailProfile,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int)
                                                -> *mut ParasailResult;
    pub fn parasail_sg_scan_profile_avx2_256_8(profile: *const ParasailProfile,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int)
                                               -> *mut ParasailResult;
    pub fn parasail_sg_scan_profile_avx2_256_sat(profile: *const ParasailProfile,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int)
                                                 -> *mut ParasailResult;
    pub fn parasail_sg_scan_profile_knc_512_32(profile: *const ParasailProfile,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int)
                                               -> *mut ParasailResult;
    pub fn parasail_sg_striped_profile_sse2_128_64(profile: *const ParasailProfile,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int)
                                                   -> *mut ParasailResult;
    pub fn parasail_sg_striped_profile_sse2_128_32(profile: *const ParasailProfile,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int)
                                                   -> *mut ParasailResult;
    pub fn parasail_sg_striped_profile_sse2_128_16(profile: *const ParasailProfile,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int)
                                                   -> *mut ParasailResult;
    pub fn parasail_sg_striped_profile_sse2_128_8(profile: *const ParasailProfile,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int)
                                                  -> *mut ParasailResult;
    pub fn parasail_sg_striped_profile_sse2_128_sat(profile: *const ParasailProfile,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int)
                                                    -> *mut ParasailResult;
    pub fn parasail_sg_striped_profile_sse41_128_64(profile: *const ParasailProfile,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int)
                                                    -> *mut ParasailResult;
    pub fn parasail_sg_striped_profile_sse41_128_32(profile: *const ParasailProfile,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int)
                                                    -> *mut ParasailResult;
    pub fn parasail_sg_striped_profile_sse41_128_16(profile: *const ParasailProfile,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int)
                                                    -> *mut ParasailResult;
    pub fn parasail_sg_striped_profile_sse41_128_8(profile: *const ParasailProfile,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int)
                                                   -> *mut ParasailResult;
    pub fn parasail_sg_striped_profile_sse41_128_sat(profile: *const ParasailProfile,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int)
                                                     -> *mut ParasailResult;
    pub fn parasail_sg_striped_profile_avx2_256_64(profile: *const ParasailProfile,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int)
                                                   -> *mut ParasailResult;
    pub fn parasail_sg_striped_profile_avx2_256_32(profile: *const ParasailProfile,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int)
                                                   -> *mut ParasailResult;
    pub fn parasail_sg_striped_profile_avx2_256_16(profile: *const ParasailProfile,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int)
                                                   -> *mut ParasailResult;
    pub fn parasail_sg_striped_profile_avx2_256_8(profile: *const ParasailProfile,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int)
                                                  -> *mut ParasailResult;
    pub fn parasail_sg_striped_profile_avx2_256_sat(profile: *const ParasailProfile,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int)
                                                    -> *mut ParasailResult;
    pub fn parasail_sg_striped_profile_knc_512_32(profile: *const ParasailProfile,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int)
                                                  -> *mut ParasailResult;
    pub fn parasail_sg_table_scan_profile_sse2_128_64(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_sg_table_scan_profile_sse2_128_32(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_sg_table_scan_profile_sse2_128_16(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_sg_table_scan_profile_sse2_128_8(profile: *const ParasailProfile,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int)
                                                     -> *mut ParasailResult;
    pub fn parasail_sg_table_scan_profile_sse2_128_sat(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_sg_table_scan_profile_sse41_128_64(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_sg_table_scan_profile_sse41_128_32(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_sg_table_scan_profile_sse41_128_16(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_sg_table_scan_profile_sse41_128_8(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_sg_table_scan_profile_sse41_128_sat(profile: *const ParasailProfile,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int)
                                                        -> *mut ParasailResult;
    pub fn parasail_sg_table_scan_profile_avx2_256_64(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_sg_table_scan_profile_avx2_256_32(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_sg_table_scan_profile_avx2_256_16(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_sg_table_scan_profile_avx2_256_8(profile: *const ParasailProfile,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int)
                                                     -> *mut ParasailResult;
    pub fn parasail_sg_table_scan_profile_avx2_256_sat(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_sg_table_scan_profile_knc_512_32(profile: *const ParasailProfile,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int)
                                                     -> *mut ParasailResult;
    pub fn parasail_sg_table_striped_profile_sse2_128_64(profile: *const ParasailProfile,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int)
                                                         -> *mut ParasailResult;
    pub fn parasail_sg_table_striped_profile_sse2_128_32(profile: *const ParasailProfile,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int)
                                                         -> *mut ParasailResult;
    pub fn parasail_sg_table_striped_profile_sse2_128_16(profile: *const ParasailProfile,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int)
                                                         -> *mut ParasailResult;
    pub fn parasail_sg_table_striped_profile_sse2_128_8(profile: *const ParasailProfile,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int)
                                                        -> *mut ParasailResult;
    pub fn parasail_sg_table_striped_profile_sse2_128_sat(profile: *const ParasailProfile,
                                                          s2: *const u8,
                                                          s2Len: c_int,
                                                          open: c_int,
                                                          gap: c_int)
                                                          -> *mut ParasailResult;
    pub fn parasail_sg_table_striped_profile_sse41_128_64(profile: *const ParasailProfile,
                                                          s2: *const u8,
                                                          s2Len: c_int,
                                                          open: c_int,
                                                          gap: c_int)
                                                          -> *mut ParasailResult;
    pub fn parasail_sg_table_striped_profile_sse41_128_32(profile: *const ParasailProfile,
                                                          s2: *const u8,
                                                          s2Len: c_int,
                                                          open: c_int,
                                                          gap: c_int)
                                                          -> *mut ParasailResult;
    pub fn parasail_sg_table_striped_profile_sse41_128_16(profile: *const ParasailProfile,
                                                          s2: *const u8,
                                                          s2Len: c_int,
                                                          open: c_int,
                                                          gap: c_int)
                                                          -> *mut ParasailResult;
    pub fn parasail_sg_table_striped_profile_sse41_128_8(profile: *const ParasailProfile,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int)
                                                         -> *mut ParasailResult;
    pub fn parasail_sg_table_striped_profile_sse41_128_sat(profile: *const ParasailProfile,
                                                           s2: *const u8,
                                                           s2Len: c_int,
                                                           open: c_int,
                                                           gap: c_int)
                                                           -> *mut ParasailResult;
    pub fn parasail_sg_table_striped_profile_avx2_256_64(profile: *const ParasailProfile,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int)
                                                         -> *mut ParasailResult;
    pub fn parasail_sg_table_striped_profile_avx2_256_32(profile: *const ParasailProfile,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int)
                                                         -> *mut ParasailResult;
    pub fn parasail_sg_table_striped_profile_avx2_256_16(profile: *const ParasailProfile,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int)
                                                         -> *mut ParasailResult;
    pub fn parasail_sg_table_striped_profile_avx2_256_8(profile: *const ParasailProfile,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int)
                                                        -> *mut ParasailResult;
    pub fn parasail_sg_table_striped_profile_avx2_256_sat(profile: *const ParasailProfile,
                                                          s2: *const u8,
                                                          s2Len: c_int,
                                                          open: c_int,
                                                          gap: c_int)
                                                          -> *mut ParasailResult;
    pub fn parasail_sg_table_striped_profile_knc_512_32(profile: *const ParasailProfile,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int)
                                                        -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_scan_profile_sse2_128_64(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_scan_profile_sse2_128_32(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_scan_profile_sse2_128_16(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_scan_profile_sse2_128_8(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_scan_profile_sse2_128_sat(profile: *const ParasailProfile,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int)
                                                        -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_scan_profile_sse41_128_64(profile: *const ParasailProfile,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int)
                                                        -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_scan_profile_sse41_128_32(profile: *const ParasailProfile,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int)
                                                        -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_scan_profile_sse41_128_16(profile: *const ParasailProfile,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int)
                                                        -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_scan_profile_sse41_128_8(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_scan_profile_sse41_128_sat(profile: *const ParasailProfile,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int)
                                                         -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_scan_profile_avx2_256_64(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_scan_profile_avx2_256_32(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_scan_profile_avx2_256_16(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_scan_profile_avx2_256_8(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_scan_profile_avx2_256_sat(profile: *const ParasailProfile,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int)
                                                        -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_scan_profile_knc_512_32(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_striped_profile_sse2_128_64(profile: *const ParasailProfile,
                                                          s2: *const u8,
                                                          s2Len: c_int,
                                                          open: c_int,
                                                          gap: c_int)
                                                          -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_striped_profile_sse2_128_32(profile: *const ParasailProfile,
                                                          s2: *const u8,
                                                          s2Len: c_int,
                                                          open: c_int,
                                                          gap: c_int)
                                                          -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_striped_profile_sse2_128_16(profile: *const ParasailProfile,
                                                          s2: *const u8,
                                                          s2Len: c_int,
                                                          open: c_int,
                                                          gap: c_int)
                                                          -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_striped_profile_sse2_128_8(profile: *const ParasailProfile,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int)
                                                         -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_striped_profile_sse2_128_sat(profile: *const ParasailProfile,
                                                           s2: *const u8,
                                                           s2Len: c_int,
                                                           open: c_int,
                                                           gap: c_int)
                                                           -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_striped_profile_sse41_128_64(profile: *const ParasailProfile,
                                                           s2: *const u8,
                                                           s2Len: c_int,
                                                           open: c_int,
                                                           gap: c_int)
                                                           -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_striped_profile_sse41_128_32(profile: *const ParasailProfile,
                                                           s2: *const u8,
                                                           s2Len: c_int,
                                                           open: c_int,
                                                           gap: c_int)
                                                           -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_striped_profile_sse41_128_16(profile: *const ParasailProfile,
                                                           s2: *const u8,
                                                           s2Len: c_int,
                                                           open: c_int,
                                                           gap: c_int)
                                                           -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_striped_profile_sse41_128_8(profile: *const ParasailProfile,
                                                          s2: *const u8,
                                                          s2Len: c_int,
                                                          open: c_int,
                                                          gap: c_int)
                                                          -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_striped_profile_sse41_128_sat(profile: *const ParasailProfile,
                                                            s2: *const u8,
                                                            s2Len: c_int,
                                                            open: c_int,
                                                            gap: c_int)
                                                            -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_striped_profile_avx2_256_64(profile: *const ParasailProfile,
                                                          s2: *const u8,
                                                          s2Len: c_int,
                                                          open: c_int,
                                                          gap: c_int)
                                                          -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_striped_profile_avx2_256_32(profile: *const ParasailProfile,
                                                          s2: *const u8,
                                                          s2Len: c_int,
                                                          open: c_int,
                                                          gap: c_int)
                                                          -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_striped_profile_avx2_256_16(profile: *const ParasailProfile,
                                                          s2: *const u8,
                                                          s2Len: c_int,
                                                          open: c_int,
                                                          gap: c_int)
                                                          -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_striped_profile_avx2_256_8(profile: *const ParasailProfile,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int)
                                                         -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_striped_profile_avx2_256_sat(profile: *const ParasailProfile,
                                                           s2: *const u8,
                                                           s2Len: c_int,
                                                           open: c_int,
                                                           gap: c_int)
                                                           -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_striped_profile_knc_512_32(profile: *const ParasailProfile,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int)
                                                         -> *mut ParasailResult;
    pub fn parasail_sg_stats_scan_profile_sse2_128_64(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_sg_stats_scan_profile_sse2_128_32(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_sg_stats_scan_profile_sse2_128_16(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_sg_stats_scan_profile_sse2_128_8(profile: *const ParasailProfile,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int)
                                                     -> *mut ParasailResult;
    pub fn parasail_sg_stats_scan_profile_sse2_128_sat(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_sg_stats_scan_profile_sse41_128_64(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_sg_stats_scan_profile_sse41_128_32(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_sg_stats_scan_profile_sse41_128_16(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_sg_stats_scan_profile_sse41_128_8(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_sg_stats_scan_profile_sse41_128_sat(profile: *const ParasailProfile,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int)
                                                        -> *mut ParasailResult;
    pub fn parasail_sg_stats_scan_profile_avx2_256_64(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_sg_stats_scan_profile_avx2_256_32(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_sg_stats_scan_profile_avx2_256_16(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_sg_stats_scan_profile_avx2_256_8(profile: *const ParasailProfile,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int)
                                                     -> *mut ParasailResult;
    pub fn parasail_sg_stats_scan_profile_avx2_256_sat(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_sg_stats_scan_profile_knc_512_32(profile: *const ParasailProfile,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int)
                                                     -> *mut ParasailResult;
    pub fn parasail_sg_stats_striped_profile_sse2_128_64(profile: *const ParasailProfile,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int)
                                                         -> *mut ParasailResult;
    pub fn parasail_sg_stats_striped_profile_sse2_128_32(profile: *const ParasailProfile,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int)
                                                         -> *mut ParasailResult;
    pub fn parasail_sg_stats_striped_profile_sse2_128_16(profile: *const ParasailProfile,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int)
                                                         -> *mut ParasailResult;
    pub fn parasail_sg_stats_striped_profile_sse2_128_8(profile: *const ParasailProfile,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int)
                                                        -> *mut ParasailResult;
    pub fn parasail_sg_stats_striped_profile_sse2_128_sat(profile: *const ParasailProfile,
                                                          s2: *const u8,
                                                          s2Len: c_int,
                                                          open: c_int,
                                                          gap: c_int)
                                                          -> *mut ParasailResult;
    pub fn parasail_sg_stats_striped_profile_sse41_128_64(profile: *const ParasailProfile,
                                                          s2: *const u8,
                                                          s2Len: c_int,
                                                          open: c_int,
                                                          gap: c_int)
                                                          -> *mut ParasailResult;
    pub fn parasail_sg_stats_striped_profile_sse41_128_32(profile: *const ParasailProfile,
                                                          s2: *const u8,
                                                          s2Len: c_int,
                                                          open: c_int,
                                                          gap: c_int)
                                                          -> *mut ParasailResult;
    pub fn parasail_sg_stats_striped_profile_sse41_128_16(profile: *const ParasailProfile,
                                                          s2: *const u8,
                                                          s2Len: c_int,
                                                          open: c_int,
                                                          gap: c_int)
                                                          -> *mut ParasailResult;
    pub fn parasail_sg_stats_striped_profile_sse41_128_8(profile: *const ParasailProfile,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int)
                                                         -> *mut ParasailResult;
    pub fn parasail_sg_stats_striped_profile_sse41_128_sat(profile: *const ParasailProfile,
                                                           s2: *const u8,
                                                           s2Len: c_int,
                                                           open: c_int,
                                                           gap: c_int)
                                                           -> *mut ParasailResult;
    pub fn parasail_sg_stats_striped_profile_avx2_256_64(profile: *const ParasailProfile,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int)
                                                         -> *mut ParasailResult;
    pub fn parasail_sg_stats_striped_profile_avx2_256_32(profile: *const ParasailProfile,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int)
                                                         -> *mut ParasailResult;
    pub fn parasail_sg_stats_striped_profile_avx2_256_16(profile: *const ParasailProfile,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int)
                                                         -> *mut ParasailResult;
    pub fn parasail_sg_stats_striped_profile_avx2_256_8(profile: *const ParasailProfile,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int)
                                                        -> *mut ParasailResult;
    pub fn parasail_sg_stats_striped_profile_avx2_256_sat(profile: *const ParasailProfile,
                                                          s2: *const u8,
                                                          s2Len: c_int,
                                                          open: c_int,
                                                          gap: c_int)
                                                          -> *mut ParasailResult;
    pub fn parasail_sg_stats_striped_profile_knc_512_32(profile: *const ParasailProfile,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int)
                                                        -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_scan_profile_sse2_128_64(profile: *const ParasailProfile,
                                                            s2: *const u8,
                                                            s2Len: c_int,
                                                            open: c_int,
                                                            gap: c_int)
                                                            -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_scan_profile_sse2_128_32(profile: *const ParasailProfile,
                                                            s2: *const u8,
                                                            s2Len: c_int,
                                                            open: c_int,
                                                            gap: c_int)
                                                            -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_scan_profile_sse2_128_16(profile: *const ParasailProfile,
                                                            s2: *const u8,
                                                            s2Len: c_int,
                                                            open: c_int,
                                                            gap: c_int)
                                                            -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_scan_profile_sse2_128_8(profile: *const ParasailProfile,
                                                           s2: *const u8,
                                                           s2Len: c_int,
                                                           open: c_int,
                                                           gap: c_int)
                                                           -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_scan_profile_sse2_128_sat(profile: *const ParasailProfile,
                                                             s2: *const u8,
                                                             s2Len: c_int,
                                                             open: c_int,
                                                             gap: c_int)
                                                             -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_scan_profile_sse41_128_64(profile: *const ParasailProfile,
                                                             s2: *const u8,
                                                             s2Len: c_int,
                                                             open: c_int,
                                                             gap: c_int)
                                                             -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_scan_profile_sse41_128_32(profile: *const ParasailProfile,
                                                             s2: *const u8,
                                                             s2Len: c_int,
                                                             open: c_int,
                                                             gap: c_int)
                                                             -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_scan_profile_sse41_128_16(profile: *const ParasailProfile,
                                                             s2: *const u8,
                                                             s2Len: c_int,
                                                             open: c_int,
                                                             gap: c_int)
                                                             -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_scan_profile_sse41_128_8(profile: *const ParasailProfile,
                                                            s2: *const u8,
                                                            s2Len: c_int,
                                                            open: c_int,
                                                            gap: c_int)
                                                            -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_scan_profile_sse41_128_sat(profile: *const ParasailProfile,
                                                              s2: *const u8,
                                                              s2Len: c_int,
                                                              open: c_int,
                                                              gap: c_int)
                                                              -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_scan_profile_avx2_256_64(profile: *const ParasailProfile,
                                                            s2: *const u8,
                                                            s2Len: c_int,
                                                            open: c_int,
                                                            gap: c_int)
                                                            -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_scan_profile_avx2_256_32(profile: *const ParasailProfile,
                                                            s2: *const u8,
                                                            s2Len: c_int,
                                                            open: c_int,
                                                            gap: c_int)
                                                            -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_scan_profile_avx2_256_16(profile: *const ParasailProfile,
                                                            s2: *const u8,
                                                            s2Len: c_int,
                                                            open: c_int,
                                                            gap: c_int)
                                                            -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_scan_profile_avx2_256_8(profile: *const ParasailProfile,
                                                           s2: *const u8,
                                                           s2Len: c_int,
                                                           open: c_int,
                                                           gap: c_int)
                                                           -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_scan_profile_avx2_256_sat(profile: *const ParasailProfile,
                                                             s2: *const u8,
                                                             s2Len: c_int,
                                                             open: c_int,
                                                             gap: c_int)
                                                             -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_scan_profile_knc_512_32(profile: *const ParasailProfile,
                                                           s2: *const u8,
                                                           s2Len: c_int,
                                                           open: c_int,
                                                           gap: c_int)
                                                           -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_striped_profile_sse2_128_64(profile: *const ParasailProfile,
                                                               s2: *const u8,
                                                               s2Len: c_int,
                                                               open: c_int,
                                                               gap: c_int)
                                                               -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_striped_profile_sse2_128_32(profile: *const ParasailProfile,
                                                               s2: *const u8,
                                                               s2Len: c_int,
                                                               open: c_int,
                                                               gap: c_int)
                                                               -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_striped_profile_sse2_128_16(profile: *const ParasailProfile,
                                                               s2: *const u8,
                                                               s2Len: c_int,
                                                               open: c_int,
                                                               gap: c_int)
                                                               -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_striped_profile_sse2_128_8(profile: *const ParasailProfile,
                                                              s2: *const u8,
                                                              s2Len: c_int,
                                                              open: c_int,
                                                              gap: c_int)
                                                              -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_striped_profile_sse2_128_sat(profile: *const ParasailProfile,
                                                                s2: *const u8,
                                                                s2Len: c_int,
                                                                open: c_int,
                                                                gap: c_int)
                                                                -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_striped_profile_sse41_128_64(profile: *const ParasailProfile,
                                                                s2: *const u8,
                                                                s2Len: c_int,
                                                                open: c_int,
                                                                gap: c_int)
                                                                -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_striped_profile_sse41_128_32(profile: *const ParasailProfile,
                                                                s2: *const u8,
                                                                s2Len: c_int,
                                                                open: c_int,
                                                                gap: c_int)
                                                                -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_striped_profile_sse41_128_16(profile: *const ParasailProfile,
                                                                s2: *const u8,
                                                                s2Len: c_int,
                                                                open: c_int,
                                                                gap: c_int)
                                                                -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_striped_profile_sse41_128_8(profile: *const ParasailProfile,
                                                               s2: *const u8,
                                                               s2Len: c_int,
                                                               open: c_int,
                                                               gap: c_int)
                                                               -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_striped_profile_sse41_128_sat(profile: *const ParasailProfile,
                                                                 s2: *const u8,
                                                                 s2Len: c_int,
                                                                 open: c_int,
                                                                 gap: c_int)
                                                                 -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_striped_profile_avx2_256_64(profile: *const ParasailProfile,
                                                               s2: *const u8,
                                                               s2Len: c_int,
                                                               open: c_int,
                                                               gap: c_int)
                                                               -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_striped_profile_avx2_256_32(profile: *const ParasailProfile,
                                                               s2: *const u8,
                                                               s2Len: c_int,
                                                               open: c_int,
                                                               gap: c_int)
                                                               -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_striped_profile_avx2_256_16(profile: *const ParasailProfile,
                                                               s2: *const u8,
                                                               s2Len: c_int,
                                                               open: c_int,
                                                               gap: c_int)
                                                               -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_striped_profile_avx2_256_8(profile: *const ParasailProfile,
                                                              s2: *const u8,
                                                              s2Len: c_int,
                                                              open: c_int,
                                                              gap: c_int)
                                                              -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_striped_profile_avx2_256_sat(profile: *const ParasailProfile,
                                                                s2: *const u8,
                                                                s2Len: c_int,
                                                                open: c_int,
                                                                gap: c_int)
                                                                -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_striped_profile_knc_512_32(profile: *const ParasailProfile,
                                                              s2: *const u8,
                                                              s2Len: c_int,
                                                              open: c_int,
                                                              gap: c_int)
                                                              -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_scan_profile_sse2_128_64(profile: *const ParasailProfile,
                                                             s2: *const u8,
                                                             s2Len: c_int,
                                                             open: c_int,
                                                             gap: c_int)
                                                             -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_scan_profile_sse2_128_32(profile: *const ParasailProfile,
                                                             s2: *const u8,
                                                             s2Len: c_int,
                                                             open: c_int,
                                                             gap: c_int)
                                                             -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_scan_profile_sse2_128_16(profile: *const ParasailProfile,
                                                             s2: *const u8,
                                                             s2Len: c_int,
                                                             open: c_int,
                                                             gap: c_int)
                                                             -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_scan_profile_sse2_128_8(profile: *const ParasailProfile,
                                                            s2: *const u8,
                                                            s2Len: c_int,
                                                            open: c_int,
                                                            gap: c_int)
                                                            -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_scan_profile_sse2_128_sat(profile: *const ParasailProfile,
                                                              s2: *const u8,
                                                              s2Len: c_int,
                                                              open: c_int,
                                                              gap: c_int)
                                                              -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_scan_profile_sse41_128_64(profile: *const ParasailProfile,
                                                              s2: *const u8,
                                                              s2Len: c_int,
                                                              open: c_int,
                                                              gap: c_int)
                                                              -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_scan_profile_sse41_128_32(profile: *const ParasailProfile,
                                                              s2: *const u8,
                                                              s2Len: c_int,
                                                              open: c_int,
                                                              gap: c_int)
                                                              -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_scan_profile_sse41_128_16(profile: *const ParasailProfile,
                                                              s2: *const u8,
                                                              s2Len: c_int,
                                                              open: c_int,
                                                              gap: c_int)
                                                              -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_scan_profile_sse41_128_8(profile: *const ParasailProfile,
                                                             s2: *const u8,
                                                             s2Len: c_int,
                                                             open: c_int,
                                                             gap: c_int)
                                                             -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_scan_profile_sse41_128_sat(profile: *const ParasailProfile,
                                                               s2: *const u8,
                                                               s2Len: c_int,
                                                               open: c_int,
                                                               gap: c_int)
                                                               -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_scan_profile_avx2_256_64(profile: *const ParasailProfile,
                                                             s2: *const u8,
                                                             s2Len: c_int,
                                                             open: c_int,
                                                             gap: c_int)
                                                             -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_scan_profile_avx2_256_32(profile: *const ParasailProfile,
                                                             s2: *const u8,
                                                             s2Len: c_int,
                                                             open: c_int,
                                                             gap: c_int)
                                                             -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_scan_profile_avx2_256_16(profile: *const ParasailProfile,
                                                             s2: *const u8,
                                                             s2Len: c_int,
                                                             open: c_int,
                                                             gap: c_int)
                                                             -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_scan_profile_avx2_256_8(profile: *const ParasailProfile,
                                                            s2: *const u8,
                                                            s2Len: c_int,
                                                            open: c_int,
                                                            gap: c_int)
                                                            -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_scan_profile_avx2_256_sat(profile: *const ParasailProfile,
                                                              s2: *const u8,
                                                              s2Len: c_int,
                                                              open: c_int,
                                                              gap: c_int)
                                                              -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_scan_profile_knc_512_32(profile: *const ParasailProfile,
                                                            s2: *const u8,
                                                            s2Len: c_int,
                                                            open: c_int,
                                                            gap: c_int)
                                                            -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_striped_profile_sse2_128_64(profile: *const ParasailProfile,
                                                                s2: *const u8,
                                                                s2Len: c_int,
                                                                open: c_int,
                                                                gap: c_int)
                                                                -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_striped_profile_sse2_128_32(profile: *const ParasailProfile,
                                                                s2: *const u8,
                                                                s2Len: c_int,
                                                                open: c_int,
                                                                gap: c_int)
                                                                -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_striped_profile_sse2_128_16(profile: *const ParasailProfile,
                                                                s2: *const u8,
                                                                s2Len: c_int,
                                                                open: c_int,
                                                                gap: c_int)
                                                                -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_striped_profile_sse2_128_8(profile: *const ParasailProfile,
                                                               s2: *const u8,
                                                               s2Len: c_int,
                                                               open: c_int,
                                                               gap: c_int)
                                                               -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_striped_profile_sse2_128_sat(profile: *const ParasailProfile,
                                                                 s2: *const u8,
                                                                 s2Len: c_int,
                                                                 open: c_int,
                                                                 gap: c_int)
                                                                 -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_striped_profile_sse41_128_64(profile: *const ParasailProfile,
                                                                 s2: *const u8,
                                                                 s2Len: c_int,
                                                                 open: c_int,
                                                                 gap: c_int)
                                                                 -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_striped_profile_sse41_128_32(profile: *const ParasailProfile,
                                                                 s2: *const u8,
                                                                 s2Len: c_int,
                                                                 open: c_int,
                                                                 gap: c_int)
                                                                 -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_striped_profile_sse41_128_16(profile: *const ParasailProfile,
                                                                 s2: *const u8,
                                                                 s2Len: c_int,
                                                                 open: c_int,
                                                                 gap: c_int)
                                                                 -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_striped_profile_sse41_128_8(profile: *const ParasailProfile,
                                                                s2: *const u8,
                                                                s2Len: c_int,
                                                                open: c_int,
                                                                gap: c_int)
                                                                -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_striped_profile_sse41_128_sat(profile: *const ParasailProfile,
                                                                  s2: *const u8,
                                                                  s2Len: c_int,
                                                                  open: c_int,
                                                                  gap: c_int)
                                                                  -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_striped_profile_avx2_256_64(profile: *const ParasailProfile,
                                                                s2: *const u8,
                                                                s2Len: c_int,
                                                                open: c_int,
                                                                gap: c_int)
                                                                -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_striped_profile_avx2_256_32(profile: *const ParasailProfile,
                                                                s2: *const u8,
                                                                s2Len: c_int,
                                                                open: c_int,
                                                                gap: c_int)
                                                                -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_striped_profile_avx2_256_16(profile: *const ParasailProfile,
                                                                s2: *const u8,
                                                                s2Len: c_int,
                                                                open: c_int,
                                                                gap: c_int)
                                                                -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_striped_profile_avx2_256_8(profile: *const ParasailProfile,
                                                               s2: *const u8,
                                                               s2Len: c_int,
                                                               open: c_int,
                                                               gap: c_int)
                                                               -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_striped_profile_avx2_256_sat(profile: *const ParasailProfile,
                                                                 s2: *const u8,
                                                                 s2Len: c_int,
                                                                 open: c_int,
                                                                 gap: c_int)
                                                                 -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_striped_profile_knc_512_32(profile: *const ParasailProfile,
                                                               s2: *const u8,
                                                               s2Len: c_int,
                                                               open: c_int,
                                                               gap: c_int)
                                                               -> *mut ParasailResult;
    pub fn parasail_sw_scan_profile_sse2_128_64(profile: *const ParasailProfile,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int)
                                                -> *mut ParasailResult;
    pub fn parasail_sw_scan_profile_sse2_128_32(profile: *const ParasailProfile,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int)
                                                -> *mut ParasailResult;
    pub fn parasail_sw_scan_profile_sse2_128_16(profile: *const ParasailProfile,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int)
                                                -> *mut ParasailResult;
    pub fn parasail_sw_scan_profile_sse2_128_8(profile: *const ParasailProfile,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int)
                                               -> *mut ParasailResult;
    pub fn parasail_sw_scan_profile_sse2_128_sat(profile: *const ParasailProfile,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int)
                                                 -> *mut ParasailResult;
    pub fn parasail_sw_scan_profile_sse41_128_64(profile: *const ParasailProfile,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int)
                                                 -> *mut ParasailResult;
    pub fn parasail_sw_scan_profile_sse41_128_32(profile: *const ParasailProfile,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int)
                                                 -> *mut ParasailResult;
    pub fn parasail_sw_scan_profile_sse41_128_16(profile: *const ParasailProfile,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int)
                                                 -> *mut ParasailResult;
    pub fn parasail_sw_scan_profile_sse41_128_8(profile: *const ParasailProfile,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int)
                                                -> *mut ParasailResult;
    pub fn parasail_sw_scan_profile_sse41_128_sat(profile: *const ParasailProfile,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int)
                                                  -> *mut ParasailResult;
    pub fn parasail_sw_scan_profile_avx2_256_64(profile: *const ParasailProfile,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int)
                                                -> *mut ParasailResult;
    pub fn parasail_sw_scan_profile_avx2_256_32(profile: *const ParasailProfile,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int)
                                                -> *mut ParasailResult;
    pub fn parasail_sw_scan_profile_avx2_256_16(profile: *const ParasailProfile,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int)
                                                -> *mut ParasailResult;
    pub fn parasail_sw_scan_profile_avx2_256_8(profile: *const ParasailProfile,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int)
                                               -> *mut ParasailResult;
    pub fn parasail_sw_scan_profile_avx2_256_sat(profile: *const ParasailProfile,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int)
                                                 -> *mut ParasailResult;
    pub fn parasail_sw_scan_profile_knc_512_32(profile: *const ParasailProfile,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int)
                                               -> *mut ParasailResult;
    pub fn parasail_sw_striped_profile_sse2_128_64(profile: *const ParasailProfile,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int)
                                                   -> *mut ParasailResult;
    pub fn parasail_sw_striped_profile_sse2_128_32(profile: *const ParasailProfile,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int)
                                                   -> *mut ParasailResult;
    pub fn parasail_sw_striped_profile_sse2_128_16(profile: *const ParasailProfile,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int)
                                                   -> *mut ParasailResult;
    pub fn parasail_sw_striped_profile_sse2_128_8(profile: *const ParasailProfile,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int)
                                                  -> *mut ParasailResult;
    pub fn parasail_sw_striped_profile_sse2_128_sat(profile: *const ParasailProfile,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int)
                                                    -> *mut ParasailResult;
    pub fn parasail_sw_striped_profile_sse41_128_64(profile: *const ParasailProfile,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int)
                                                    -> *mut ParasailResult;
    pub fn parasail_sw_striped_profile_sse41_128_32(profile: *const ParasailProfile,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int)
                                                    -> *mut ParasailResult;
    pub fn parasail_sw_striped_profile_sse41_128_16(profile: *const ParasailProfile,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int)
                                                    -> *mut ParasailResult;
    pub fn parasail_sw_striped_profile_sse41_128_8(profile: *const ParasailProfile,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int)
                                                   -> *mut ParasailResult;
    pub fn parasail_sw_striped_profile_sse41_128_sat(profile: *const ParasailProfile,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int)
                                                     -> *mut ParasailResult;
    pub fn parasail_sw_striped_profile_avx2_256_64(profile: *const ParasailProfile,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int)
                                                   -> *mut ParasailResult;
    pub fn parasail_sw_striped_profile_avx2_256_32(profile: *const ParasailProfile,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int)
                                                   -> *mut ParasailResult;
    pub fn parasail_sw_striped_profile_avx2_256_16(profile: *const ParasailProfile,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int)
                                                   -> *mut ParasailResult;
    pub fn parasail_sw_striped_profile_avx2_256_8(profile: *const ParasailProfile,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int)
                                                  -> *mut ParasailResult;
    pub fn parasail_sw_striped_profile_avx2_256_sat(profile: *const ParasailProfile,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int)
                                                    -> *mut ParasailResult;
    pub fn parasail_sw_striped_profile_knc_512_32(profile: *const ParasailProfile,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int)
                                                  -> *mut ParasailResult;
    pub fn parasail_sw_table_scan_profile_sse2_128_64(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_sw_table_scan_profile_sse2_128_32(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_sw_table_scan_profile_sse2_128_16(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_sw_table_scan_profile_sse2_128_8(profile: *const ParasailProfile,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int)
                                                     -> *mut ParasailResult;
    pub fn parasail_sw_table_scan_profile_sse2_128_sat(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_sw_table_scan_profile_sse41_128_64(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_sw_table_scan_profile_sse41_128_32(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_sw_table_scan_profile_sse41_128_16(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_sw_table_scan_profile_sse41_128_8(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_sw_table_scan_profile_sse41_128_sat(profile: *const ParasailProfile,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int)
                                                        -> *mut ParasailResult;
    pub fn parasail_sw_table_scan_profile_avx2_256_64(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_sw_table_scan_profile_avx2_256_32(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_sw_table_scan_profile_avx2_256_16(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_sw_table_scan_profile_avx2_256_8(profile: *const ParasailProfile,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int)
                                                     -> *mut ParasailResult;
    pub fn parasail_sw_table_scan_profile_avx2_256_sat(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_sw_table_scan_profile_knc_512_32(profile: *const ParasailProfile,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int)
                                                     -> *mut ParasailResult;
    pub fn parasail_sw_table_striped_profile_sse2_128_64(profile: *const ParasailProfile,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int)
                                                         -> *mut ParasailResult;
    pub fn parasail_sw_table_striped_profile_sse2_128_32(profile: *const ParasailProfile,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int)
                                                         -> *mut ParasailResult;
    pub fn parasail_sw_table_striped_profile_sse2_128_16(profile: *const ParasailProfile,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int)
                                                         -> *mut ParasailResult;
    pub fn parasail_sw_table_striped_profile_sse2_128_8(profile: *const ParasailProfile,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int)
                                                        -> *mut ParasailResult;
    pub fn parasail_sw_table_striped_profile_sse2_128_sat(profile: *const ParasailProfile,
                                                          s2: *const u8,
                                                          s2Len: c_int,
                                                          open: c_int,
                                                          gap: c_int)
                                                          -> *mut ParasailResult;
    pub fn parasail_sw_table_striped_profile_sse41_128_64(profile: *const ParasailProfile,
                                                          s2: *const u8,
                                                          s2Len: c_int,
                                                          open: c_int,
                                                          gap: c_int)
                                                          -> *mut ParasailResult;
    pub fn parasail_sw_table_striped_profile_sse41_128_32(profile: *const ParasailProfile,
                                                          s2: *const u8,
                                                          s2Len: c_int,
                                                          open: c_int,
                                                          gap: c_int)
                                                          -> *mut ParasailResult;
    pub fn parasail_sw_table_striped_profile_sse41_128_16(profile: *const ParasailProfile,
                                                          s2: *const u8,
                                                          s2Len: c_int,
                                                          open: c_int,
                                                          gap: c_int)
                                                          -> *mut ParasailResult;
    pub fn parasail_sw_table_striped_profile_sse41_128_8(profile: *const ParasailProfile,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int)
                                                         -> *mut ParasailResult;
    pub fn parasail_sw_table_striped_profile_sse41_128_sat(profile: *const ParasailProfile,
                                                           s2: *const u8,
                                                           s2Len: c_int,
                                                           open: c_int,
                                                           gap: c_int)
                                                           -> *mut ParasailResult;
    pub fn parasail_sw_table_striped_profile_avx2_256_64(profile: *const ParasailProfile,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int)
                                                         -> *mut ParasailResult;
    pub fn parasail_sw_table_striped_profile_avx2_256_32(profile: *const ParasailProfile,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int)
                                                         -> *mut ParasailResult;
    pub fn parasail_sw_table_striped_profile_avx2_256_16(profile: *const ParasailProfile,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int)
                                                         -> *mut ParasailResult;
    pub fn parasail_sw_table_striped_profile_avx2_256_8(profile: *const ParasailProfile,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int)
                                                        -> *mut ParasailResult;
    pub fn parasail_sw_table_striped_profile_avx2_256_sat(profile: *const ParasailProfile,
                                                          s2: *const u8,
                                                          s2Len: c_int,
                                                          open: c_int,
                                                          gap: c_int)
                                                          -> *mut ParasailResult;
    pub fn parasail_sw_table_striped_profile_knc_512_32(profile: *const ParasailProfile,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int)
                                                        -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_scan_profile_sse2_128_64(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_scan_profile_sse2_128_32(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_scan_profile_sse2_128_16(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_scan_profile_sse2_128_8(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_scan_profile_sse2_128_sat(profile: *const ParasailProfile,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int)
                                                        -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_scan_profile_sse41_128_64(profile: *const ParasailProfile,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int)
                                                        -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_scan_profile_sse41_128_32(profile: *const ParasailProfile,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int)
                                                        -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_scan_profile_sse41_128_16(profile: *const ParasailProfile,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int)
                                                        -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_scan_profile_sse41_128_8(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_scan_profile_sse41_128_sat(profile: *const ParasailProfile,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int)
                                                         -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_scan_profile_avx2_256_64(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_scan_profile_avx2_256_32(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_scan_profile_avx2_256_16(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_scan_profile_avx2_256_8(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_scan_profile_avx2_256_sat(profile: *const ParasailProfile,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int)
                                                        -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_scan_profile_knc_512_32(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_striped_profile_sse2_128_64(profile: *const ParasailProfile,
                                                          s2: *const u8,
                                                          s2Len: c_int,
                                                          open: c_int,
                                                          gap: c_int)
                                                          -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_striped_profile_sse2_128_32(profile: *const ParasailProfile,
                                                          s2: *const u8,
                                                          s2Len: c_int,
                                                          open: c_int,
                                                          gap: c_int)
                                                          -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_striped_profile_sse2_128_16(profile: *const ParasailProfile,
                                                          s2: *const u8,
                                                          s2Len: c_int,
                                                          open: c_int,
                                                          gap: c_int)
                                                          -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_striped_profile_sse2_128_8(profile: *const ParasailProfile,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int)
                                                         -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_striped_profile_sse2_128_sat(profile: *const ParasailProfile,
                                                           s2: *const u8,
                                                           s2Len: c_int,
                                                           open: c_int,
                                                           gap: c_int)
                                                           -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_striped_profile_sse41_128_64(profile: *const ParasailProfile,
                                                           s2: *const u8,
                                                           s2Len: c_int,
                                                           open: c_int,
                                                           gap: c_int)
                                                           -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_striped_profile_sse41_128_32(profile: *const ParasailProfile,
                                                           s2: *const u8,
                                                           s2Len: c_int,
                                                           open: c_int,
                                                           gap: c_int)
                                                           -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_striped_profile_sse41_128_16(profile: *const ParasailProfile,
                                                           s2: *const u8,
                                                           s2Len: c_int,
                                                           open: c_int,
                                                           gap: c_int)
                                                           -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_striped_profile_sse41_128_8(profile: *const ParasailProfile,
                                                          s2: *const u8,
                                                          s2Len: c_int,
                                                          open: c_int,
                                                          gap: c_int)
                                                          -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_striped_profile_sse41_128_sat(profile: *const ParasailProfile,
                                                            s2: *const u8,
                                                            s2Len: c_int,
                                                            open: c_int,
                                                            gap: c_int)
                                                            -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_striped_profile_avx2_256_64(profile: *const ParasailProfile,
                                                          s2: *const u8,
                                                          s2Len: c_int,
                                                          open: c_int,
                                                          gap: c_int)
                                                          -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_striped_profile_avx2_256_32(profile: *const ParasailProfile,
                                                          s2: *const u8,
                                                          s2Len: c_int,
                                                          open: c_int,
                                                          gap: c_int)
                                                          -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_striped_profile_avx2_256_16(profile: *const ParasailProfile,
                                                          s2: *const u8,
                                                          s2Len: c_int,
                                                          open: c_int,
                                                          gap: c_int)
                                                          -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_striped_profile_avx2_256_8(profile: *const ParasailProfile,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int)
                                                         -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_striped_profile_avx2_256_sat(profile: *const ParasailProfile,
                                                           s2: *const u8,
                                                           s2Len: c_int,
                                                           open: c_int,
                                                           gap: c_int)
                                                           -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_striped_profile_knc_512_32(profile: *const ParasailProfile,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int)
                                                         -> *mut ParasailResult;
    pub fn parasail_sw_stats_scan_profile_sse2_128_64(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_sw_stats_scan_profile_sse2_128_32(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_sw_stats_scan_profile_sse2_128_16(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_sw_stats_scan_profile_sse2_128_8(profile: *const ParasailProfile,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int)
                                                     -> *mut ParasailResult;
    pub fn parasail_sw_stats_scan_profile_sse2_128_sat(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_sw_stats_scan_profile_sse41_128_64(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_sw_stats_scan_profile_sse41_128_32(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_sw_stats_scan_profile_sse41_128_16(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_sw_stats_scan_profile_sse41_128_8(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_sw_stats_scan_profile_sse41_128_sat(profile: *const ParasailProfile,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int)
                                                        -> *mut ParasailResult;
    pub fn parasail_sw_stats_scan_profile_avx2_256_64(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_sw_stats_scan_profile_avx2_256_32(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_sw_stats_scan_profile_avx2_256_16(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_sw_stats_scan_profile_avx2_256_8(profile: *const ParasailProfile,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int)
                                                     -> *mut ParasailResult;
    pub fn parasail_sw_stats_scan_profile_avx2_256_sat(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_sw_stats_scan_profile_knc_512_32(profile: *const ParasailProfile,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int)
                                                     -> *mut ParasailResult;
    pub fn parasail_sw_stats_striped_profile_sse2_128_64(profile: *const ParasailProfile,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int)
                                                         -> *mut ParasailResult;
    pub fn parasail_sw_stats_striped_profile_sse2_128_32(profile: *const ParasailProfile,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int)
                                                         -> *mut ParasailResult;
    pub fn parasail_sw_stats_striped_profile_sse2_128_16(profile: *const ParasailProfile,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int)
                                                         -> *mut ParasailResult;
    pub fn parasail_sw_stats_striped_profile_sse2_128_8(profile: *const ParasailProfile,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int)
                                                        -> *mut ParasailResult;
    pub fn parasail_sw_stats_striped_profile_sse2_128_sat(profile: *const ParasailProfile,
                                                          s2: *const u8,
                                                          s2Len: c_int,
                                                          open: c_int,
                                                          gap: c_int)
                                                          -> *mut ParasailResult;
    pub fn parasail_sw_stats_striped_profile_sse41_128_64(profile: *const ParasailProfile,
                                                          s2: *const u8,
                                                          s2Len: c_int,
                                                          open: c_int,
                                                          gap: c_int)
                                                          -> *mut ParasailResult;
    pub fn parasail_sw_stats_striped_profile_sse41_128_32(profile: *const ParasailProfile,
                                                          s2: *const u8,
                                                          s2Len: c_int,
                                                          open: c_int,
                                                          gap: c_int)
                                                          -> *mut ParasailResult;
    pub fn parasail_sw_stats_striped_profile_sse41_128_16(profile: *const ParasailProfile,
                                                          s2: *const u8,
                                                          s2Len: c_int,
                                                          open: c_int,
                                                          gap: c_int)
                                                          -> *mut ParasailResult;
    pub fn parasail_sw_stats_striped_profile_sse41_128_8(profile: *const ParasailProfile,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int)
                                                         -> *mut ParasailResult;
    pub fn parasail_sw_stats_striped_profile_sse41_128_sat(profile: *const ParasailProfile,
                                                           s2: *const u8,
                                                           s2Len: c_int,
                                                           open: c_int,
                                                           gap: c_int)
                                                           -> *mut ParasailResult;
    pub fn parasail_sw_stats_striped_profile_avx2_256_64(profile: *const ParasailProfile,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int)
                                                         -> *mut ParasailResult;
    pub fn parasail_sw_stats_striped_profile_avx2_256_32(profile: *const ParasailProfile,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int)
                                                         -> *mut ParasailResult;
    pub fn parasail_sw_stats_striped_profile_avx2_256_16(profile: *const ParasailProfile,
                                                         s2: *const u8,
                                                         s2Len: c_int,
                                                         open: c_int,
                                                         gap: c_int)
                                                         -> *mut ParasailResult;
    pub fn parasail_sw_stats_striped_profile_avx2_256_8(profile: *const ParasailProfile,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int)
                                                        -> *mut ParasailResult;
    pub fn parasail_sw_stats_striped_profile_avx2_256_sat(profile: *const ParasailProfile,
                                                          s2: *const u8,
                                                          s2Len: c_int,
                                                          open: c_int,
                                                          gap: c_int)
                                                          -> *mut ParasailResult;
    pub fn parasail_sw_stats_striped_profile_knc_512_32(profile: *const ParasailProfile,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int)
                                                        -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_scan_profile_sse2_128_64(profile: *const ParasailProfile,
                                                            s2: *const u8,
                                                            s2Len: c_int,
                                                            open: c_int,
                                                            gap: c_int)
                                                            -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_scan_profile_sse2_128_32(profile: *const ParasailProfile,
                                                            s2: *const u8,
                                                            s2Len: c_int,
                                                            open: c_int,
                                                            gap: c_int)
                                                            -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_scan_profile_sse2_128_16(profile: *const ParasailProfile,
                                                            s2: *const u8,
                                                            s2Len: c_int,
                                                            open: c_int,
                                                            gap: c_int)
                                                            -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_scan_profile_sse2_128_8(profile: *const ParasailProfile,
                                                           s2: *const u8,
                                                           s2Len: c_int,
                                                           open: c_int,
                                                           gap: c_int)
                                                           -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_scan_profile_sse2_128_sat(profile: *const ParasailProfile,
                                                             s2: *const u8,
                                                             s2Len: c_int,
                                                             open: c_int,
                                                             gap: c_int)
                                                             -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_scan_profile_sse41_128_64(profile: *const ParasailProfile,
                                                             s2: *const u8,
                                                             s2Len: c_int,
                                                             open: c_int,
                                                             gap: c_int)
                                                             -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_scan_profile_sse41_128_32(profile: *const ParasailProfile,
                                                             s2: *const u8,
                                                             s2Len: c_int,
                                                             open: c_int,
                                                             gap: c_int)
                                                             -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_scan_profile_sse41_128_16(profile: *const ParasailProfile,
                                                             s2: *const u8,
                                                             s2Len: c_int,
                                                             open: c_int,
                                                             gap: c_int)
                                                             -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_scan_profile_sse41_128_8(profile: *const ParasailProfile,
                                                            s2: *const u8,
                                                            s2Len: c_int,
                                                            open: c_int,
                                                            gap: c_int)
                                                            -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_scan_profile_sse41_128_sat(profile: *const ParasailProfile,
                                                              s2: *const u8,
                                                              s2Len: c_int,
                                                              open: c_int,
                                                              gap: c_int)
                                                              -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_scan_profile_avx2_256_64(profile: *const ParasailProfile,
                                                            s2: *const u8,
                                                            s2Len: c_int,
                                                            open: c_int,
                                                            gap: c_int)
                                                            -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_scan_profile_avx2_256_32(profile: *const ParasailProfile,
                                                            s2: *const u8,
                                                            s2Len: c_int,
                                                            open: c_int,
                                                            gap: c_int)
                                                            -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_scan_profile_avx2_256_16(profile: *const ParasailProfile,
                                                            s2: *const u8,
                                                            s2Len: c_int,
                                                            open: c_int,
                                                            gap: c_int)
                                                            -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_scan_profile_avx2_256_8(profile: *const ParasailProfile,
                                                           s2: *const u8,
                                                           s2Len: c_int,
                                                           open: c_int,
                                                           gap: c_int)
                                                           -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_scan_profile_avx2_256_sat(profile: *const ParasailProfile,
                                                             s2: *const u8,
                                                             s2Len: c_int,
                                                             open: c_int,
                                                             gap: c_int)
                                                             -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_scan_profile_knc_512_32(profile: *const ParasailProfile,
                                                           s2: *const u8,
                                                           s2Len: c_int,
                                                           open: c_int,
                                                           gap: c_int)
                                                           -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_striped_profile_sse2_128_64(profile: *const ParasailProfile,
                                                               s2: *const u8,
                                                               s2Len: c_int,
                                                               open: c_int,
                                                               gap: c_int)
                                                               -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_striped_profile_sse2_128_32(profile: *const ParasailProfile,
                                                               s2: *const u8,
                                                               s2Len: c_int,
                                                               open: c_int,
                                                               gap: c_int)
                                                               -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_striped_profile_sse2_128_16(profile: *const ParasailProfile,
                                                               s2: *const u8,
                                                               s2Len: c_int,
                                                               open: c_int,
                                                               gap: c_int)
                                                               -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_striped_profile_sse2_128_8(profile: *const ParasailProfile,
                                                              s2: *const u8,
                                                              s2Len: c_int,
                                                              open: c_int,
                                                              gap: c_int)
                                                              -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_striped_profile_sse2_128_sat(profile: *const ParasailProfile,
                                                                s2: *const u8,
                                                                s2Len: c_int,
                                                                open: c_int,
                                                                gap: c_int)
                                                                -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_striped_profile_sse41_128_64(profile: *const ParasailProfile,
                                                                s2: *const u8,
                                                                s2Len: c_int,
                                                                open: c_int,
                                                                gap: c_int)
                                                                -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_striped_profile_sse41_128_32(profile: *const ParasailProfile,
                                                                s2: *const u8,
                                                                s2Len: c_int,
                                                                open: c_int,
                                                                gap: c_int)
                                                                -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_striped_profile_sse41_128_16(profile: *const ParasailProfile,
                                                                s2: *const u8,
                                                                s2Len: c_int,
                                                                open: c_int,
                                                                gap: c_int)
                                                                -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_striped_profile_sse41_128_8(profile: *const ParasailProfile,
                                                               s2: *const u8,
                                                               s2Len: c_int,
                                                               open: c_int,
                                                               gap: c_int)
                                                               -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_striped_profile_sse41_128_sat(profile: *const ParasailProfile,
                                                                 s2: *const u8,
                                                                 s2Len: c_int,
                                                                 open: c_int,
                                                                 gap: c_int)
                                                                 -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_striped_profile_avx2_256_64(profile: *const ParasailProfile,
                                                               s2: *const u8,
                                                               s2Len: c_int,
                                                               open: c_int,
                                                               gap: c_int)
                                                               -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_striped_profile_avx2_256_32(profile: *const ParasailProfile,
                                                               s2: *const u8,
                                                               s2Len: c_int,
                                                               open: c_int,
                                                               gap: c_int)
                                                               -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_striped_profile_avx2_256_16(profile: *const ParasailProfile,
                                                               s2: *const u8,
                                                               s2Len: c_int,
                                                               open: c_int,
                                                               gap: c_int)
                                                               -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_striped_profile_avx2_256_8(profile: *const ParasailProfile,
                                                              s2: *const u8,
                                                              s2Len: c_int,
                                                              open: c_int,
                                                              gap: c_int)
                                                              -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_striped_profile_avx2_256_sat(profile: *const ParasailProfile,
                                                                s2: *const u8,
                                                                s2Len: c_int,
                                                                open: c_int,
                                                                gap: c_int)
                                                                -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_striped_profile_knc_512_32(profile: *const ParasailProfile,
                                                              s2: *const u8,
                                                              s2Len: c_int,
                                                              open: c_int,
                                                              gap: c_int)
                                                              -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_scan_profile_sse2_128_64(profile: *const ParasailProfile,
                                                             s2: *const u8,
                                                             s2Len: c_int,
                                                             open: c_int,
                                                             gap: c_int)
                                                             -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_scan_profile_sse2_128_32(profile: *const ParasailProfile,
                                                             s2: *const u8,
                                                             s2Len: c_int,
                                                             open: c_int,
                                                             gap: c_int)
                                                             -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_scan_profile_sse2_128_16(profile: *const ParasailProfile,
                                                             s2: *const u8,
                                                             s2Len: c_int,
                                                             open: c_int,
                                                             gap: c_int)
                                                             -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_scan_profile_sse2_128_8(profile: *const ParasailProfile,
                                                            s2: *const u8,
                                                            s2Len: c_int,
                                                            open: c_int,
                                                            gap: c_int)
                                                            -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_scan_profile_sse2_128_sat(profile: *const ParasailProfile,
                                                              s2: *const u8,
                                                              s2Len: c_int,
                                                              open: c_int,
                                                              gap: c_int)
                                                              -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_scan_profile_sse41_128_64(profile: *const ParasailProfile,
                                                              s2: *const u8,
                                                              s2Len: c_int,
                                                              open: c_int,
                                                              gap: c_int)
                                                              -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_scan_profile_sse41_128_32(profile: *const ParasailProfile,
                                                              s2: *const u8,
                                                              s2Len: c_int,
                                                              open: c_int,
                                                              gap: c_int)
                                                              -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_scan_profile_sse41_128_16(profile: *const ParasailProfile,
                                                              s2: *const u8,
                                                              s2Len: c_int,
                                                              open: c_int,
                                                              gap: c_int)
                                                              -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_scan_profile_sse41_128_8(profile: *const ParasailProfile,
                                                             s2: *const u8,
                                                             s2Len: c_int,
                                                             open: c_int,
                                                             gap: c_int)
                                                             -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_scan_profile_sse41_128_sat(profile: *const ParasailProfile,
                                                               s2: *const u8,
                                                               s2Len: c_int,
                                                               open: c_int,
                                                               gap: c_int)
                                                               -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_scan_profile_avx2_256_64(profile: *const ParasailProfile,
                                                             s2: *const u8,
                                                             s2Len: c_int,
                                                             open: c_int,
                                                             gap: c_int)
                                                             -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_scan_profile_avx2_256_32(profile: *const ParasailProfile,
                                                             s2: *const u8,
                                                             s2Len: c_int,
                                                             open: c_int,
                                                             gap: c_int)
                                                             -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_scan_profile_avx2_256_16(profile: *const ParasailProfile,
                                                             s2: *const u8,
                                                             s2Len: c_int,
                                                             open: c_int,
                                                             gap: c_int)
                                                             -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_scan_profile_avx2_256_8(profile: *const ParasailProfile,
                                                            s2: *const u8,
                                                            s2Len: c_int,
                                                            open: c_int,
                                                            gap: c_int)
                                                            -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_scan_profile_avx2_256_sat(profile: *const ParasailProfile,
                                                              s2: *const u8,
                                                              s2Len: c_int,
                                                              open: c_int,
                                                              gap: c_int)
                                                              -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_scan_profile_knc_512_32(profile: *const ParasailProfile,
                                                            s2: *const u8,
                                                            s2Len: c_int,
                                                            open: c_int,
                                                            gap: c_int)
                                                            -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_striped_profile_sse2_128_64(profile: *const ParasailProfile,
                                                                s2: *const u8,
                                                                s2Len: c_int,
                                                                open: c_int,
                                                                gap: c_int)
                                                                -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_striped_profile_sse2_128_32(profile: *const ParasailProfile,
                                                                s2: *const u8,
                                                                s2Len: c_int,
                                                                open: c_int,
                                                                gap: c_int)
                                                                -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_striped_profile_sse2_128_16(profile: *const ParasailProfile,
                                                                s2: *const u8,
                                                                s2Len: c_int,
                                                                open: c_int,
                                                                gap: c_int)
                                                                -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_striped_profile_sse2_128_8(profile: *const ParasailProfile,
                                                               s2: *const u8,
                                                               s2Len: c_int,
                                                               open: c_int,
                                                               gap: c_int)
                                                               -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_striped_profile_sse2_128_sat(profile: *const ParasailProfile,
                                                                 s2: *const u8,
                                                                 s2Len: c_int,
                                                                 open: c_int,
                                                                 gap: c_int)
                                                                 -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_striped_profile_sse41_128_64(profile: *const ParasailProfile,
                                                                 s2: *const u8,
                                                                 s2Len: c_int,
                                                                 open: c_int,
                                                                 gap: c_int)
                                                                 -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_striped_profile_sse41_128_32(profile: *const ParasailProfile,
                                                                 s2: *const u8,
                                                                 s2Len: c_int,
                                                                 open: c_int,
                                                                 gap: c_int)
                                                                 -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_striped_profile_sse41_128_16(profile: *const ParasailProfile,
                                                                 s2: *const u8,
                                                                 s2Len: c_int,
                                                                 open: c_int,
                                                                 gap: c_int)
                                                                 -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_striped_profile_sse41_128_8(profile: *const ParasailProfile,
                                                                s2: *const u8,
                                                                s2Len: c_int,
                                                                open: c_int,
                                                                gap: c_int)
                                                                -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_striped_profile_sse41_128_sat(profile: *const ParasailProfile,
                                                                  s2: *const u8,
                                                                  s2Len: c_int,
                                                                  open: c_int,
                                                                  gap: c_int)
                                                                  -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_striped_profile_avx2_256_64(profile: *const ParasailProfile,
                                                                s2: *const u8,
                                                                s2Len: c_int,
                                                                open: c_int,
                                                                gap: c_int)
                                                                -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_striped_profile_avx2_256_32(profile: *const ParasailProfile,
                                                                s2: *const u8,
                                                                s2Len: c_int,
                                                                open: c_int,
                                                                gap: c_int)
                                                                -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_striped_profile_avx2_256_16(profile: *const ParasailProfile,
                                                                s2: *const u8,
                                                                s2Len: c_int,
                                                                open: c_int,
                                                                gap: c_int)
                                                                -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_striped_profile_avx2_256_8(profile: *const ParasailProfile,
                                                               s2: *const u8,
                                                               s2Len: c_int,
                                                               open: c_int,
                                                               gap: c_int)
                                                               -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_striped_profile_avx2_256_sat(profile: *const ParasailProfile,
                                                                 s2: *const u8,
                                                                 s2Len: c_int,
                                                                 open: c_int,
                                                                 gap: c_int)
                                                                 -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_striped_profile_knc_512_32(profile: *const ParasailProfile,
                                                               s2: *const u8,
                                                               s2Len: c_int,
                                                               open: c_int,
                                                               gap: c_int)
                                                               -> *mut ParasailResult;
    pub fn parasail_sw_blocked_sse41_128_32(s1: *const u8,
                                            s1Len: c_int,
                                            s2: *const u8,
                                            s2Len: c_int,
                                            open: c_int,
                                            gap: c_int,
                                            matrix: *const ParasailMatrix)
                                            -> *mut ParasailResult;
    pub fn parasail_sw_blocked_sse41_128_16(s1: *const u8,
                                            s1Len: c_int,
                                            s2: *const u8,
                                            s2Len: c_int,
                                            open: c_int,
                                            gap: c_int,
                                            matrix: *const ParasailMatrix)
                                            -> *mut ParasailResult;
    pub fn parasail_sw_table_blocked_sse41_128_32(s1: *const u8,
                                                  s1Len: c_int,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int,
                                                  matrix: *const ParasailMatrix)
                                                  -> *mut ParasailResult;
    pub fn parasail_sw_table_blocked_sse41_128_16(s1: *const u8,
                                                  s1Len: c_int,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int,
                                                  matrix: *const ParasailMatrix)
                                                  -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_blocked_sse41_128_32(s1: *const u8,
                                                   s1Len: c_int,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int,
                                                   matrix: *const ParasailMatrix)
                                                   -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_blocked_sse41_128_16(s1: *const u8,
                                                   s1Len: c_int,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int,
                                                   matrix: *const ParasailMatrix)
                                                   -> *mut ParasailResult;
    pub fn parasail_nw_scan_64(s1: *const u8,
                               s1Len: c_int,
                               s2: *const u8,
                               s2Len: c_int,
                               open: c_int,
                               gap: c_int,
                               matrix: *const ParasailMatrix)
                               -> *mut ParasailResult;
    pub fn parasail_nw_scan_32(s1: *const u8,
                               s1Len: c_int,
                               s2: *const u8,
                               s2Len: c_int,
                               open: c_int,
                               gap: c_int,
                               matrix: *const ParasailMatrix)
                               -> *mut ParasailResult;
    pub fn parasail_nw_scan_16(s1: *const u8,
                               s1Len: c_int,
                               s2: *const u8,
                               s2Len: c_int,
                               open: c_int,
                               gap: c_int,
                               matrix: *const ParasailMatrix)
                               -> *mut ParasailResult;
    pub fn parasail_nw_scan_8(s1: *const u8,
                              s1Len: c_int,
                              s2: *const u8,
                              s2Len: c_int,
                              open: c_int,
                              gap: c_int,
                              matrix: *const ParasailMatrix)
                              -> *mut ParasailResult;
    pub fn parasail_nw_scan_sat(s1: *const u8,
                                s1Len: c_int,
                                s2: *const u8,
                                s2Len: c_int,
                                open: c_int,
                                gap: c_int,
                                matrix: *const ParasailMatrix)
                                -> *mut ParasailResult;
    pub fn parasail_nw_striped_64(s1: *const u8,
                                  s1Len: c_int,
                                  s2: *const u8,
                                  s2Len: c_int,
                                  open: c_int,
                                  gap: c_int,
                                  matrix: *const ParasailMatrix)
                                  -> *mut ParasailResult;
    pub fn parasail_nw_striped_32(s1: *const u8,
                                  s1Len: c_int,
                                  s2: *const u8,
                                  s2Len: c_int,
                                  open: c_int,
                                  gap: c_int,
                                  matrix: *const ParasailMatrix)
                                  -> *mut ParasailResult;
    pub fn parasail_nw_striped_16(s1: *const u8,
                                  s1Len: c_int,
                                  s2: *const u8,
                                  s2Len: c_int,
                                  open: c_int,
                                  gap: c_int,
                                  matrix: *const ParasailMatrix)
                                  -> *mut ParasailResult;
    pub fn parasail_nw_striped_8(s1: *const u8,
                                 s1Len: c_int,
                                 s2: *const u8,
                                 s2Len: c_int,
                                 open: c_int,
                                 gap: c_int,
                                 matrix: *const ParasailMatrix)
                                 -> *mut ParasailResult;
    pub fn parasail_nw_striped_sat(s1: *const u8,
                                   s1Len: c_int,
                                   s2: *const u8,
                                   s2Len: c_int,
                                   open: c_int,
                                   gap: c_int,
                                   matrix: *const ParasailMatrix)
                                   -> *mut ParasailResult;
    pub fn parasail_nw_diag_64(s1: *const u8,
                               s1Len: c_int,
                               s2: *const u8,
                               s2Len: c_int,
                               open: c_int,
                               gap: c_int,
                               matrix: *const ParasailMatrix)
                               -> *mut ParasailResult;
    pub fn parasail_nw_diag_32(s1: *const u8,
                               s1Len: c_int,
                               s2: *const u8,
                               s2Len: c_int,
                               open: c_int,
                               gap: c_int,
                               matrix: *const ParasailMatrix)
                               -> *mut ParasailResult;
    pub fn parasail_nw_diag_16(s1: *const u8,
                               s1Len: c_int,
                               s2: *const u8,
                               s2Len: c_int,
                               open: c_int,
                               gap: c_int,
                               matrix: *const ParasailMatrix)
                               -> *mut ParasailResult;
    pub fn parasail_nw_diag_8(s1: *const u8,
                              s1Len: c_int,
                              s2: *const u8,
                              s2Len: c_int,
                              open: c_int,
                              gap: c_int,
                              matrix: *const ParasailMatrix)
                              -> *mut ParasailResult;
    pub fn parasail_nw_diag_sat(s1: *const u8,
                                s1Len: c_int,
                                s2: *const u8,
                                s2Len: c_int,
                                open: c_int,
                                gap: c_int,
                                matrix: *const ParasailMatrix)
                                -> *mut ParasailResult;
    pub fn parasail_nw_table_scan_64(s1: *const u8,
                                     s1Len: c_int,
                                     s2: *const u8,
                                     s2Len: c_int,
                                     open: c_int,
                                     gap: c_int,
                                     matrix: *const ParasailMatrix)
                                     -> *mut ParasailResult;
    pub fn parasail_nw_table_scan_32(s1: *const u8,
                                     s1Len: c_int,
                                     s2: *const u8,
                                     s2Len: c_int,
                                     open: c_int,
                                     gap: c_int,
                                     matrix: *const ParasailMatrix)
                                     -> *mut ParasailResult;
    pub fn parasail_nw_table_scan_16(s1: *const u8,
                                     s1Len: c_int,
                                     s2: *const u8,
                                     s2Len: c_int,
                                     open: c_int,
                                     gap: c_int,
                                     matrix: *const ParasailMatrix)
                                     -> *mut ParasailResult;
    pub fn parasail_nw_table_scan_8(s1: *const u8,
                                    s1Len: c_int,
                                    s2: *const u8,
                                    s2Len: c_int,
                                    open: c_int,
                                    gap: c_int,
                                    matrix: *const ParasailMatrix)
                                    -> *mut ParasailResult;
    pub fn parasail_nw_table_scan_sat(s1: *const u8,
                                      s1Len: c_int,
                                      s2: *const u8,
                                      s2Len: c_int,
                                      open: c_int,
                                      gap: c_int,
                                      matrix: *const ParasailMatrix)
                                      -> *mut ParasailResult;
    pub fn parasail_nw_table_striped_64(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_nw_table_striped_32(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_nw_table_striped_16(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_nw_table_striped_8(s1: *const u8,
                                       s1Len: c_int,
                                       s2: *const u8,
                                       s2Len: c_int,
                                       open: c_int,
                                       gap: c_int,
                                       matrix: *const ParasailMatrix)
                                       -> *mut ParasailResult;
    pub fn parasail_nw_table_striped_sat(s1: *const u8,
                                         s1Len: c_int,
                                         s2: *const u8,
                                         s2Len: c_int,
                                         open: c_int,
                                         gap: c_int,
                                         matrix: *const ParasailMatrix)
                                         -> *mut ParasailResult;
    pub fn parasail_nw_table_diag_64(s1: *const u8,
                                     s1Len: c_int,
                                     s2: *const u8,
                                     s2Len: c_int,
                                     open: c_int,
                                     gap: c_int,
                                     matrix: *const ParasailMatrix)
                                     -> *mut ParasailResult;
    pub fn parasail_nw_table_diag_32(s1: *const u8,
                                     s1Len: c_int,
                                     s2: *const u8,
                                     s2Len: c_int,
                                     open: c_int,
                                     gap: c_int,
                                     matrix: *const ParasailMatrix)
                                     -> *mut ParasailResult;
    pub fn parasail_nw_table_diag_16(s1: *const u8,
                                     s1Len: c_int,
                                     s2: *const u8,
                                     s2Len: c_int,
                                     open: c_int,
                                     gap: c_int,
                                     matrix: *const ParasailMatrix)
                                     -> *mut ParasailResult;
    pub fn parasail_nw_table_diag_8(s1: *const u8,
                                    s1Len: c_int,
                                    s2: *const u8,
                                    s2Len: c_int,
                                    open: c_int,
                                    gap: c_int,
                                    matrix: *const ParasailMatrix)
                                    -> *mut ParasailResult;
    pub fn parasail_nw_table_diag_sat(s1: *const u8,
                                      s1Len: c_int,
                                      s2: *const u8,
                                      s2Len: c_int,
                                      open: c_int,
                                      gap: c_int,
                                      matrix: *const ParasailMatrix)
                                      -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_scan_64(s1: *const u8,
                                      s1Len: c_int,
                                      s2: *const u8,
                                      s2Len: c_int,
                                      open: c_int,
                                      gap: c_int,
                                      matrix: *const ParasailMatrix)
                                      -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_scan_32(s1: *const u8,
                                      s1Len: c_int,
                                      s2: *const u8,
                                      s2Len: c_int,
                                      open: c_int,
                                      gap: c_int,
                                      matrix: *const ParasailMatrix)
                                      -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_scan_16(s1: *const u8,
                                      s1Len: c_int,
                                      s2: *const u8,
                                      s2Len: c_int,
                                      open: c_int,
                                      gap: c_int,
                                      matrix: *const ParasailMatrix)
                                      -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_scan_8(s1: *const u8,
                                     s1Len: c_int,
                                     s2: *const u8,
                                     s2Len: c_int,
                                     open: c_int,
                                     gap: c_int,
                                     matrix: *const ParasailMatrix)
                                     -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_scan_sat(s1: *const u8,
                                       s1Len: c_int,
                                       s2: *const u8,
                                       s2Len: c_int,
                                       open: c_int,
                                       gap: c_int,
                                       matrix: *const ParasailMatrix)
                                       -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_striped_64(s1: *const u8,
                                         s1Len: c_int,
                                         s2: *const u8,
                                         s2Len: c_int,
                                         open: c_int,
                                         gap: c_int,
                                         matrix: *const ParasailMatrix)
                                         -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_striped_32(s1: *const u8,
                                         s1Len: c_int,
                                         s2: *const u8,
                                         s2Len: c_int,
                                         open: c_int,
                                         gap: c_int,
                                         matrix: *const ParasailMatrix)
                                         -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_striped_16(s1: *const u8,
                                         s1Len: c_int,
                                         s2: *const u8,
                                         s2Len: c_int,
                                         open: c_int,
                                         gap: c_int,
                                         matrix: *const ParasailMatrix)
                                         -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_striped_8(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_striped_sat(s1: *const u8,
                                          s1Len: c_int,
                                          s2: *const u8,
                                          s2Len: c_int,
                                          open: c_int,
                                          gap: c_int,
                                          matrix: *const ParasailMatrix)
                                          -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_diag_64(s1: *const u8,
                                      s1Len: c_int,
                                      s2: *const u8,
                                      s2Len: c_int,
                                      open: c_int,
                                      gap: c_int,
                                      matrix: *const ParasailMatrix)
                                      -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_diag_32(s1: *const u8,
                                      s1Len: c_int,
                                      s2: *const u8,
                                      s2Len: c_int,
                                      open: c_int,
                                      gap: c_int,
                                      matrix: *const ParasailMatrix)
                                      -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_diag_16(s1: *const u8,
                                      s1Len: c_int,
                                      s2: *const u8,
                                      s2Len: c_int,
                                      open: c_int,
                                      gap: c_int,
                                      matrix: *const ParasailMatrix)
                                      -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_diag_8(s1: *const u8,
                                     s1Len: c_int,
                                     s2: *const u8,
                                     s2Len: c_int,
                                     open: c_int,
                                     gap: c_int,
                                     matrix: *const ParasailMatrix)
                                     -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_diag_sat(s1: *const u8,
                                       s1Len: c_int,
                                       s2: *const u8,
                                       s2Len: c_int,
                                       open: c_int,
                                       gap: c_int,
                                       matrix: *const ParasailMatrix)
                                       -> *mut ParasailResult;
    pub fn parasail_nw_stats_scan_64(s1: *const u8,
                                     s1Len: c_int,
                                     s2: *const u8,
                                     s2Len: c_int,
                                     open: c_int,
                                     gap: c_int,
                                     matrix: *const ParasailMatrix)
                                     -> *mut ParasailResult;
    pub fn parasail_nw_stats_scan_32(s1: *const u8,
                                     s1Len: c_int,
                                     s2: *const u8,
                                     s2Len: c_int,
                                     open: c_int,
                                     gap: c_int,
                                     matrix: *const ParasailMatrix)
                                     -> *mut ParasailResult;
    pub fn parasail_nw_stats_scan_16(s1: *const u8,
                                     s1Len: c_int,
                                     s2: *const u8,
                                     s2Len: c_int,
                                     open: c_int,
                                     gap: c_int,
                                     matrix: *const ParasailMatrix)
                                     -> *mut ParasailResult;
    pub fn parasail_nw_stats_scan_8(s1: *const u8,
                                    s1Len: c_int,
                                    s2: *const u8,
                                    s2Len: c_int,
                                    open: c_int,
                                    gap: c_int,
                                    matrix: *const ParasailMatrix)
                                    -> *mut ParasailResult;
    pub fn parasail_nw_stats_scan_sat(s1: *const u8,
                                      s1Len: c_int,
                                      s2: *const u8,
                                      s2Len: c_int,
                                      open: c_int,
                                      gap: c_int,
                                      matrix: *const ParasailMatrix)
                                      -> *mut ParasailResult;
    pub fn parasail_nw_stats_striped_64(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_nw_stats_striped_32(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_nw_stats_striped_16(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_nw_stats_striped_8(s1: *const u8,
                                       s1Len: c_int,
                                       s2: *const u8,
                                       s2Len: c_int,
                                       open: c_int,
                                       gap: c_int,
                                       matrix: *const ParasailMatrix)
                                       -> *mut ParasailResult;
    pub fn parasail_nw_stats_striped_sat(s1: *const u8,
                                         s1Len: c_int,
                                         s2: *const u8,
                                         s2Len: c_int,
                                         open: c_int,
                                         gap: c_int,
                                         matrix: *const ParasailMatrix)
                                         -> *mut ParasailResult;
    pub fn parasail_nw_stats_diag_64(s1: *const u8,
                                     s1Len: c_int,
                                     s2: *const u8,
                                     s2Len: c_int,
                                     open: c_int,
                                     gap: c_int,
                                     matrix: *const ParasailMatrix)
                                     -> *mut ParasailResult;
    pub fn parasail_nw_stats_diag_32(s1: *const u8,
                                     s1Len: c_int,
                                     s2: *const u8,
                                     s2Len: c_int,
                                     open: c_int,
                                     gap: c_int,
                                     matrix: *const ParasailMatrix)
                                     -> *mut ParasailResult;
    pub fn parasail_nw_stats_diag_16(s1: *const u8,
                                     s1Len: c_int,
                                     s2: *const u8,
                                     s2Len: c_int,
                                     open: c_int,
                                     gap: c_int,
                                     matrix: *const ParasailMatrix)
                                     -> *mut ParasailResult;
    pub fn parasail_nw_stats_diag_8(s1: *const u8,
                                    s1Len: c_int,
                                    s2: *const u8,
                                    s2Len: c_int,
                                    open: c_int,
                                    gap: c_int,
                                    matrix: *const ParasailMatrix)
                                    -> *mut ParasailResult;
    pub fn parasail_nw_stats_diag_sat(s1: *const u8,
                                      s1Len: c_int,
                                      s2: *const u8,
                                      s2Len: c_int,
                                      open: c_int,
                                      gap: c_int,
                                      matrix: *const ParasailMatrix)
                                      -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_scan_64(s1: *const u8,
                                           s1Len: c_int,
                                           s2: *const u8,
                                           s2Len: c_int,
                                           open: c_int,
                                           gap: c_int,
                                           matrix: *const ParasailMatrix)
                                           -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_scan_32(s1: *const u8,
                                           s1Len: c_int,
                                           s2: *const u8,
                                           s2Len: c_int,
                                           open: c_int,
                                           gap: c_int,
                                           matrix: *const ParasailMatrix)
                                           -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_scan_16(s1: *const u8,
                                           s1Len: c_int,
                                           s2: *const u8,
                                           s2Len: c_int,
                                           open: c_int,
                                           gap: c_int,
                                           matrix: *const ParasailMatrix)
                                           -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_scan_8(s1: *const u8,
                                          s1Len: c_int,
                                          s2: *const u8,
                                          s2Len: c_int,
                                          open: c_int,
                                          gap: c_int,
                                          matrix: *const ParasailMatrix)
                                          -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_scan_sat(s1: *const u8,
                                            s1Len: c_int,
                                            s2: *const u8,
                                            s2Len: c_int,
                                            open: c_int,
                                            gap: c_int,
                                            matrix: *const ParasailMatrix)
                                            -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_striped_64(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_striped_32(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_striped_16(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_striped_8(s1: *const u8,
                                             s1Len: c_int,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int,
                                             matrix: *const ParasailMatrix)
                                             -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_striped_sat(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_diag_64(s1: *const u8,
                                           s1Len: c_int,
                                           s2: *const u8,
                                           s2Len: c_int,
                                           open: c_int,
                                           gap: c_int,
                                           matrix: *const ParasailMatrix)
                                           -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_diag_32(s1: *const u8,
                                           s1Len: c_int,
                                           s2: *const u8,
                                           s2Len: c_int,
                                           open: c_int,
                                           gap: c_int,
                                           matrix: *const ParasailMatrix)
                                           -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_diag_16(s1: *const u8,
                                           s1Len: c_int,
                                           s2: *const u8,
                                           s2Len: c_int,
                                           open: c_int,
                                           gap: c_int,
                                           matrix: *const ParasailMatrix)
                                           -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_diag_8(s1: *const u8,
                                          s1Len: c_int,
                                          s2: *const u8,
                                          s2Len: c_int,
                                          open: c_int,
                                          gap: c_int,
                                          matrix: *const ParasailMatrix)
                                          -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_diag_sat(s1: *const u8,
                                            s1Len: c_int,
                                            s2: *const u8,
                                            s2Len: c_int,
                                            open: c_int,
                                            gap: c_int,
                                            matrix: *const ParasailMatrix)
                                            -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_scan_64(s1: *const u8,
                                            s1Len: c_int,
                                            s2: *const u8,
                                            s2Len: c_int,
                                            open: c_int,
                                            gap: c_int,
                                            matrix: *const ParasailMatrix)
                                            -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_scan_32(s1: *const u8,
                                            s1Len: c_int,
                                            s2: *const u8,
                                            s2Len: c_int,
                                            open: c_int,
                                            gap: c_int,
                                            matrix: *const ParasailMatrix)
                                            -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_scan_16(s1: *const u8,
                                            s1Len: c_int,
                                            s2: *const u8,
                                            s2Len: c_int,
                                            open: c_int,
                                            gap: c_int,
                                            matrix: *const ParasailMatrix)
                                            -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_scan_8(s1: *const u8,
                                           s1Len: c_int,
                                           s2: *const u8,
                                           s2Len: c_int,
                                           open: c_int,
                                           gap: c_int,
                                           matrix: *const ParasailMatrix)
                                           -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_scan_sat(s1: *const u8,
                                             s1Len: c_int,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int,
                                             matrix: *const ParasailMatrix)
                                             -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_striped_64(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_striped_32(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_striped_16(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_striped_8(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_striped_sat(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_diag_64(s1: *const u8,
                                            s1Len: c_int,
                                            s2: *const u8,
                                            s2Len: c_int,
                                            open: c_int,
                                            gap: c_int,
                                            matrix: *const ParasailMatrix)
                                            -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_diag_32(s1: *const u8,
                                            s1Len: c_int,
                                            s2: *const u8,
                                            s2Len: c_int,
                                            open: c_int,
                                            gap: c_int,
                                            matrix: *const ParasailMatrix)
                                            -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_diag_16(s1: *const u8,
                                            s1Len: c_int,
                                            s2: *const u8,
                                            s2Len: c_int,
                                            open: c_int,
                                            gap: c_int,
                                            matrix: *const ParasailMatrix)
                                            -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_diag_8(s1: *const u8,
                                           s1Len: c_int,
                                           s2: *const u8,
                                           s2Len: c_int,
                                           open: c_int,
                                           gap: c_int,
                                           matrix: *const ParasailMatrix)
                                           -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_diag_sat(s1: *const u8,
                                             s1Len: c_int,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int,
                                             matrix: *const ParasailMatrix)
                                             -> *mut ParasailResult;
    pub fn parasail_sg_scan_64(s1: *const u8,
                               s1Len: c_int,
                               s2: *const u8,
                               s2Len: c_int,
                               open: c_int,
                               gap: c_int,
                               matrix: *const ParasailMatrix)
                               -> *mut ParasailResult;
    pub fn parasail_sg_scan_32(s1: *const u8,
                               s1Len: c_int,
                               s2: *const u8,
                               s2Len: c_int,
                               open: c_int,
                               gap: c_int,
                               matrix: *const ParasailMatrix)
                               -> *mut ParasailResult;
    pub fn parasail_sg_scan_16(s1: *const u8,
                               s1Len: c_int,
                               s2: *const u8,
                               s2Len: c_int,
                               open: c_int,
                               gap: c_int,
                               matrix: *const ParasailMatrix)
                               -> *mut ParasailResult;
    pub fn parasail_sg_scan_8(s1: *const u8,
                              s1Len: c_int,
                              s2: *const u8,
                              s2Len: c_int,
                              open: c_int,
                              gap: c_int,
                              matrix: *const ParasailMatrix)
                              -> *mut ParasailResult;
    pub fn parasail_sg_scan_sat(s1: *const u8,
                                s1Len: c_int,
                                s2: *const u8,
                                s2Len: c_int,
                                open: c_int,
                                gap: c_int,
                                matrix: *const ParasailMatrix)
                                -> *mut ParasailResult;
    pub fn parasail_sg_striped_64(s1: *const u8,
                                  s1Len: c_int,
                                  s2: *const u8,
                                  s2Len: c_int,
                                  open: c_int,
                                  gap: c_int,
                                  matrix: *const ParasailMatrix)
                                  -> *mut ParasailResult;
    pub fn parasail_sg_striped_32(s1: *const u8,
                                  s1Len: c_int,
                                  s2: *const u8,
                                  s2Len: c_int,
                                  open: c_int,
                                  gap: c_int,
                                  matrix: *const ParasailMatrix)
                                  -> *mut ParasailResult;
    pub fn parasail_sg_striped_16(s1: *const u8,
                                  s1Len: c_int,
                                  s2: *const u8,
                                  s2Len: c_int,
                                  open: c_int,
                                  gap: c_int,
                                  matrix: *const ParasailMatrix)
                                  -> *mut ParasailResult;
    pub fn parasail_sg_striped_8(s1: *const u8,
                                 s1Len: c_int,
                                 s2: *const u8,
                                 s2Len: c_int,
                                 open: c_int,
                                 gap: c_int,
                                 matrix: *const ParasailMatrix)
                                 -> *mut ParasailResult;
    pub fn parasail_sg_striped_sat(s1: *const u8,
                                   s1Len: c_int,
                                   s2: *const u8,
                                   s2Len: c_int,
                                   open: c_int,
                                   gap: c_int,
                                   matrix: *const ParasailMatrix)
                                   -> *mut ParasailResult;
    pub fn parasail_sg_diag_64(s1: *const u8,
                               s1Len: c_int,
                               s2: *const u8,
                               s2Len: c_int,
                               open: c_int,
                               gap: c_int,
                               matrix: *const ParasailMatrix)
                               -> *mut ParasailResult;
    pub fn parasail_sg_diag_32(s1: *const u8,
                               s1Len: c_int,
                               s2: *const u8,
                               s2Len: c_int,
                               open: c_int,
                               gap: c_int,
                               matrix: *const ParasailMatrix)
                               -> *mut ParasailResult;
    pub fn parasail_sg_diag_16(s1: *const u8,
                               s1Len: c_int,
                               s2: *const u8,
                               s2Len: c_int,
                               open: c_int,
                               gap: c_int,
                               matrix: *const ParasailMatrix)
                               -> *mut ParasailResult;
    pub fn parasail_sg_diag_8(s1: *const u8,
                              s1Len: c_int,
                              s2: *const u8,
                              s2Len: c_int,
                              open: c_int,
                              gap: c_int,
                              matrix: *const ParasailMatrix)
                              -> *mut ParasailResult;
    pub fn parasail_sg_diag_sat(s1: *const u8,
                                s1Len: c_int,
                                s2: *const u8,
                                s2Len: c_int,
                                open: c_int,
                                gap: c_int,
                                matrix: *const ParasailMatrix)
                                -> *mut ParasailResult;
    pub fn parasail_sg_table_scan_64(s1: *const u8,
                                     s1Len: c_int,
                                     s2: *const u8,
                                     s2Len: c_int,
                                     open: c_int,
                                     gap: c_int,
                                     matrix: *const ParasailMatrix)
                                     -> *mut ParasailResult;
    pub fn parasail_sg_table_scan_32(s1: *const u8,
                                     s1Len: c_int,
                                     s2: *const u8,
                                     s2Len: c_int,
                                     open: c_int,
                                     gap: c_int,
                                     matrix: *const ParasailMatrix)
                                     -> *mut ParasailResult;
    pub fn parasail_sg_table_scan_16(s1: *const u8,
                                     s1Len: c_int,
                                     s2: *const u8,
                                     s2Len: c_int,
                                     open: c_int,
                                     gap: c_int,
                                     matrix: *const ParasailMatrix)
                                     -> *mut ParasailResult;
    pub fn parasail_sg_table_scan_8(s1: *const u8,
                                    s1Len: c_int,
                                    s2: *const u8,
                                    s2Len: c_int,
                                    open: c_int,
                                    gap: c_int,
                                    matrix: *const ParasailMatrix)
                                    -> *mut ParasailResult;
    pub fn parasail_sg_table_scan_sat(s1: *const u8,
                                      s1Len: c_int,
                                      s2: *const u8,
                                      s2Len: c_int,
                                      open: c_int,
                                      gap: c_int,
                                      matrix: *const ParasailMatrix)
                                      -> *mut ParasailResult;
    pub fn parasail_sg_table_striped_64(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_sg_table_striped_32(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_sg_table_striped_16(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_sg_table_striped_8(s1: *const u8,
                                       s1Len: c_int,
                                       s2: *const u8,
                                       s2Len: c_int,
                                       open: c_int,
                                       gap: c_int,
                                       matrix: *const ParasailMatrix)
                                       -> *mut ParasailResult;
    pub fn parasail_sg_table_striped_sat(s1: *const u8,
                                         s1Len: c_int,
                                         s2: *const u8,
                                         s2Len: c_int,
                                         open: c_int,
                                         gap: c_int,
                                         matrix: *const ParasailMatrix)
                                         -> *mut ParasailResult;
    pub fn parasail_sg_table_diag_64(s1: *const u8,
                                     s1Len: c_int,
                                     s2: *const u8,
                                     s2Len: c_int,
                                     open: c_int,
                                     gap: c_int,
                                     matrix: *const ParasailMatrix)
                                     -> *mut ParasailResult;
    pub fn parasail_sg_table_diag_32(s1: *const u8,
                                     s1Len: c_int,
                                     s2: *const u8,
                                     s2Len: c_int,
                                     open: c_int,
                                     gap: c_int,
                                     matrix: *const ParasailMatrix)
                                     -> *mut ParasailResult;
    pub fn parasail_sg_table_diag_16(s1: *const u8,
                                     s1Len: c_int,
                                     s2: *const u8,
                                     s2Len: c_int,
                                     open: c_int,
                                     gap: c_int,
                                     matrix: *const ParasailMatrix)
                                     -> *mut ParasailResult;
    pub fn parasail_sg_table_diag_8(s1: *const u8,
                                    s1Len: c_int,
                                    s2: *const u8,
                                    s2Len: c_int,
                                    open: c_int,
                                    gap: c_int,
                                    matrix: *const ParasailMatrix)
                                    -> *mut ParasailResult;
    pub fn parasail_sg_table_diag_sat(s1: *const u8,
                                      s1Len: c_int,
                                      s2: *const u8,
                                      s2Len: c_int,
                                      open: c_int,
                                      gap: c_int,
                                      matrix: *const ParasailMatrix)
                                      -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_scan_64(s1: *const u8,
                                      s1Len: c_int,
                                      s2: *const u8,
                                      s2Len: c_int,
                                      open: c_int,
                                      gap: c_int,
                                      matrix: *const ParasailMatrix)
                                      -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_scan_32(s1: *const u8,
                                      s1Len: c_int,
                                      s2: *const u8,
                                      s2Len: c_int,
                                      open: c_int,
                                      gap: c_int,
                                      matrix: *const ParasailMatrix)
                                      -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_scan_16(s1: *const u8,
                                      s1Len: c_int,
                                      s2: *const u8,
                                      s2Len: c_int,
                                      open: c_int,
                                      gap: c_int,
                                      matrix: *const ParasailMatrix)
                                      -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_scan_8(s1: *const u8,
                                     s1Len: c_int,
                                     s2: *const u8,
                                     s2Len: c_int,
                                     open: c_int,
                                     gap: c_int,
                                     matrix: *const ParasailMatrix)
                                     -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_scan_sat(s1: *const u8,
                                       s1Len: c_int,
                                       s2: *const u8,
                                       s2Len: c_int,
                                       open: c_int,
                                       gap: c_int,
                                       matrix: *const ParasailMatrix)
                                       -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_striped_64(s1: *const u8,
                                         s1Len: c_int,
                                         s2: *const u8,
                                         s2Len: c_int,
                                         open: c_int,
                                         gap: c_int,
                                         matrix: *const ParasailMatrix)
                                         -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_striped_32(s1: *const u8,
                                         s1Len: c_int,
                                         s2: *const u8,
                                         s2Len: c_int,
                                         open: c_int,
                                         gap: c_int,
                                         matrix: *const ParasailMatrix)
                                         -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_striped_16(s1: *const u8,
                                         s1Len: c_int,
                                         s2: *const u8,
                                         s2Len: c_int,
                                         open: c_int,
                                         gap: c_int,
                                         matrix: *const ParasailMatrix)
                                         -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_striped_8(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_striped_sat(s1: *const u8,
                                          s1Len: c_int,
                                          s2: *const u8,
                                          s2Len: c_int,
                                          open: c_int,
                                          gap: c_int,
                                          matrix: *const ParasailMatrix)
                                          -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_diag_64(s1: *const u8,
                                      s1Len: c_int,
                                      s2: *const u8,
                                      s2Len: c_int,
                                      open: c_int,
                                      gap: c_int,
                                      matrix: *const ParasailMatrix)
                                      -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_diag_32(s1: *const u8,
                                      s1Len: c_int,
                                      s2: *const u8,
                                      s2Len: c_int,
                                      open: c_int,
                                      gap: c_int,
                                      matrix: *const ParasailMatrix)
                                      -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_diag_16(s1: *const u8,
                                      s1Len: c_int,
                                      s2: *const u8,
                                      s2Len: c_int,
                                      open: c_int,
                                      gap: c_int,
                                      matrix: *const ParasailMatrix)
                                      -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_diag_8(s1: *const u8,
                                     s1Len: c_int,
                                     s2: *const u8,
                                     s2Len: c_int,
                                     open: c_int,
                                     gap: c_int,
                                     matrix: *const ParasailMatrix)
                                     -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_diag_sat(s1: *const u8,
                                       s1Len: c_int,
                                       s2: *const u8,
                                       s2Len: c_int,
                                       open: c_int,
                                       gap: c_int,
                                       matrix: *const ParasailMatrix)
                                       -> *mut ParasailResult;
    pub fn parasail_sg_stats_scan_64(s1: *const u8,
                                     s1Len: c_int,
                                     s2: *const u8,
                                     s2Len: c_int,
                                     open: c_int,
                                     gap: c_int,
                                     matrix: *const ParasailMatrix)
                                     -> *mut ParasailResult;
    pub fn parasail_sg_stats_scan_32(s1: *const u8,
                                     s1Len: c_int,
                                     s2: *const u8,
                                     s2Len: c_int,
                                     open: c_int,
                                     gap: c_int,
                                     matrix: *const ParasailMatrix)
                                     -> *mut ParasailResult;
    pub fn parasail_sg_stats_scan_16(s1: *const u8,
                                     s1Len: c_int,
                                     s2: *const u8,
                                     s2Len: c_int,
                                     open: c_int,
                                     gap: c_int,
                                     matrix: *const ParasailMatrix)
                                     -> *mut ParasailResult;
    pub fn parasail_sg_stats_scan_8(s1: *const u8,
                                    s1Len: c_int,
                                    s2: *const u8,
                                    s2Len: c_int,
                                    open: c_int,
                                    gap: c_int,
                                    matrix: *const ParasailMatrix)
                                    -> *mut ParasailResult;
    pub fn parasail_sg_stats_scan_sat(s1: *const u8,
                                      s1Len: c_int,
                                      s2: *const u8,
                                      s2Len: c_int,
                                      open: c_int,
                                      gap: c_int,
                                      matrix: *const ParasailMatrix)
                                      -> *mut ParasailResult;
    pub fn parasail_sg_stats_striped_64(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_sg_stats_striped_32(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_sg_stats_striped_16(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_sg_stats_striped_8(s1: *const u8,
                                       s1Len: c_int,
                                       s2: *const u8,
                                       s2Len: c_int,
                                       open: c_int,
                                       gap: c_int,
                                       matrix: *const ParasailMatrix)
                                       -> *mut ParasailResult;
    pub fn parasail_sg_stats_striped_sat(s1: *const u8,
                                         s1Len: c_int,
                                         s2: *const u8,
                                         s2Len: c_int,
                                         open: c_int,
                                         gap: c_int,
                                         matrix: *const ParasailMatrix)
                                         -> *mut ParasailResult;
    pub fn parasail_sg_stats_diag_64(s1: *const u8,
                                     s1Len: c_int,
                                     s2: *const u8,
                                     s2Len: c_int,
                                     open: c_int,
                                     gap: c_int,
                                     matrix: *const ParasailMatrix)
                                     -> *mut ParasailResult;
    pub fn parasail_sg_stats_diag_32(s1: *const u8,
                                     s1Len: c_int,
                                     s2: *const u8,
                                     s2Len: c_int,
                                     open: c_int,
                                     gap: c_int,
                                     matrix: *const ParasailMatrix)
                                     -> *mut ParasailResult;
    pub fn parasail_sg_stats_diag_16(s1: *const u8,
                                     s1Len: c_int,
                                     s2: *const u8,
                                     s2Len: c_int,
                                     open: c_int,
                                     gap: c_int,
                                     matrix: *const ParasailMatrix)
                                     -> *mut ParasailResult;
    pub fn parasail_sg_stats_diag_8(s1: *const u8,
                                    s1Len: c_int,
                                    s2: *const u8,
                                    s2Len: c_int,
                                    open: c_int,
                                    gap: c_int,
                                    matrix: *const ParasailMatrix)
                                    -> *mut ParasailResult;
    pub fn parasail_sg_stats_diag_sat(s1: *const u8,
                                      s1Len: c_int,
                                      s2: *const u8,
                                      s2Len: c_int,
                                      open: c_int,
                                      gap: c_int,
                                      matrix: *const ParasailMatrix)
                                      -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_scan_64(s1: *const u8,
                                           s1Len: c_int,
                                           s2: *const u8,
                                           s2Len: c_int,
                                           open: c_int,
                                           gap: c_int,
                                           matrix: *const ParasailMatrix)
                                           -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_scan_32(s1: *const u8,
                                           s1Len: c_int,
                                           s2: *const u8,
                                           s2Len: c_int,
                                           open: c_int,
                                           gap: c_int,
                                           matrix: *const ParasailMatrix)
                                           -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_scan_16(s1: *const u8,
                                           s1Len: c_int,
                                           s2: *const u8,
                                           s2Len: c_int,
                                           open: c_int,
                                           gap: c_int,
                                           matrix: *const ParasailMatrix)
                                           -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_scan_8(s1: *const u8,
                                          s1Len: c_int,
                                          s2: *const u8,
                                          s2Len: c_int,
                                          open: c_int,
                                          gap: c_int,
                                          matrix: *const ParasailMatrix)
                                          -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_scan_sat(s1: *const u8,
                                            s1Len: c_int,
                                            s2: *const u8,
                                            s2Len: c_int,
                                            open: c_int,
                                            gap: c_int,
                                            matrix: *const ParasailMatrix)
                                            -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_striped_64(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_striped_32(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_striped_16(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_striped_8(s1: *const u8,
                                             s1Len: c_int,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int,
                                             matrix: *const ParasailMatrix)
                                             -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_striped_sat(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_diag_64(s1: *const u8,
                                           s1Len: c_int,
                                           s2: *const u8,
                                           s2Len: c_int,
                                           open: c_int,
                                           gap: c_int,
                                           matrix: *const ParasailMatrix)
                                           -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_diag_32(s1: *const u8,
                                           s1Len: c_int,
                                           s2: *const u8,
                                           s2Len: c_int,
                                           open: c_int,
                                           gap: c_int,
                                           matrix: *const ParasailMatrix)
                                           -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_diag_16(s1: *const u8,
                                           s1Len: c_int,
                                           s2: *const u8,
                                           s2Len: c_int,
                                           open: c_int,
                                           gap: c_int,
                                           matrix: *const ParasailMatrix)
                                           -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_diag_8(s1: *const u8,
                                          s1Len: c_int,
                                          s2: *const u8,
                                          s2Len: c_int,
                                          open: c_int,
                                          gap: c_int,
                                          matrix: *const ParasailMatrix)
                                          -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_diag_sat(s1: *const u8,
                                            s1Len: c_int,
                                            s2: *const u8,
                                            s2Len: c_int,
                                            open: c_int,
                                            gap: c_int,
                                            matrix: *const ParasailMatrix)
                                            -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_scan_64(s1: *const u8,
                                            s1Len: c_int,
                                            s2: *const u8,
                                            s2Len: c_int,
                                            open: c_int,
                                            gap: c_int,
                                            matrix: *const ParasailMatrix)
                                            -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_scan_32(s1: *const u8,
                                            s1Len: c_int,
                                            s2: *const u8,
                                            s2Len: c_int,
                                            open: c_int,
                                            gap: c_int,
                                            matrix: *const ParasailMatrix)
                                            -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_scan_16(s1: *const u8,
                                            s1Len: c_int,
                                            s2: *const u8,
                                            s2Len: c_int,
                                            open: c_int,
                                            gap: c_int,
                                            matrix: *const ParasailMatrix)
                                            -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_scan_8(s1: *const u8,
                                           s1Len: c_int,
                                           s2: *const u8,
                                           s2Len: c_int,
                                           open: c_int,
                                           gap: c_int,
                                           matrix: *const ParasailMatrix)
                                           -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_scan_sat(s1: *const u8,
                                             s1Len: c_int,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int,
                                             matrix: *const ParasailMatrix)
                                             -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_striped_64(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_striped_32(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_striped_16(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_striped_8(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_striped_sat(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_diag_64(s1: *const u8,
                                            s1Len: c_int,
                                            s2: *const u8,
                                            s2Len: c_int,
                                            open: c_int,
                                            gap: c_int,
                                            matrix: *const ParasailMatrix)
                                            -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_diag_32(s1: *const u8,
                                            s1Len: c_int,
                                            s2: *const u8,
                                            s2Len: c_int,
                                            open: c_int,
                                            gap: c_int,
                                            matrix: *const ParasailMatrix)
                                            -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_diag_16(s1: *const u8,
                                            s1Len: c_int,
                                            s2: *const u8,
                                            s2Len: c_int,
                                            open: c_int,
                                            gap: c_int,
                                            matrix: *const ParasailMatrix)
                                            -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_diag_8(s1: *const u8,
                                           s1Len: c_int,
                                           s2: *const u8,
                                           s2Len: c_int,
                                           open: c_int,
                                           gap: c_int,
                                           matrix: *const ParasailMatrix)
                                           -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_diag_sat(s1: *const u8,
                                             s1Len: c_int,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int,
                                             matrix: *const ParasailMatrix)
                                             -> *mut ParasailResult;
    pub fn parasail_sw_scan_64(s1: *const u8,
                               s1Len: c_int,
                               s2: *const u8,
                               s2Len: c_int,
                               open: c_int,
                               gap: c_int,
                               matrix: *const ParasailMatrix)
                               -> *mut ParasailResult;
    pub fn parasail_sw_scan_32(s1: *const u8,
                               s1Len: c_int,
                               s2: *const u8,
                               s2Len: c_int,
                               open: c_int,
                               gap: c_int,
                               matrix: *const ParasailMatrix)
                               -> *mut ParasailResult;
    pub fn parasail_sw_scan_16(s1: *const u8,
                               s1Len: c_int,
                               s2: *const u8,
                               s2Len: c_int,
                               open: c_int,
                               gap: c_int,
                               matrix: *const ParasailMatrix)
                               -> *mut ParasailResult;
    pub fn parasail_sw_scan_8(s1: *const u8,
                              s1Len: c_int,
                              s2: *const u8,
                              s2Len: c_int,
                              open: c_int,
                              gap: c_int,
                              matrix: *const ParasailMatrix)
                              -> *mut ParasailResult;
    pub fn parasail_sw_scan_sat(s1: *const u8,
                                s1Len: c_int,
                                s2: *const u8,
                                s2Len: c_int,
                                open: c_int,
                                gap: c_int,
                                matrix: *const ParasailMatrix)
                                -> *mut ParasailResult;
    pub fn parasail_sw_striped_64(s1: *const u8,
                                  s1Len: c_int,
                                  s2: *const u8,
                                  s2Len: c_int,
                                  open: c_int,
                                  gap: c_int,
                                  matrix: *const ParasailMatrix)
                                  -> *mut ParasailResult;
    pub fn parasail_sw_striped_32(s1: *const u8,
                                  s1Len: c_int,
                                  s2: *const u8,
                                  s2Len: c_int,
                                  open: c_int,
                                  gap: c_int,
                                  matrix: *const ParasailMatrix)
                                  -> *mut ParasailResult;
    pub fn parasail_sw_striped_16(s1: *const u8,
                                  s1Len: c_int,
                                  s2: *const u8,
                                  s2Len: c_int,
                                  open: c_int,
                                  gap: c_int,
                                  matrix: *const ParasailMatrix)
                                  -> *mut ParasailResult;
    pub fn parasail_sw_striped_8(s1: *const u8,
                                 s1Len: c_int,
                                 s2: *const u8,
                                 s2Len: c_int,
                                 open: c_int,
                                 gap: c_int,
                                 matrix: *const ParasailMatrix)
                                 -> *mut ParasailResult;
    pub fn parasail_sw_striped_sat(s1: *const u8,
                                   s1Len: c_int,
                                   s2: *const u8,
                                   s2Len: c_int,
                                   open: c_int,
                                   gap: c_int,
                                   matrix: *const ParasailMatrix)
                                   -> *mut ParasailResult;
    pub fn parasail_sw_diag_64(s1: *const u8,
                               s1Len: c_int,
                               s2: *const u8,
                               s2Len: c_int,
                               open: c_int,
                               gap: c_int,
                               matrix: *const ParasailMatrix)
                               -> *mut ParasailResult;
    pub fn parasail_sw_diag_32(s1: *const u8,
                               s1Len: c_int,
                               s2: *const u8,
                               s2Len: c_int,
                               open: c_int,
                               gap: c_int,
                               matrix: *const ParasailMatrix)
                               -> *mut ParasailResult;
    pub fn parasail_sw_diag_16(s1: *const u8,
                               s1Len: c_int,
                               s2: *const u8,
                               s2Len: c_int,
                               open: c_int,
                               gap: c_int,
                               matrix: *const ParasailMatrix)
                               -> *mut ParasailResult;
    pub fn parasail_sw_diag_8(s1: *const u8,
                              s1Len: c_int,
                              s2: *const u8,
                              s2Len: c_int,
                              open: c_int,
                              gap: c_int,
                              matrix: *const ParasailMatrix)
                              -> *mut ParasailResult;
    pub fn parasail_sw_diag_sat(s1: *const u8,
                                s1Len: c_int,
                                s2: *const u8,
                                s2Len: c_int,
                                open: c_int,
                                gap: c_int,
                                matrix: *const ParasailMatrix)
                                -> *mut ParasailResult;
    pub fn parasail_sw_table_scan_64(s1: *const u8,
                                     s1Len: c_int,
                                     s2: *const u8,
                                     s2Len: c_int,
                                     open: c_int,
                                     gap: c_int,
                                     matrix: *const ParasailMatrix)
                                     -> *mut ParasailResult;
    pub fn parasail_sw_table_scan_32(s1: *const u8,
                                     s1Len: c_int,
                                     s2: *const u8,
                                     s2Len: c_int,
                                     open: c_int,
                                     gap: c_int,
                                     matrix: *const ParasailMatrix)
                                     -> *mut ParasailResult;
    pub fn parasail_sw_table_scan_16(s1: *const u8,
                                     s1Len: c_int,
                                     s2: *const u8,
                                     s2Len: c_int,
                                     open: c_int,
                                     gap: c_int,
                                     matrix: *const ParasailMatrix)
                                     -> *mut ParasailResult;
    pub fn parasail_sw_table_scan_8(s1: *const u8,
                                    s1Len: c_int,
                                    s2: *const u8,
                                    s2Len: c_int,
                                    open: c_int,
                                    gap: c_int,
                                    matrix: *const ParasailMatrix)
                                    -> *mut ParasailResult;
    pub fn parasail_sw_table_scan_sat(s1: *const u8,
                                      s1Len: c_int,
                                      s2: *const u8,
                                      s2Len: c_int,
                                      open: c_int,
                                      gap: c_int,
                                      matrix: *const ParasailMatrix)
                                      -> *mut ParasailResult;
    pub fn parasail_sw_table_striped_64(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_sw_table_striped_32(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_sw_table_striped_16(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_sw_table_striped_8(s1: *const u8,
                                       s1Len: c_int,
                                       s2: *const u8,
                                       s2Len: c_int,
                                       open: c_int,
                                       gap: c_int,
                                       matrix: *const ParasailMatrix)
                                       -> *mut ParasailResult;
    pub fn parasail_sw_table_striped_sat(s1: *const u8,
                                         s1Len: c_int,
                                         s2: *const u8,
                                         s2Len: c_int,
                                         open: c_int,
                                         gap: c_int,
                                         matrix: *const ParasailMatrix)
                                         -> *mut ParasailResult;
    pub fn parasail_sw_table_diag_64(s1: *const u8,
                                     s1Len: c_int,
                                     s2: *const u8,
                                     s2Len: c_int,
                                     open: c_int,
                                     gap: c_int,
                                     matrix: *const ParasailMatrix)
                                     -> *mut ParasailResult;
    pub fn parasail_sw_table_diag_32(s1: *const u8,
                                     s1Len: c_int,
                                     s2: *const u8,
                                     s2Len: c_int,
                                     open: c_int,
                                     gap: c_int,
                                     matrix: *const ParasailMatrix)
                                     -> *mut ParasailResult;
    pub fn parasail_sw_table_diag_16(s1: *const u8,
                                     s1Len: c_int,
                                     s2: *const u8,
                                     s2Len: c_int,
                                     open: c_int,
                                     gap: c_int,
                                     matrix: *const ParasailMatrix)
                                     -> *mut ParasailResult;
    pub fn parasail_sw_table_diag_8(s1: *const u8,
                                    s1Len: c_int,
                                    s2: *const u8,
                                    s2Len: c_int,
                                    open: c_int,
                                    gap: c_int,
                                    matrix: *const ParasailMatrix)
                                    -> *mut ParasailResult;
    pub fn parasail_sw_table_diag_sat(s1: *const u8,
                                      s1Len: c_int,
                                      s2: *const u8,
                                      s2Len: c_int,
                                      open: c_int,
                                      gap: c_int,
                                      matrix: *const ParasailMatrix)
                                      -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_scan_64(s1: *const u8,
                                      s1Len: c_int,
                                      s2: *const u8,
                                      s2Len: c_int,
                                      open: c_int,
                                      gap: c_int,
                                      matrix: *const ParasailMatrix)
                                      -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_scan_32(s1: *const u8,
                                      s1Len: c_int,
                                      s2: *const u8,
                                      s2Len: c_int,
                                      open: c_int,
                                      gap: c_int,
                                      matrix: *const ParasailMatrix)
                                      -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_scan_16(s1: *const u8,
                                      s1Len: c_int,
                                      s2: *const u8,
                                      s2Len: c_int,
                                      open: c_int,
                                      gap: c_int,
                                      matrix: *const ParasailMatrix)
                                      -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_scan_8(s1: *const u8,
                                     s1Len: c_int,
                                     s2: *const u8,
                                     s2Len: c_int,
                                     open: c_int,
                                     gap: c_int,
                                     matrix: *const ParasailMatrix)
                                     -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_scan_sat(s1: *const u8,
                                       s1Len: c_int,
                                       s2: *const u8,
                                       s2Len: c_int,
                                       open: c_int,
                                       gap: c_int,
                                       matrix: *const ParasailMatrix)
                                       -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_striped_64(s1: *const u8,
                                         s1Len: c_int,
                                         s2: *const u8,
                                         s2Len: c_int,
                                         open: c_int,
                                         gap: c_int,
                                         matrix: *const ParasailMatrix)
                                         -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_striped_32(s1: *const u8,
                                         s1Len: c_int,
                                         s2: *const u8,
                                         s2Len: c_int,
                                         open: c_int,
                                         gap: c_int,
                                         matrix: *const ParasailMatrix)
                                         -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_striped_16(s1: *const u8,
                                         s1Len: c_int,
                                         s2: *const u8,
                                         s2Len: c_int,
                                         open: c_int,
                                         gap: c_int,
                                         matrix: *const ParasailMatrix)
                                         -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_striped_8(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_striped_sat(s1: *const u8,
                                          s1Len: c_int,
                                          s2: *const u8,
                                          s2Len: c_int,
                                          open: c_int,
                                          gap: c_int,
                                          matrix: *const ParasailMatrix)
                                          -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_diag_64(s1: *const u8,
                                      s1Len: c_int,
                                      s2: *const u8,
                                      s2Len: c_int,
                                      open: c_int,
                                      gap: c_int,
                                      matrix: *const ParasailMatrix)
                                      -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_diag_32(s1: *const u8,
                                      s1Len: c_int,
                                      s2: *const u8,
                                      s2Len: c_int,
                                      open: c_int,
                                      gap: c_int,
                                      matrix: *const ParasailMatrix)
                                      -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_diag_16(s1: *const u8,
                                      s1Len: c_int,
                                      s2: *const u8,
                                      s2Len: c_int,
                                      open: c_int,
                                      gap: c_int,
                                      matrix: *const ParasailMatrix)
                                      -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_diag_8(s1: *const u8,
                                     s1Len: c_int,
                                     s2: *const u8,
                                     s2Len: c_int,
                                     open: c_int,
                                     gap: c_int,
                                     matrix: *const ParasailMatrix)
                                     -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_diag_sat(s1: *const u8,
                                       s1Len: c_int,
                                       s2: *const u8,
                                       s2Len: c_int,
                                       open: c_int,
                                       gap: c_int,
                                       matrix: *const ParasailMatrix)
                                       -> *mut ParasailResult;
    pub fn parasail_sw_stats_scan_64(s1: *const u8,
                                     s1Len: c_int,
                                     s2: *const u8,
                                     s2Len: c_int,
                                     open: c_int,
                                     gap: c_int,
                                     matrix: *const ParasailMatrix)
                                     -> *mut ParasailResult;
    pub fn parasail_sw_stats_scan_32(s1: *const u8,
                                     s1Len: c_int,
                                     s2: *const u8,
                                     s2Len: c_int,
                                     open: c_int,
                                     gap: c_int,
                                     matrix: *const ParasailMatrix)
                                     -> *mut ParasailResult;
    pub fn parasail_sw_stats_scan_16(s1: *const u8,
                                     s1Len: c_int,
                                     s2: *const u8,
                                     s2Len: c_int,
                                     open: c_int,
                                     gap: c_int,
                                     matrix: *const ParasailMatrix)
                                     -> *mut ParasailResult;
    pub fn parasail_sw_stats_scan_8(s1: *const u8,
                                    s1Len: c_int,
                                    s2: *const u8,
                                    s2Len: c_int,
                                    open: c_int,
                                    gap: c_int,
                                    matrix: *const ParasailMatrix)
                                    -> *mut ParasailResult;
    pub fn parasail_sw_stats_scan_sat(s1: *const u8,
                                      s1Len: c_int,
                                      s2: *const u8,
                                      s2Len: c_int,
                                      open: c_int,
                                      gap: c_int,
                                      matrix: *const ParasailMatrix)
                                      -> *mut ParasailResult;
    pub fn parasail_sw_stats_striped_64(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_sw_stats_striped_32(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_sw_stats_striped_16(s1: *const u8,
                                        s1Len: c_int,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int,
                                        matrix: *const ParasailMatrix)
                                        -> *mut ParasailResult;
    pub fn parasail_sw_stats_striped_8(s1: *const u8,
                                       s1Len: c_int,
                                       s2: *const u8,
                                       s2Len: c_int,
                                       open: c_int,
                                       gap: c_int,
                                       matrix: *const ParasailMatrix)
                                       -> *mut ParasailResult;
    pub fn parasail_sw_stats_striped_sat(s1: *const u8,
                                         s1Len: c_int,
                                         s2: *const u8,
                                         s2Len: c_int,
                                         open: c_int,
                                         gap: c_int,
                                         matrix: *const ParasailMatrix)
                                         -> *mut ParasailResult;
    pub fn parasail_sw_stats_diag_64(s1: *const u8,
                                     s1Len: c_int,
                                     s2: *const u8,
                                     s2Len: c_int,
                                     open: c_int,
                                     gap: c_int,
                                     matrix: *const ParasailMatrix)
                                     -> *mut ParasailResult;
    pub fn parasail_sw_stats_diag_32(s1: *const u8,
                                     s1Len: c_int,
                                     s2: *const u8,
                                     s2Len: c_int,
                                     open: c_int,
                                     gap: c_int,
                                     matrix: *const ParasailMatrix)
                                     -> *mut ParasailResult;
    pub fn parasail_sw_stats_diag_16(s1: *const u8,
                                     s1Len: c_int,
                                     s2: *const u8,
                                     s2Len: c_int,
                                     open: c_int,
                                     gap: c_int,
                                     matrix: *const ParasailMatrix)
                                     -> *mut ParasailResult;
    pub fn parasail_sw_stats_diag_8(s1: *const u8,
                                    s1Len: c_int,
                                    s2: *const u8,
                                    s2Len: c_int,
                                    open: c_int,
                                    gap: c_int,
                                    matrix: *const ParasailMatrix)
                                    -> *mut ParasailResult;
    pub fn parasail_sw_stats_diag_sat(s1: *const u8,
                                      s1Len: c_int,
                                      s2: *const u8,
                                      s2Len: c_int,
                                      open: c_int,
                                      gap: c_int,
                                      matrix: *const ParasailMatrix)
                                      -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_scan_64(s1: *const u8,
                                           s1Len: c_int,
                                           s2: *const u8,
                                           s2Len: c_int,
                                           open: c_int,
                                           gap: c_int,
                                           matrix: *const ParasailMatrix)
                                           -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_scan_32(s1: *const u8,
                                           s1Len: c_int,
                                           s2: *const u8,
                                           s2Len: c_int,
                                           open: c_int,
                                           gap: c_int,
                                           matrix: *const ParasailMatrix)
                                           -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_scan_16(s1: *const u8,
                                           s1Len: c_int,
                                           s2: *const u8,
                                           s2Len: c_int,
                                           open: c_int,
                                           gap: c_int,
                                           matrix: *const ParasailMatrix)
                                           -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_scan_8(s1: *const u8,
                                          s1Len: c_int,
                                          s2: *const u8,
                                          s2Len: c_int,
                                          open: c_int,
                                          gap: c_int,
                                          matrix: *const ParasailMatrix)
                                          -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_scan_sat(s1: *const u8,
                                            s1Len: c_int,
                                            s2: *const u8,
                                            s2Len: c_int,
                                            open: c_int,
                                            gap: c_int,
                                            matrix: *const ParasailMatrix)
                                            -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_striped_64(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_striped_32(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_striped_16(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_striped_8(s1: *const u8,
                                             s1Len: c_int,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int,
                                             matrix: *const ParasailMatrix)
                                             -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_striped_sat(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_diag_64(s1: *const u8,
                                           s1Len: c_int,
                                           s2: *const u8,
                                           s2Len: c_int,
                                           open: c_int,
                                           gap: c_int,
                                           matrix: *const ParasailMatrix)
                                           -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_diag_32(s1: *const u8,
                                           s1Len: c_int,
                                           s2: *const u8,
                                           s2Len: c_int,
                                           open: c_int,
                                           gap: c_int,
                                           matrix: *const ParasailMatrix)
                                           -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_diag_16(s1: *const u8,
                                           s1Len: c_int,
                                           s2: *const u8,
                                           s2Len: c_int,
                                           open: c_int,
                                           gap: c_int,
                                           matrix: *const ParasailMatrix)
                                           -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_diag_8(s1: *const u8,
                                          s1Len: c_int,
                                          s2: *const u8,
                                          s2Len: c_int,
                                          open: c_int,
                                          gap: c_int,
                                          matrix: *const ParasailMatrix)
                                          -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_diag_sat(s1: *const u8,
                                            s1Len: c_int,
                                            s2: *const u8,
                                            s2Len: c_int,
                                            open: c_int,
                                            gap: c_int,
                                            matrix: *const ParasailMatrix)
                                            -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_scan_64(s1: *const u8,
                                            s1Len: c_int,
                                            s2: *const u8,
                                            s2Len: c_int,
                                            open: c_int,
                                            gap: c_int,
                                            matrix: *const ParasailMatrix)
                                            -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_scan_32(s1: *const u8,
                                            s1Len: c_int,
                                            s2: *const u8,
                                            s2Len: c_int,
                                            open: c_int,
                                            gap: c_int,
                                            matrix: *const ParasailMatrix)
                                            -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_scan_16(s1: *const u8,
                                            s1Len: c_int,
                                            s2: *const u8,
                                            s2Len: c_int,
                                            open: c_int,
                                            gap: c_int,
                                            matrix: *const ParasailMatrix)
                                            -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_scan_8(s1: *const u8,
                                           s1Len: c_int,
                                           s2: *const u8,
                                           s2Len: c_int,
                                           open: c_int,
                                           gap: c_int,
                                           matrix: *const ParasailMatrix)
                                           -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_scan_sat(s1: *const u8,
                                             s1Len: c_int,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int,
                                             matrix: *const ParasailMatrix)
                                             -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_striped_64(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_striped_32(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_striped_16(s1: *const u8,
                                               s1Len: c_int,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_striped_8(s1: *const u8,
                                              s1Len: c_int,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_striped_sat(s1: *const u8,
                                                s1Len: c_int,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int,
                                                matrix: *const ParasailMatrix)
                                                -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_diag_64(s1: *const u8,
                                            s1Len: c_int,
                                            s2: *const u8,
                                            s2Len: c_int,
                                            open: c_int,
                                            gap: c_int,
                                            matrix: *const ParasailMatrix)
                                            -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_diag_32(s1: *const u8,
                                            s1Len: c_int,
                                            s2: *const u8,
                                            s2Len: c_int,
                                            open: c_int,
                                            gap: c_int,
                                            matrix: *const ParasailMatrix)
                                            -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_diag_16(s1: *const u8,
                                            s1Len: c_int,
                                            s2: *const u8,
                                            s2Len: c_int,
                                            open: c_int,
                                            gap: c_int,
                                            matrix: *const ParasailMatrix)
                                            -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_diag_8(s1: *const u8,
                                           s1Len: c_int,
                                           s2: *const u8,
                                           s2Len: c_int,
                                           open: c_int,
                                           gap: c_int,
                                           matrix: *const ParasailMatrix)
                                           -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_diag_sat(s1: *const u8,
                                             s1Len: c_int,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int,
                                             matrix: *const ParasailMatrix)
                                             -> *mut ParasailResult;
    pub fn parasail_nw_scan_profile_64(profile: *const ParasailProfile,
                                       s2: *const u8,
                                       s2Len: c_int,
                                       open: c_int,
                                       gap: c_int)
                                       -> *mut ParasailResult;
    pub fn parasail_nw_scan_profile_32(profile: *const ParasailProfile,
                                       s2: *const u8,
                                       s2Len: c_int,
                                       open: c_int,
                                       gap: c_int)
                                       -> *mut ParasailResult;
    pub fn parasail_nw_scan_profile_16(profile: *const ParasailProfile,
                                       s2: *const u8,
                                       s2Len: c_int,
                                       open: c_int,
                                       gap: c_int)
                                       -> *mut ParasailResult;
    pub fn parasail_nw_scan_profile_8(profile: *const ParasailProfile,
                                      s2: *const u8,
                                      s2Len: c_int,
                                      open: c_int,
                                      gap: c_int)
                                      -> *mut ParasailResult;
    pub fn parasail_nw_scan_profile_sat(profile: *const ParasailProfile,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int)
                                        -> *mut ParasailResult;
    pub fn parasail_nw_striped_profile_64(profile: *const ParasailProfile,
                                          s2: *const u8,
                                          s2Len: c_int,
                                          open: c_int,
                                          gap: c_int)
                                          -> *mut ParasailResult;
    pub fn parasail_nw_striped_profile_32(profile: *const ParasailProfile,
                                          s2: *const u8,
                                          s2Len: c_int,
                                          open: c_int,
                                          gap: c_int)
                                          -> *mut ParasailResult;
    pub fn parasail_nw_striped_profile_16(profile: *const ParasailProfile,
                                          s2: *const u8,
                                          s2Len: c_int,
                                          open: c_int,
                                          gap: c_int)
                                          -> *mut ParasailResult;
    pub fn parasail_nw_striped_profile_8(profile: *const ParasailProfile,
                                         s2: *const u8,
                                         s2Len: c_int,
                                         open: c_int,
                                         gap: c_int)
                                         -> *mut ParasailResult;
    pub fn parasail_nw_striped_profile_sat(profile: *const ParasailProfile,
                                           s2: *const u8,
                                           s2Len: c_int,
                                           open: c_int,
                                           gap: c_int)
                                           -> *mut ParasailResult;
    pub fn parasail_nw_table_scan_profile_64(profile: *const ParasailProfile,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int)
                                             -> *mut ParasailResult;
    pub fn parasail_nw_table_scan_profile_32(profile: *const ParasailProfile,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int)
                                             -> *mut ParasailResult;
    pub fn parasail_nw_table_scan_profile_16(profile: *const ParasailProfile,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int)
                                             -> *mut ParasailResult;
    pub fn parasail_nw_table_scan_profile_8(profile: *const ParasailProfile,
                                            s2: *const u8,
                                            s2Len: c_int,
                                            open: c_int,
                                            gap: c_int)
                                            -> *mut ParasailResult;
    pub fn parasail_nw_table_scan_profile_sat(profile: *const ParasailProfile,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int)
                                              -> *mut ParasailResult;
    pub fn parasail_nw_table_striped_profile_64(profile: *const ParasailProfile,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int)
                                                -> *mut ParasailResult;
    pub fn parasail_nw_table_striped_profile_32(profile: *const ParasailProfile,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int)
                                                -> *mut ParasailResult;
    pub fn parasail_nw_table_striped_profile_16(profile: *const ParasailProfile,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int)
                                                -> *mut ParasailResult;
    pub fn parasail_nw_table_striped_profile_8(profile: *const ParasailProfile,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int)
                                               -> *mut ParasailResult;
    pub fn parasail_nw_table_striped_profile_sat(profile: *const ParasailProfile,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int)
                                                 -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_scan_profile_64(profile: *const ParasailProfile,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int)
                                              -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_scan_profile_32(profile: *const ParasailProfile,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int)
                                              -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_scan_profile_16(profile: *const ParasailProfile,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int)
                                              -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_scan_profile_8(profile: *const ParasailProfile,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int)
                                             -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_scan_profile_sat(profile: *const ParasailProfile,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int)
                                               -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_striped_profile_64(profile: *const ParasailProfile,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int)
                                                 -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_striped_profile_32(profile: *const ParasailProfile,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int)
                                                 -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_striped_profile_16(profile: *const ParasailProfile,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int)
                                                 -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_striped_profile_8(profile: *const ParasailProfile,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int)
                                                -> *mut ParasailResult;
    pub fn parasail_nw_rowcol_striped_profile_sat(profile: *const ParasailProfile,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int)
                                                  -> *mut ParasailResult;
    pub fn parasail_nw_stats_scan_profile_64(profile: *const ParasailProfile,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int)
                                             -> *mut ParasailResult;
    pub fn parasail_nw_stats_scan_profile_32(profile: *const ParasailProfile,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int)
                                             -> *mut ParasailResult;
    pub fn parasail_nw_stats_scan_profile_16(profile: *const ParasailProfile,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int)
                                             -> *mut ParasailResult;
    pub fn parasail_nw_stats_scan_profile_8(profile: *const ParasailProfile,
                                            s2: *const u8,
                                            s2Len: c_int,
                                            open: c_int,
                                            gap: c_int)
                                            -> *mut ParasailResult;
    pub fn parasail_nw_stats_scan_profile_sat(profile: *const ParasailProfile,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int)
                                              -> *mut ParasailResult;
    pub fn parasail_nw_stats_striped_profile_64(profile: *const ParasailProfile,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int)
                                                -> *mut ParasailResult;
    pub fn parasail_nw_stats_striped_profile_32(profile: *const ParasailProfile,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int)
                                                -> *mut ParasailResult;
    pub fn parasail_nw_stats_striped_profile_16(profile: *const ParasailProfile,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int)
                                                -> *mut ParasailResult;
    pub fn parasail_nw_stats_striped_profile_8(profile: *const ParasailProfile,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int)
                                               -> *mut ParasailResult;
    pub fn parasail_nw_stats_striped_profile_sat(profile: *const ParasailProfile,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int)
                                                 -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_scan_profile_64(profile: *const ParasailProfile,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int)
                                                   -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_scan_profile_32(profile: *const ParasailProfile,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int)
                                                   -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_scan_profile_16(profile: *const ParasailProfile,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int)
                                                   -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_scan_profile_8(profile: *const ParasailProfile,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int)
                                                  -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_scan_profile_sat(profile: *const ParasailProfile,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int)
                                                    -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_striped_profile_64(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_striped_profile_32(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_striped_profile_16(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_striped_profile_8(profile: *const ParasailProfile,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int)
                                                     -> *mut ParasailResult;
    pub fn parasail_nw_stats_table_striped_profile_sat(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_scan_profile_64(profile: *const ParasailProfile,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int)
                                                    -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_scan_profile_32(profile: *const ParasailProfile,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int)
                                                    -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_scan_profile_16(profile: *const ParasailProfile,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int)
                                                    -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_scan_profile_8(profile: *const ParasailProfile,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int)
                                                   -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_scan_profile_sat(profile: *const ParasailProfile,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int)
                                                     -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_striped_profile_64(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_striped_profile_32(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_striped_profile_16(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_striped_profile_8(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_nw_stats_rowcol_striped_profile_sat(profile: *const ParasailProfile,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int)
                                                        -> *mut ParasailResult;
    pub fn parasail_sg_scan_profile_64(profile: *const ParasailProfile,
                                       s2: *const u8,
                                       s2Len: c_int,
                                       open: c_int,
                                       gap: c_int)
                                       -> *mut ParasailResult;
    pub fn parasail_sg_scan_profile_32(profile: *const ParasailProfile,
                                       s2: *const u8,
                                       s2Len: c_int,
                                       open: c_int,
                                       gap: c_int)
                                       -> *mut ParasailResult;
    pub fn parasail_sg_scan_profile_16(profile: *const ParasailProfile,
                                       s2: *const u8,
                                       s2Len: c_int,
                                       open: c_int,
                                       gap: c_int)
                                       -> *mut ParasailResult;
    pub fn parasail_sg_scan_profile_8(profile: *const ParasailProfile,
                                      s2: *const u8,
                                      s2Len: c_int,
                                      open: c_int,
                                      gap: c_int)
                                      -> *mut ParasailResult;
    pub fn parasail_sg_scan_profile_sat(profile: *const ParasailProfile,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int)
                                        -> *mut ParasailResult;
    pub fn parasail_sg_striped_profile_64(profile: *const ParasailProfile,
                                          s2: *const u8,
                                          s2Len: c_int,
                                          open: c_int,
                                          gap: c_int)
                                          -> *mut ParasailResult;
    pub fn parasail_sg_striped_profile_32(profile: *const ParasailProfile,
                                          s2: *const u8,
                                          s2Len: c_int,
                                          open: c_int,
                                          gap: c_int)
                                          -> *mut ParasailResult;
    pub fn parasail_sg_striped_profile_16(profile: *const ParasailProfile,
                                          s2: *const u8,
                                          s2Len: c_int,
                                          open: c_int,
                                          gap: c_int)
                                          -> *mut ParasailResult;
    pub fn parasail_sg_striped_profile_8(profile: *const ParasailProfile,
                                         s2: *const u8,
                                         s2Len: c_int,
                                         open: c_int,
                                         gap: c_int)
                                         -> *mut ParasailResult;
    pub fn parasail_sg_striped_profile_sat(profile: *const ParasailProfile,
                                           s2: *const u8,
                                           s2Len: c_int,
                                           open: c_int,
                                           gap: c_int)
                                           -> *mut ParasailResult;
    pub fn parasail_sg_table_scan_profile_64(profile: *const ParasailProfile,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int)
                                             -> *mut ParasailResult;
    pub fn parasail_sg_table_scan_profile_32(profile: *const ParasailProfile,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int)
                                             -> *mut ParasailResult;
    pub fn parasail_sg_table_scan_profile_16(profile: *const ParasailProfile,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int)
                                             -> *mut ParasailResult;
    pub fn parasail_sg_table_scan_profile_8(profile: *const ParasailProfile,
                                            s2: *const u8,
                                            s2Len: c_int,
                                            open: c_int,
                                            gap: c_int)
                                            -> *mut ParasailResult;
    pub fn parasail_sg_table_scan_profile_sat(profile: *const ParasailProfile,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int)
                                              -> *mut ParasailResult;
    pub fn parasail_sg_table_striped_profile_64(profile: *const ParasailProfile,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int)
                                                -> *mut ParasailResult;
    pub fn parasail_sg_table_striped_profile_32(profile: *const ParasailProfile,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int)
                                                -> *mut ParasailResult;
    pub fn parasail_sg_table_striped_profile_16(profile: *const ParasailProfile,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int)
                                                -> *mut ParasailResult;
    pub fn parasail_sg_table_striped_profile_8(profile: *const ParasailProfile,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int)
                                               -> *mut ParasailResult;
    pub fn parasail_sg_table_striped_profile_sat(profile: *const ParasailProfile,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int)
                                                 -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_scan_profile_64(profile: *const ParasailProfile,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int)
                                              -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_scan_profile_32(profile: *const ParasailProfile,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int)
                                              -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_scan_profile_16(profile: *const ParasailProfile,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int)
                                              -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_scan_profile_8(profile: *const ParasailProfile,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int)
                                             -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_scan_profile_sat(profile: *const ParasailProfile,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int)
                                               -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_striped_profile_64(profile: *const ParasailProfile,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int)
                                                 -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_striped_profile_32(profile: *const ParasailProfile,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int)
                                                 -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_striped_profile_16(profile: *const ParasailProfile,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int)
                                                 -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_striped_profile_8(profile: *const ParasailProfile,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int)
                                                -> *mut ParasailResult;
    pub fn parasail_sg_rowcol_striped_profile_sat(profile: *const ParasailProfile,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int)
                                                  -> *mut ParasailResult;
    pub fn parasail_sg_stats_scan_profile_64(profile: *const ParasailProfile,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int)
                                             -> *mut ParasailResult;
    pub fn parasail_sg_stats_scan_profile_32(profile: *const ParasailProfile,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int)
                                             -> *mut ParasailResult;
    pub fn parasail_sg_stats_scan_profile_16(profile: *const ParasailProfile,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int)
                                             -> *mut ParasailResult;
    pub fn parasail_sg_stats_scan_profile_8(profile: *const ParasailProfile,
                                            s2: *const u8,
                                            s2Len: c_int,
                                            open: c_int,
                                            gap: c_int)
                                            -> *mut ParasailResult;
    pub fn parasail_sg_stats_scan_profile_sat(profile: *const ParasailProfile,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int)
                                              -> *mut ParasailResult;
    pub fn parasail_sg_stats_striped_profile_64(profile: *const ParasailProfile,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int)
                                                -> *mut ParasailResult;
    pub fn parasail_sg_stats_striped_profile_32(profile: *const ParasailProfile,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int)
                                                -> *mut ParasailResult;
    pub fn parasail_sg_stats_striped_profile_16(profile: *const ParasailProfile,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int)
                                                -> *mut ParasailResult;
    pub fn parasail_sg_stats_striped_profile_8(profile: *const ParasailProfile,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int)
                                               -> *mut ParasailResult;
    pub fn parasail_sg_stats_striped_profile_sat(profile: *const ParasailProfile,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int)
                                                 -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_scan_profile_64(profile: *const ParasailProfile,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int)
                                                   -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_scan_profile_32(profile: *const ParasailProfile,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int)
                                                   -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_scan_profile_16(profile: *const ParasailProfile,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int)
                                                   -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_scan_profile_8(profile: *const ParasailProfile,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int)
                                                  -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_scan_profile_sat(profile: *const ParasailProfile,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int)
                                                    -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_striped_profile_64(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_striped_profile_32(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_striped_profile_16(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_striped_profile_8(profile: *const ParasailProfile,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int)
                                                     -> *mut ParasailResult;
    pub fn parasail_sg_stats_table_striped_profile_sat(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_scan_profile_64(profile: *const ParasailProfile,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int)
                                                    -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_scan_profile_32(profile: *const ParasailProfile,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int)
                                                    -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_scan_profile_16(profile: *const ParasailProfile,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int)
                                                    -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_scan_profile_8(profile: *const ParasailProfile,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int)
                                                   -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_scan_profile_sat(profile: *const ParasailProfile,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int)
                                                     -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_striped_profile_64(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_striped_profile_32(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_striped_profile_16(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_striped_profile_8(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_sg_stats_rowcol_striped_profile_sat(profile: *const ParasailProfile,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int)
                                                        -> *mut ParasailResult;
    pub fn parasail_sw_scan_profile_64(profile: *const ParasailProfile,
                                       s2: *const u8,
                                       s2Len: c_int,
                                       open: c_int,
                                       gap: c_int)
                                       -> *mut ParasailResult;
    pub fn parasail_sw_scan_profile_32(profile: *const ParasailProfile,
                                       s2: *const u8,
                                       s2Len: c_int,
                                       open: c_int,
                                       gap: c_int)
                                       -> *mut ParasailResult;
    pub fn parasail_sw_scan_profile_16(profile: *const ParasailProfile,
                                       s2: *const u8,
                                       s2Len: c_int,
                                       open: c_int,
                                       gap: c_int)
                                       -> *mut ParasailResult;
    pub fn parasail_sw_scan_profile_8(profile: *const ParasailProfile,
                                      s2: *const u8,
                                      s2Len: c_int,
                                      open: c_int,
                                      gap: c_int)
                                      -> *mut ParasailResult;
    pub fn parasail_sw_scan_profile_sat(profile: *const ParasailProfile,
                                        s2: *const u8,
                                        s2Len: c_int,
                                        open: c_int,
                                        gap: c_int)
                                        -> *mut ParasailResult;
    pub fn parasail_sw_striped_profile_64(profile: *const ParasailProfile,
                                          s2: *const u8,
                                          s2Len: c_int,
                                          open: c_int,
                                          gap: c_int)
                                          -> *mut ParasailResult;
    pub fn parasail_sw_striped_profile_32(profile: *const ParasailProfile,
                                          s2: *const u8,
                                          s2Len: c_int,
                                          open: c_int,
                                          gap: c_int)
                                          -> *mut ParasailResult;
    pub fn parasail_sw_striped_profile_16(profile: *const ParasailProfile,
                                          s2: *const u8,
                                          s2Len: c_int,
                                          open: c_int,
                                          gap: c_int)
                                          -> *mut ParasailResult;
    pub fn parasail_sw_striped_profile_8(profile: *const ParasailProfile,
                                         s2: *const u8,
                                         s2Len: c_int,
                                         open: c_int,
                                         gap: c_int)
                                         -> *mut ParasailResult;
    pub fn parasail_sw_striped_profile_sat(profile: *const ParasailProfile,
                                           s2: *const u8,
                                           s2Len: c_int,
                                           open: c_int,
                                           gap: c_int)
                                           -> *mut ParasailResult;
    pub fn parasail_sw_table_scan_profile_64(profile: *const ParasailProfile,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int)
                                             -> *mut ParasailResult;
    pub fn parasail_sw_table_scan_profile_32(profile: *const ParasailProfile,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int)
                                             -> *mut ParasailResult;
    pub fn parasail_sw_table_scan_profile_16(profile: *const ParasailProfile,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int)
                                             -> *mut ParasailResult;
    pub fn parasail_sw_table_scan_profile_8(profile: *const ParasailProfile,
                                            s2: *const u8,
                                            s2Len: c_int,
                                            open: c_int,
                                            gap: c_int)
                                            -> *mut ParasailResult;
    pub fn parasail_sw_table_scan_profile_sat(profile: *const ParasailProfile,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int)
                                              -> *mut ParasailResult;
    pub fn parasail_sw_table_striped_profile_64(profile: *const ParasailProfile,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int)
                                                -> *mut ParasailResult;
    pub fn parasail_sw_table_striped_profile_32(profile: *const ParasailProfile,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int)
                                                -> *mut ParasailResult;
    pub fn parasail_sw_table_striped_profile_16(profile: *const ParasailProfile,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int)
                                                -> *mut ParasailResult;
    pub fn parasail_sw_table_striped_profile_8(profile: *const ParasailProfile,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int)
                                               -> *mut ParasailResult;
    pub fn parasail_sw_table_striped_profile_sat(profile: *const ParasailProfile,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int)
                                                 -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_scan_profile_64(profile: *const ParasailProfile,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int)
                                              -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_scan_profile_32(profile: *const ParasailProfile,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int)
                                              -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_scan_profile_16(profile: *const ParasailProfile,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int)
                                              -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_scan_profile_8(profile: *const ParasailProfile,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int)
                                             -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_scan_profile_sat(profile: *const ParasailProfile,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int)
                                               -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_striped_profile_64(profile: *const ParasailProfile,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int)
                                                 -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_striped_profile_32(profile: *const ParasailProfile,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int)
                                                 -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_striped_profile_16(profile: *const ParasailProfile,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int)
                                                 -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_striped_profile_8(profile: *const ParasailProfile,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int)
                                                -> *mut ParasailResult;
    pub fn parasail_sw_rowcol_striped_profile_sat(profile: *const ParasailProfile,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int)
                                                  -> *mut ParasailResult;
    pub fn parasail_sw_stats_scan_profile_64(profile: *const ParasailProfile,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int)
                                             -> *mut ParasailResult;
    pub fn parasail_sw_stats_scan_profile_32(profile: *const ParasailProfile,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int)
                                             -> *mut ParasailResult;
    pub fn parasail_sw_stats_scan_profile_16(profile: *const ParasailProfile,
                                             s2: *const u8,
                                             s2Len: c_int,
                                             open: c_int,
                                             gap: c_int)
                                             -> *mut ParasailResult;
    pub fn parasail_sw_stats_scan_profile_8(profile: *const ParasailProfile,
                                            s2: *const u8,
                                            s2Len: c_int,
                                            open: c_int,
                                            gap: c_int)
                                            -> *mut ParasailResult;
    pub fn parasail_sw_stats_scan_profile_sat(profile: *const ParasailProfile,
                                              s2: *const u8,
                                              s2Len: c_int,
                                              open: c_int,
                                              gap: c_int)
                                              -> *mut ParasailResult;
    pub fn parasail_sw_stats_striped_profile_64(profile: *const ParasailProfile,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int)
                                                -> *mut ParasailResult;
    pub fn parasail_sw_stats_striped_profile_32(profile: *const ParasailProfile,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int)
                                                -> *mut ParasailResult;
    pub fn parasail_sw_stats_striped_profile_16(profile: *const ParasailProfile,
                                                s2: *const u8,
                                                s2Len: c_int,
                                                open: c_int,
                                                gap: c_int)
                                                -> *mut ParasailResult;
    pub fn parasail_sw_stats_striped_profile_8(profile: *const ParasailProfile,
                                               s2: *const u8,
                                               s2Len: c_int,
                                               open: c_int,
                                               gap: c_int)
                                               -> *mut ParasailResult;
    pub fn parasail_sw_stats_striped_profile_sat(profile: *const ParasailProfile,
                                                 s2: *const u8,
                                                 s2Len: c_int,
                                                 open: c_int,
                                                 gap: c_int)
                                                 -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_scan_profile_64(profile: *const ParasailProfile,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int)
                                                   -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_scan_profile_32(profile: *const ParasailProfile,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int)
                                                   -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_scan_profile_16(profile: *const ParasailProfile,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int)
                                                   -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_scan_profile_8(profile: *const ParasailProfile,
                                                  s2: *const u8,
                                                  s2Len: c_int,
                                                  open: c_int,
                                                  gap: c_int)
                                                  -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_scan_profile_sat(profile: *const ParasailProfile,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int)
                                                    -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_striped_profile_64(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_striped_profile_32(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_striped_profile_16(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_striped_profile_8(profile: *const ParasailProfile,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int)
                                                     -> *mut ParasailResult;
    pub fn parasail_sw_stats_table_striped_profile_sat(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_scan_profile_64(profile: *const ParasailProfile,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int)
                                                    -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_scan_profile_32(profile: *const ParasailProfile,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int)
                                                    -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_scan_profile_16(profile: *const ParasailProfile,
                                                    s2: *const u8,
                                                    s2Len: c_int,
                                                    open: c_int,
                                                    gap: c_int)
                                                    -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_scan_profile_8(profile: *const ParasailProfile,
                                                   s2: *const u8,
                                                   s2Len: c_int,
                                                   open: c_int,
                                                   gap: c_int)
                                                   -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_scan_profile_sat(profile: *const ParasailProfile,
                                                     s2: *const u8,
                                                     s2Len: c_int,
                                                     open: c_int,
                                                     gap: c_int)
                                                     -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_striped_profile_64(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_striped_profile_32(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_striped_profile_16(profile: *const ParasailProfile,
                                                       s2: *const u8,
                                                       s2Len: c_int,
                                                       open: c_int,
                                                       gap: c_int)
                                                       -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_striped_profile_8(profile: *const ParasailProfile,
                                                      s2: *const u8,
                                                      s2Len: c_int,
                                                      open: c_int,
                                                      gap: c_int)
                                                      -> *mut ParasailResult;
    pub fn parasail_sw_stats_rowcol_striped_profile_sat(profile: *const ParasailProfile,
                                                        s2: *const u8,
                                                        s2Len: c_int,
                                                        open: c_int,
                                                        gap: c_int)
                                                        -> *mut ParasailResult;
    pub fn parasail_profile_create_sse_128_64(s1: *const u8,
                                              s1Len: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailProfile;
    pub fn parasail_profile_create_sse_128_32(s1: *const u8,
                                              s1Len: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailProfile;
    pub fn parasail_profile_create_sse_128_16(s1: *const u8,
                                              s1Len: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailProfile;
    pub fn parasail_profile_create_sse_128_8(s1: *const u8,
                                             s1Len: c_int,
                                             matrix: *const ParasailMatrix)
                                             -> *mut ParasailProfile;
    pub fn parasail_profile_create_sse_128_sat(s1: *const u8,
                                               s1Len: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailProfile;
    pub fn parasail_profile_create_avx_256_64(s1: *const u8,
                                              s1Len: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailProfile;
    pub fn parasail_profile_create_avx_256_32(s1: *const u8,
                                              s1Len: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailProfile;
    pub fn parasail_profile_create_avx_256_16(s1: *const u8,
                                              s1Len: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailProfile;
    pub fn parasail_profile_create_avx_256_8(s1: *const u8,
                                             s1Len: c_int,
                                             matrix: *const ParasailMatrix)
                                             -> *mut ParasailProfile;
    pub fn parasail_profile_create_avx_256_sat(s1: *const u8,
                                               s1Len: c_int,
                                               matrix: *const ParasailMatrix)
                                               -> *mut ParasailProfile;
    pub fn parasail_profile_create_knc_512_32(s1: *const u8,
                                              s1Len: c_int,
                                              matrix: *const ParasailMatrix)
                                              -> *mut ParasailProfile;
    pub fn parasail_profile_create_64(s1: *const u8,
                                      s1Len: c_int,
                                      matrix: *const ParasailMatrix)
                                      -> *mut ParasailProfile;
    pub fn parasail_profile_create_32(s1: *const u8,
                                      s1Len: c_int,
                                      matrix: *const ParasailMatrix)
                                      -> *mut ParasailProfile;
    pub fn parasail_profile_create_16(s1: *const u8,
                                      s1Len: c_int,
                                      matrix: *const ParasailMatrix)
                                      -> *mut ParasailProfile;
    pub fn parasail_profile_create_8(s1: *const u8,
                                     s1Len: c_int,
                                     matrix: *const ParasailMatrix)
                                     -> *mut ParasailProfile;
    pub fn parasail_profile_create_sat(s1: *const u8,
                                       s1Len: c_int,
                                       matrix: *const ParasailMatrix)
                                       -> *mut ParasailProfile;
    pub fn parasail_profile_create_stats_sse_128_64(s1: *const u8,
                                                    s1Len: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailProfile;
    pub fn parasail_profile_create_stats_sse_128_32(s1: *const u8,
                                                    s1Len: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailProfile;
    pub fn parasail_profile_create_stats_sse_128_16(s1: *const u8,
                                                    s1Len: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailProfile;
    pub fn parasail_profile_create_stats_sse_128_8(s1: *const u8,
                                                   s1Len: c_int,
                                                   matrix: *const ParasailMatrix)
                                                   -> *mut ParasailProfile;
    pub fn parasail_profile_create_stats_sse_128_sat(s1: *const u8,
                                                     s1Len: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailProfile;
    pub fn parasail_profile_create_stats_avx_256_64(s1: *const u8,
                                                    s1Len: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailProfile;
    pub fn parasail_profile_create_stats_avx_256_32(s1: *const u8,
                                                    s1Len: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailProfile;
    pub fn parasail_profile_create_stats_avx_256_16(s1: *const u8,
                                                    s1Len: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailProfile;
    pub fn parasail_profile_create_stats_avx_256_8(s1: *const u8,
                                                   s1Len: c_int,
                                                   matrix: *const ParasailMatrix)
                                                   -> *mut ParasailProfile;
    pub fn parasail_profile_create_stats_avx_256_sat(s1: *const u8,
                                                     s1Len: c_int,
                                                     matrix: *const ParasailMatrix)
                                                     -> *mut ParasailProfile;
    pub fn parasail_profile_create_stats_knc_512_32(s1: *const u8,
                                                    s1Len: c_int,
                                                    matrix: *const ParasailMatrix)
                                                    -> *mut ParasailProfile;
    pub fn parasail_profile_create_stats_64(s1: *const u8,
                                            s1Len: c_int,
                                            matrix: *const ParasailMatrix)
                                            -> *mut ParasailProfile;
    pub fn parasail_profile_create_stats_32(s1: *const u8,
                                            s1Len: c_int,
                                            matrix: *const ParasailMatrix)
                                            -> *mut ParasailProfile;
    pub fn parasail_profile_create_stats_16(s1: *const u8,
                                            s1Len: c_int,
                                            matrix: *const ParasailMatrix)
                                            -> *mut ParasailProfile;
    pub fn parasail_profile_create_stats_8(s1: *const u8,
                                           s1Len: c_int,
                                           matrix: *const ParasailMatrix)
                                           -> *mut ParasailProfile;
    pub fn parasail_profile_create_stats_sat(s1: *const u8,
                                             s1Len: c_int,
                                             matrix: *const ParasailMatrix)
                                             -> *mut ParasailProfile;
}
