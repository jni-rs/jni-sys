#![allow(bad_style, improper_ctypes)]

extern crate jni_sys;
extern crate libc;

use jni_sys::*;

include!(concat!(env!("OUT_DIR"), "/all.rs"));
