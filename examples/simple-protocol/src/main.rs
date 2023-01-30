#![allow(dead_code)]

use fp_bindgen::{prelude::*, types::CargoDependency};
use once_cell::sync::Lazy;
use std::collections::{BTreeMap, BTreeSet};

mod types;
use types::*;

fp_import! {
    // No arguments, no return type:
    fn import_void_function();

    // No arguments, empty return type:
    fn import_void_function_empty_return() -> ();

    // No arguments, generic empty result type:
    fn import_void_function_empty_result() -> Result<(), u32>;

    // Passing primitives:
    fn import_primitive_bool(arg: bool) -> bool;
    fn import_primitive_f32(arg: f32) -> f32;
    fn import_primitive_f64(arg: f64) -> f64;
    fn import_primitive_i8(arg: i8) -> i8;
    fn import_primitive_i16(arg: i16) -> i16;
    fn import_primitive_i32(arg: i32) -> i32;
    fn import_primitive_i64(arg: i64) -> i64;
    fn import_primitive_u8(arg: u8) -> u8;
    fn import_primitive_u16(arg: u16) -> u16;
    fn import_primitive_u32(arg: u32) -> u32;
    fn import_primitive_u64(arg: u64) -> u64;

    // Simple "useful" function:
    fn import_get_origin_x() -> i32;
    fn import_get_origin_y() -> i32;

    // Passing arrays:
    fn import_array_u8(arg: [u8; 3]) -> [u8; 3];
    fn import_array_u16(arg: [u16; 3]) -> [u16; 3];
    fn import_array_u32(arg: [u32; 3]) -> [u32; 3];
    fn import_array_i8(arg: [i8; 3]) -> [i8; 3];
    fn import_array_i16(arg: [i16; 3]) -> [i16; 3];
    fn import_array_i32(arg: [i32; 3]) -> [i32; 3];
    fn import_array_f32(arg: [f32; 3]) -> [f32; 3];
    fn import_array_f64(arg: [f64; 3]) -> [f64; 3];

    // Passing strings:
    fn import_string(arg: String) -> String;

    // Multiple arguments:
    fn import_multiple_primitives(arg1: i8, arg2: String) -> i64;

    // Integration with the `time` crate:
    fn import_timestamp(arg: MyDateTime) -> MyDateTime;

    // Passing custom types with flattened properties.
    //
    // See `types/flattening.rs` for more info.
    fn import_fp_flatten(arg: FpFlatten) -> FpFlatten;
    fn import_serde_flatten(arg: SerdeFlatten) -> SerdeFlatten;

    /// Logs a message to the (development) console.
    fn log(message: String);
}

fp_export! {
    // ===============================================================
    // Exported functions that we call as part of the end-to-end tests
    // ===============================================================

    // No arguments, no return type:
    fn export_void_function();

    // Passing primitives:
    fn export_primitive_bool(arg: bool) -> bool;
    fn export_primitive_f32(arg: f32) -> f32;
    fn export_primitive_f64(arg: f64) -> f64;
    fn export_primitive_i8(arg: i8) -> i8;
    fn export_primitive_i16(arg: i16) -> i16;
    fn export_primitive_i32(arg: i32) -> i32;
    fn export_primitive_i64(arg: i64) -> i64;
    fn export_primitive_u8(arg: u8) -> u8;
    fn export_primitive_u16(arg: u16) -> u16;
    fn export_primitive_u32(arg: u32) -> u32;
    fn export_primitive_u64(arg: u64) -> u64;

    // Simple "useful" function:
    fn export_compute_distance(x: i32, y: i32) -> u32;

    // Passing arrays:
    fn export_array_u8(arg: [u8; 3]) -> [u8; 3];
    fn export_array_u16(arg: [u16; 3]) -> [u16; 3];
    fn export_array_u32(arg: [u32; 3]) -> [u32; 3];
    fn export_array_i8(arg: [i8; 3]) -> [i8; 3];
    fn export_array_i16(arg: [i16; 3]) -> [i16; 3];
    fn export_array_i32(arg: [i32; 3]) -> [i32; 3];
    fn export_array_f32(arg: [f32; 3]) -> [f32; 3];
    fn export_array_f64(arg: [f64; 3]) -> [f64; 3];

    // Passing strings:
    fn export_string(arg: String) -> String;

    // Multiple arguments:
    fn export_multiple_primitives(arg1: i8, arg2: String) -> i64;

    // Integration with the `time` crate:
    fn export_timestamp(arg: MyDateTime) -> MyDateTime;

    // Passing custom types with flattened properties.
    //
    // See `types/flattening.rs` for more info.
    fn export_fp_flatten(arg: FpFlatten) -> FpFlatten;
    fn export_serde_flatten(arg: SerdeFlatten) -> SerdeFlatten;

    /// Called on the plugin to give it a chance to initialize.
    fn init();
}

const VERSION: &str = "0.1.0";
const AUTHORS: &str = r#"["kajacx <kajacx@gmail.com>", "Fiberplane <info@fiberplane.com>"]"#;
const NAME: &str = "simple-bindings";

static PLUGIN_DEPENDENCIES: Lazy<BTreeMap<&str, CargoDependency>> = Lazy::new(|| {
    BTreeMap::from([(
        "fp-bindgen-support",
        CargoDependency {
            path: Some("../../../../fp-bindgen-support"),
            features: BTreeSet::from(["async", "guest"]),
            ..CargoDependency::default()
        },
    )])
});

fn main() {
    for bindings_type in [
        BindingsType::RustPlugin(RustPluginConfig {
            name: NAME,
            authors: AUTHORS,
            version: VERSION,
            dependencies: PLUGIN_DEPENDENCIES.clone(),
        }),
        BindingsType::RustWasmerRuntime,
        BindingsType::RustWasmerWasiRuntime,
        BindingsType::TsRuntimeWithExtendedConfig(
            TsExtendedRuntimeConfig::new()
                .with_msgpack_module("https://unpkg.com/@msgpack/msgpack@2.7.2/mod.ts")
                .with_raw_export_wrappers(),
        ),
    ] {
        let output_path = format!("bindings/{}", bindings_type);

        fp_bindgen!(BindingConfig {
            bindings_type,
            path: &output_path,
        });
        println!("Generated bindings written to `{}/`.", output_path);
    }
}
