pub mod bindings;
pub mod types;

use bytes::Bytes;
use serde_bytes::ByteBuf;
use types::*;

fn import_void_function() {}
fn import_void_function_empty_result() -> Result<(), u32> {
    Ok(())
}
fn import_void_function_empty_return() -> () {}

fn import_primitive_bool(arg: bool) -> bool {
    todo!()
}
fn import_primitive_f32(arg: f32) -> f32 {
    todo!()
}
fn import_primitive_f64(arg: f64) -> f64 {
    todo!()
}
fn import_primitive_i8(arg: i8) -> i8 {
    todo!()
}
fn import_primitive_i16(arg: i16) -> i16 {
    todo!()
}
fn import_primitive_i32(arg: i32) -> i32 {
    todo!()
}
fn import_primitive_i64(arg: i64) -> i64 {
    todo!()
}
fn import_primitive_u8(arg: u8) -> u8 {
    todo!()
}
fn import_primitive_u16(arg: u16) -> u16 {
    todo!()
}
fn import_primitive_u32(arg: u32) -> u32 {
    todo!()
}
fn import_primitive_u64(arg: u64) -> u64 {
    todo!()
}

fn import_get_origin_x() -> i32 {
    4
}
fn import_get_origin_y() -> i32 {
    6
}

fn log(msg: String) {
    println!("Provider log: {}", msg);
}
