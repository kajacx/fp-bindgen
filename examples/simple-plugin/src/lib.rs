use bytes::{Bytes, BytesMut};
use serde_bytes::ByteBuf;
use simple_bindings::*;
use std::collections::BTreeMap;
use std::panic;
use time::{macros::datetime, OffsetDateTime};

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
fn export_compute_distance(x: i32, y: i32) -> u32 {
    ((x - import_get_origin_x()).abs() + (y - import_get_origin_y()).abs()) as u32
}

#[fp_export_impl(simple_bindings)]
fn export_array_u8(arg: [u8; 3]) -> [u8; 3] {
    assert_eq!(arg, [1u8, 2u8, 3u8]);
    [1u8, 2u8, 3u8]
}

#[fp_export_impl(simple_bindings)]
fn export_array_u16(arg: [u16; 3]) -> [u16; 3] {
    assert_eq!(arg, [1u16, 2u16, 3u16]);
    [1u16, 2u16, 3u16]
}

#[fp_export_impl(simple_bindings)]
fn export_array_u32(arg: [u32; 3]) -> [u32; 3] {
    assert_eq!(arg, [1u32, 2u32, 3u32]);
    [1u32, 2u32, 3u32]
}

#[fp_export_impl(simple_bindings)]
fn export_array_i8(arg: [i8; 3]) -> [i8; 3] {
    assert_eq!(arg, [1i8, 2i8, 3i8]);
    [1i8, 2i8, 3i8]
}

#[fp_export_impl(simple_bindings)]
fn export_array_i16(arg: [i16; 3]) -> [i16; 3] {
    assert_eq!(arg, [1i16, 2i16, 3i16]);
    [1i16, 2i16, 3i16]
}

#[fp_export_impl(simple_bindings)]
fn export_array_i32(arg: [i32; 3]) -> [i32; 3] {
    assert_eq!(arg, [1i32, 2i32, 3i32]);
    [1i32, 2i32, 3i32]
}

#[fp_export_impl(simple_bindings)]
fn export_array_f32(arg: [f32; 3]) -> [f32; 3] {
    assert_eq!(arg, [1.0f32, 2.0f32, 3.0f32]);
    [1.0f32, 2.0f32, 3.0f32]
}

#[fp_export_impl(simple_bindings)]
fn export_array_f64(arg: [f64; 3]) -> [f64; 3] {
    assert_eq!(arg, [1.0f64, 2.0f64, 3.0f64]);
    [1.0f64, 2.0f64, 3.0f64]
}

#[fp_export_impl(simple_bindings)]
fn export_string(arg: String) -> String {
    assert_eq!(arg, "Hello, plugin!");
    "Hello, world!".to_owned()
}

#[fp_export_impl(simple_bindings)]
fn export_multiple_primitives(arg1: i8, arg2: String) -> i64 {
    assert_eq!(arg1, -8);
    assert_eq!(arg2, "Hello, 🇳🇱!");
    -64
}

#[fp_export_impl(simple_bindings)]
fn export_timestamp(arg: MyDateTime) -> MyDateTime {
    assert_eq!(arg, MyDateTime(datetime!(2022-04-12 19:10 UTC)));
    MyDateTime(datetime!(2022-04-13 12:37 UTC))
}

#[fp_export_impl(simple_bindings)]
fn export_fp_flatten(arg: FpFlatten) -> FpFlatten {
    assert_eq!(
        arg,
        FpFlatten {
            flattened: FlattenedStruct {
                foo: "Hello, 🇳🇱!".to_owned(),
                bar: -64,
            }
        }
    );
    FpFlatten {
        flattened: FlattenedStruct {
            foo: "Hello, 🇩🇪!".to_owned(),
            bar: -64,
        },
    }
}

#[fp_export_impl(simple_bindings)]
fn export_serde_flatten(arg: SerdeFlatten) -> SerdeFlatten {
    assert_eq!(
        arg,
        SerdeFlatten {
            flattened: FlattenedStruct {
                foo: "Hello, 🇳🇱!".to_owned(),
                bar: -64,
            }
        }
    );
    SerdeFlatten {
        flattened: FlattenedStruct {
            foo: "Hello, 🇩🇪!".to_owned(),
            bar: -64,
        },
    }
}

#[fp_export_impl(simple_bindings)]
fn init() {
    init_panic_hook();
    tracing::info!("Simple plugin initialized");
}
