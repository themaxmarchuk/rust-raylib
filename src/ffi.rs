#![allow(non_snake_case)]
#![allow(non_camel_case_types)]


#[cfg(not(feature = "prebuilt_ffi"))]
include!(concat!(env!("OUT_DIR"), "/raylib_ffi.rs"));

#[cfg(feature = "prebuilt_ffi")]
include!("raylib_ffi.rs");
