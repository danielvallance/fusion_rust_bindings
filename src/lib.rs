#![no_std]
#![feature(prelude_2024)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use core::{
    clone::Clone, concat, default::Default, env, fmt::Debug, include, marker::Copy,
    prelude::rust_2024::derive,
};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
