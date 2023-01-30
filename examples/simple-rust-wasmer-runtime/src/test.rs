#[cfg(not(feature = "wasi"))]
use crate::spec::bindings::Runtime;
#[cfg(not(feature = "wasi"))]
use crate::spec::types::*;
#[cfg(feature = "wasi")]
use crate::wasi_spec::bindings::Runtime;
#[cfg(feature = "wasi")]
use crate::wasi_spec::types::*;
use anyhow::Result;
use bytes::Bytes;
use serde_bytes::ByteBuf;
use std::collections::BTreeMap;
use time::{macros::datetime, OffsetDateTime};

#[cfg(not(feature = "wasi"))]
const WASM_BYTES: &'static [u8] =
    include_bytes!("../../simple-plugin/target/wasm32-unknown-unknown/debug/simple_plugin.wasm");
#[cfg(feature = "wasi")]
const WASM_BYTES: &'static [u8] =
    include_bytes!("../../simple-plugin/target/wasm32-wasi/debug/simple_plugin.wasm");

#[test]
fn primitives() -> Result<()> {
    let rt = new_runtime()?;

    assert_eq!(rt.export_primitive_bool(true)?, true);
    assert_eq!(rt.export_primitive_bool(false)?, false);

    assert_eq!(rt.export_primitive_u8(8)?, 8);
    assert_eq!(rt.export_primitive_u16(16)?, 16);
    assert_eq!(rt.export_primitive_u32(32)?, 32);
    assert_eq!(rt.export_primitive_u64(64)?, 64);
    assert_eq!(rt.export_primitive_i8(-8)?, -8);
    assert_eq!(rt.export_primitive_i16(-16)?, -16);
    assert_eq!(rt.export_primitive_i32(-32)?, -32);
    assert_eq!(rt.export_primitive_i64(-64)?, -64);

    assert_eq!(rt.export_primitive_f32(3.1415926535)?, 3.1415926535);
    assert_eq!(
        rt.export_primitive_f64(2.718281828459f64)?,
        2.718281828459f64
    );
    Ok(())
}

#[test]
fn point_distance() -> Result<()> {
    let rt = new_runtime()?;

    // "origin" is at 4, 6
    assert_eq!(rt.export_compute_distance(5, 6)?, 1);
    assert_eq!(rt.export_compute_distance(2, 8)?, 4);

    Ok(())
}

#[test]
fn arrays() -> Result<()> {
    let rt = new_runtime()?;

    assert_eq!(rt.export_array_u8([1u8, 2u8, 3u8])?, [1u8, 2u8, 3u8]);
    assert_eq!(rt.export_array_u16([1u16, 2u16, 3u16])?, [1u16, 2u16, 3u16]);
    assert_eq!(rt.export_array_u32([1u32, 2u32, 3u32])?, [1u32, 2u32, 3u32]);
    assert_eq!(rt.export_array_i8([1i8, 2i8, 3i8])?, [1i8, 2i8, 3i8]);
    assert_eq!(rt.export_array_i16([1i16, 2i16, 3i16])?, [1i16, 2i16, 3i16]);
    assert_eq!(rt.export_array_i32([1i32, 2i32, 3i32])?, [1i32, 2i32, 3i32]);
    assert_eq!(rt.export_array_f32([1f32, 2f32, 3f32])?, [1f32, 2f32, 3f32]);
    assert_eq!(rt.export_array_f64([1f64, 2f64, 3f64])?, [1f64, 2f64, 3f64]);
    Ok(())
}

#[test]
fn string() -> Result<()> {
    let rt = new_runtime()?;
    assert_eq!(
        rt.export_string("Hello, plugin!".to_string())?,
        "Hello, world!"
    );

    Ok(())
}

#[test]
fn timestamp() -> Result<()> {
    let rt = new_runtime()?;
    assert_eq!(
        rt.export_timestamp(MyDateTime(datetime!(2022-04-12 19:10 UTC)))?,
        MyDateTime(datetime!(2022-04-13 12:37 UTC))
    );
    Ok(())
}

#[test]
fn property_renaming() -> Result<()> {
    let rt = new_runtime()?;

    assert_eq!(
        rt.export_fp_flatten(FpFlatten {
            flattened: FlattenedStruct {
                foo: "Hello, 🇳🇱!".to_owned(),
                bar: -64,
            }
        })?,
        FpFlatten {
            flattened: FlattenedStruct {
                foo: "Hello, 🇩🇪!".to_owned(),
                bar: -64,
            },
        }
    );

    assert_eq!(
        rt.export_serde_flatten(SerdeFlatten {
            flattened: FlattenedStruct {
                foo: "Hello, 🇳🇱!".to_owned(),
                bar: -64,
            }
        })?,
        SerdeFlatten {
            flattened: FlattenedStruct {
                foo: "Hello, 🇩🇪!".to_owned(),
                bar: -64,
            },
        }
    );

    Ok(())
}

fn new_runtime() -> Result<Runtime> {
    let rt = Runtime::new(WASM_BYTES)?;
    rt.init()?;
    Ok(rt)
}
