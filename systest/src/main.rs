#![allow(bad_style, improper_ctypes)]
#![allow(function_casts_as_integer)]
#![allow(clippy::all)]

extern crate jni_sys;
extern crate libc;

use jni_sys::*;

include!(concat!(env!("OUT_DIR"), "/all.rs"));
