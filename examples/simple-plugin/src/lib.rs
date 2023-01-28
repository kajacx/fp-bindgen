use bytes::{Bytes, BytesMut};
use serde_bytes::ByteBuf;
use simple_bindings::*;
use std::collections::BTreeMap;
use std::panic;

fn init_panic_hook() {
    use std::sync::Once;
    static SET_HOOK: Once = Once::new();
    SET_HOOK.call_once(|| {
        panic::set_hook(Box::new(|info| log(info.to_string())));
    });
}

#[fp_export_impl(simple_bindings)]
fn export_primitive_bool(arg: bool) -> bool {
    arg
}

#[fp_export_impl(simple_bindings)]
fn export_primitive_f32(arg: f32) -> f32 {
    assert!(arg > 3.14);
    assert!(arg < 3.15);
    3.1415926535
}

#[fp_export_impl(simple_bindings)]
fn export_primitive_f64(arg: f64) -> f64 {
    assert!(arg > 2.7182);
    assert!(arg < 2.7183);
    2.718281828459
}

#[fp_export_impl(simple_bindings)]
fn export_primitive_i8(arg: i8) -> i8 {
    assert_eq!(arg, -8);
    -8
}

#[fp_export_impl(simple_bindings)]
fn export_primitive_i16(arg: i16) -> i16 {
    assert_eq!(arg, -16);
    -16
}

#[fp_export_impl(simple_bindings)]
fn export_primitive_i32(arg: i32) -> i32 {
    assert_eq!(arg, -32);
    -32
}

#[fp_export_impl(simple_bindings)]
fn export_primitive_i64(arg: i64) -> i64 {
    assert_eq!(arg, -64);
    -64
}

#[fp_export_impl(simple_bindings)]
fn export_primitive_u8(arg: u8) -> u8 {
    assert_eq!(arg, 8);
    8
}

#[fp_export_impl(simple_bindings)]
fn export_primitive_u16(arg: u16) -> u16 {
    assert_eq!(arg, 16);
    16
}

#[fp_export_impl(simple_bindings)]
fn export_primitive_u32(arg: u32) -> u32 {
    assert_eq!(arg, 32);
    32
}

#[fp_export_impl(simple_bindings)]
fn export_primitive_u64(arg: u64) -> u64 {
    assert_eq!(arg, 64);
    64
}

#[fp_export_impl(simple_bindings)]
fn init() {
    init_panic_hook();
    tracing::info!("Simple plugin initialized");
}
