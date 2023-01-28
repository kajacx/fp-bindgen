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

fn new_runtime() -> Result<Runtime> {
    let rt = Runtime::new(WASM_BYTES)?;
    rt.init()?;
    Ok(rt)
}
