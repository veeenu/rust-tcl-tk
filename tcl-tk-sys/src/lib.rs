#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {}
#[link(name = "Cocoa", kind = "framework")]
extern "C" {}
#[link(name = "Carbon", kind = "framework")]
extern "C" {}
#[link(name = "IOKit", kind = "framework")]
extern "C" {}
#[link(name = "QuartzCore", kind = "framework")]
extern "C" {}
#[link(name = "Security", kind = "framework")]
extern "C" {}
#[link(name = "z")]
extern "C" {}

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
