#![allow(bad_style)]

extern crate jni_sys;

use jni_sys::*;

include!(concat!(env!("OUT_DIR"), "/all.rs"));
