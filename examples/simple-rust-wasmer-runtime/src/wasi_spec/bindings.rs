use super::types::*;
use fp_bindgen_support::{
    common::{abi::WasmAbi, mem::FatPtr},
    host::{
        errors::{InvocationError, RuntimeError},
        mem::{
            deserialize_from_slice, export_to_guest, export_to_guest_raw, import_from_guest,
            import_from_guest_raw, serialize_to_vec,
        },
        r#async::{create_future_value, future::ModuleRawFuture, resolve_async_value},
        runtime::RuntimeInstanceData,
    },
};
use std::cell::RefCell;
use wasmer::{imports, Function, ImportObject, Instance, Module, Store, WasmerEnv};

#[derive(Clone)]
pub struct Runtime {
    instance: Instance,
    env: RuntimeInstanceData,
}

impl Runtime {
    pub fn new(wasm_module: impl AsRef<[u8]>) -> Result<Self, RuntimeError> {
        let store = Self::default_store();
        let module = Module::new(&store, wasm_module)?;
        let mut env = RuntimeInstanceData::default();
        let mut wasi_env = wasmer_wasi::WasiState::new("fp").finalize().unwrap();
        let mut import_object = wasi_env.import_object(&module).unwrap();
        let namespace = create_import_object(module.store(), &env);
        import_object.register("fp", namespace);
        let instance = Instance::new(&module, &import_object).unwrap();
        env.init_with_instance(&instance).unwrap();
        Ok(Self { instance, env })
    }

    #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
    fn default_store() -> wasmer::Store {
        let compiler = wasmer::Cranelift::default();
        let engine = wasmer::Universal::new(compiler).engine();
        Store::new(&engine)
    }

    #[cfg(not(any(target_arch = "arm", target_arch = "aarch64")))]
    fn default_store() -> wasmer::Store {
        let compiler = wasmer::Singlepass::default();
        let engine = wasmer::Universal::new(compiler).engine();
        Store::new(&engine)
    }

    pub fn export_primitive_bool(&self, arg: bool) -> Result<bool, InvocationError> {
        let result = self.export_primitive_bool_raw(arg);
        result
    }
    pub fn export_primitive_bool_raw(&self, arg: bool) -> Result<bool, InvocationError> {
        let function = self
            .instance
            .exports
            .get_native_function::<<bool as WasmAbi>::AbiType, <bool as WasmAbi>::AbiType>(
                "__fp_gen_export_primitive_bool",
            )
            .map_err(|_| {
                InvocationError::FunctionNotExported("__fp_gen_export_primitive_bool".to_owned())
            })?;
        let result = function.call(arg.to_abi())?;
        let result = WasmAbi::from_abi(result);
        Ok(result)
    }

    pub fn export_primitive_f32(&self, arg: f32) -> Result<f32, InvocationError> {
        let result = self.export_primitive_f32_raw(arg);
        result
    }
    pub fn export_primitive_f32_raw(&self, arg: f32) -> Result<f32, InvocationError> {
        let function = self
            .instance
            .exports
            .get_native_function::<<f32 as WasmAbi>::AbiType, <f32 as WasmAbi>::AbiType>(
                "__fp_gen_export_primitive_f32",
            )
            .map_err(|_| {
                InvocationError::FunctionNotExported("__fp_gen_export_primitive_f32".to_owned())
            })?;
        let result = function.call(arg.to_abi())?;
        let result = WasmAbi::from_abi(result);
        Ok(result)
    }

    pub fn export_primitive_f64(&self, arg: f64) -> Result<f64, InvocationError> {
        let result = self.export_primitive_f64_raw(arg);
        result
    }
    pub fn export_primitive_f64_raw(&self, arg: f64) -> Result<f64, InvocationError> {
        let function = self
            .instance
            .exports
            .get_native_function::<<f64 as WasmAbi>::AbiType, <f64 as WasmAbi>::AbiType>(
                "__fp_gen_export_primitive_f64",
            )
            .map_err(|_| {
                InvocationError::FunctionNotExported("__fp_gen_export_primitive_f64".to_owned())
            })?;
        let result = function.call(arg.to_abi())?;
        let result = WasmAbi::from_abi(result);
        Ok(result)
    }

    pub fn export_primitive_i16(&self, arg: i16) -> Result<i16, InvocationError> {
        let result = self.export_primitive_i16_raw(arg);
        result
    }
    pub fn export_primitive_i16_raw(&self, arg: i16) -> Result<i16, InvocationError> {
        let function = self
            .instance
            .exports
            .get_native_function::<<i16 as WasmAbi>::AbiType, <i16 as WasmAbi>::AbiType>(
                "__fp_gen_export_primitive_i16",
            )
            .map_err(|_| {
                InvocationError::FunctionNotExported("__fp_gen_export_primitive_i16".to_owned())
            })?;
        let result = function.call(arg.to_abi())?;
        let result = WasmAbi::from_abi(result);
        Ok(result)
    }

    pub fn export_primitive_i32(&self, arg: i32) -> Result<i32, InvocationError> {
        let result = self.export_primitive_i32_raw(arg);
        result
    }
    pub fn export_primitive_i32_raw(&self, arg: i32) -> Result<i32, InvocationError> {
        let function = self
            .instance
            .exports
            .get_native_function::<<i32 as WasmAbi>::AbiType, <i32 as WasmAbi>::AbiType>(
                "__fp_gen_export_primitive_i32",
            )
            .map_err(|_| {
                InvocationError::FunctionNotExported("__fp_gen_export_primitive_i32".to_owned())
            })?;
        let result = function.call(arg.to_abi())?;
        let result = WasmAbi::from_abi(result);
        Ok(result)
    }

    pub fn export_primitive_i64(&self, arg: i64) -> Result<i64, InvocationError> {
        let result = self.export_primitive_i64_raw(arg);
        result
    }
    pub fn export_primitive_i64_raw(&self, arg: i64) -> Result<i64, InvocationError> {
        let function = self
            .instance
            .exports
            .get_native_function::<<i64 as WasmAbi>::AbiType, <i64 as WasmAbi>::AbiType>(
                "__fp_gen_export_primitive_i64",
            )
            .map_err(|_| {
                InvocationError::FunctionNotExported("__fp_gen_export_primitive_i64".to_owned())
            })?;
        let result = function.call(arg.to_abi())?;
        let result = WasmAbi::from_abi(result);
        Ok(result)
    }

    pub fn export_primitive_i8(&self, arg: i8) -> Result<i8, InvocationError> {
        let result = self.export_primitive_i8_raw(arg);
        result
    }
    pub fn export_primitive_i8_raw(&self, arg: i8) -> Result<i8, InvocationError> {
        let function = self
            .instance
            .exports
            .get_native_function::<<i8 as WasmAbi>::AbiType, <i8 as WasmAbi>::AbiType>(
                "__fp_gen_export_primitive_i8",
            )
            .map_err(|_| {
                InvocationError::FunctionNotExported("__fp_gen_export_primitive_i8".to_owned())
            })?;
        let result = function.call(arg.to_abi())?;
        let result = WasmAbi::from_abi(result);
        Ok(result)
    }

    pub fn export_primitive_u16(&self, arg: u16) -> Result<u16, InvocationError> {
        let result = self.export_primitive_u16_raw(arg);
        result
    }
    pub fn export_primitive_u16_raw(&self, arg: u16) -> Result<u16, InvocationError> {
        let function = self
            .instance
            .exports
            .get_native_function::<<u16 as WasmAbi>::AbiType, <u16 as WasmAbi>::AbiType>(
                "__fp_gen_export_primitive_u16",
            )
            .map_err(|_| {
                InvocationError::FunctionNotExported("__fp_gen_export_primitive_u16".to_owned())
            })?;
        let result = function.call(arg.to_abi())?;
        let result = WasmAbi::from_abi(result);
        Ok(result)
    }

    pub fn export_primitive_u32(&self, arg: u32) -> Result<u32, InvocationError> {
        let result = self.export_primitive_u32_raw(arg);
        result
    }
    pub fn export_primitive_u32_raw(&self, arg: u32) -> Result<u32, InvocationError> {
        let function = self
            .instance
            .exports
            .get_native_function::<<u32 as WasmAbi>::AbiType, <u32 as WasmAbi>::AbiType>(
                "__fp_gen_export_primitive_u32",
            )
            .map_err(|_| {
                InvocationError::FunctionNotExported("__fp_gen_export_primitive_u32".to_owned())
            })?;
        let result = function.call(arg.to_abi())?;
        let result = WasmAbi::from_abi(result);
        Ok(result)
    }

    pub fn export_primitive_u64(&self, arg: u64) -> Result<u64, InvocationError> {
        let result = self.export_primitive_u64_raw(arg);
        result
    }
    pub fn export_primitive_u64_raw(&self, arg: u64) -> Result<u64, InvocationError> {
        let function = self
            .instance
            .exports
            .get_native_function::<<u64 as WasmAbi>::AbiType, <u64 as WasmAbi>::AbiType>(
                "__fp_gen_export_primitive_u64",
            )
            .map_err(|_| {
                InvocationError::FunctionNotExported("__fp_gen_export_primitive_u64".to_owned())
            })?;
        let result = function.call(arg.to_abi())?;
        let result = WasmAbi::from_abi(result);
        Ok(result)
    }

    pub fn export_primitive_u8(&self, arg: u8) -> Result<u8, InvocationError> {
        let result = self.export_primitive_u8_raw(arg);
        result
    }
    pub fn export_primitive_u8_raw(&self, arg: u8) -> Result<u8, InvocationError> {
        let function = self
            .instance
            .exports
            .get_native_function::<<u8 as WasmAbi>::AbiType, <u8 as WasmAbi>::AbiType>(
                "__fp_gen_export_primitive_u8",
            )
            .map_err(|_| {
                InvocationError::FunctionNotExported("__fp_gen_export_primitive_u8".to_owned())
            })?;
        let result = function.call(arg.to_abi())?;
        let result = WasmAbi::from_abi(result);
        Ok(result)
    }

    pub fn export_void_function(&self) -> Result<(), InvocationError> {
        let result = self.export_void_function_raw();
        result
    }
    pub fn export_void_function_raw(&self) -> Result<(), InvocationError> {
        let function = self
            .instance
            .exports
            .get_native_function::<(), ()>("__fp_gen_export_void_function")
            .map_err(|_| {
                InvocationError::FunctionNotExported("__fp_gen_export_void_function".to_owned())
            })?;
        let result = function.call()?;
        let result = WasmAbi::from_abi(result);
        Ok(result)
    }

    /// Called on the plugin to give it a chance to initialize.
    pub fn init(&self) -> Result<(), InvocationError> {
        let result = self.init_raw();
        result
    }
    pub fn init_raw(&self) -> Result<(), InvocationError> {
        let function = self
            .instance
            .exports
            .get_native_function::<(), ()>("__fp_gen_init")
            .map_err(|_| InvocationError::FunctionNotExported("__fp_gen_init".to_owned()))?;
        let result = function.call()?;
        let result = WasmAbi::from_abi(result);
        Ok(result)
    }
}

fn create_import_object(store: &Store, env: &RuntimeInstanceData) -> wasmer::Exports {
    let mut namespace = wasmer::Exports::new();
    namespace.insert(
        "__fp_host_resolve_async_value",
        Function::new_native_with_env(store, env.clone(), resolve_async_value),
    );
    namespace.insert(
        "__fp_gen_import_primitive_bool",
        Function::new_native_with_env(store, env.clone(), _import_primitive_bool),
    );
    namespace.insert(
        "__fp_gen_import_primitive_f32",
        Function::new_native_with_env(store, env.clone(), _import_primitive_f32),
    );
    namespace.insert(
        "__fp_gen_import_primitive_f64",
        Function::new_native_with_env(store, env.clone(), _import_primitive_f64),
    );
    namespace.insert(
        "__fp_gen_import_primitive_i16",
        Function::new_native_with_env(store, env.clone(), _import_primitive_i16),
    );
    namespace.insert(
        "__fp_gen_import_primitive_i32",
        Function::new_native_with_env(store, env.clone(), _import_primitive_i32),
    );
    namespace.insert(
        "__fp_gen_import_primitive_i64",
        Function::new_native_with_env(store, env.clone(), _import_primitive_i64),
    );
    namespace.insert(
        "__fp_gen_import_primitive_i8",
        Function::new_native_with_env(store, env.clone(), _import_primitive_i8),
    );
    namespace.insert(
        "__fp_gen_import_primitive_u16",
        Function::new_native_with_env(store, env.clone(), _import_primitive_u16),
    );
    namespace.insert(
        "__fp_gen_import_primitive_u32",
        Function::new_native_with_env(store, env.clone(), _import_primitive_u32),
    );
    namespace.insert(
        "__fp_gen_import_primitive_u64",
        Function::new_native_with_env(store, env.clone(), _import_primitive_u64),
    );
    namespace.insert(
        "__fp_gen_import_primitive_u8",
        Function::new_native_with_env(store, env.clone(), _import_primitive_u8),
    );
    namespace.insert(
        "__fp_gen_import_void_function",
        Function::new_native_with_env(store, env.clone(), _import_void_function),
    );
    namespace.insert(
        "__fp_gen_import_void_function_empty_result",
        Function::new_native_with_env(store, env.clone(), _import_void_function_empty_result),
    );
    namespace.insert(
        "__fp_gen_import_void_function_empty_return",
        Function::new_native_with_env(store, env.clone(), _import_void_function_empty_return),
    );
    namespace.insert(
        "__fp_gen_log",
        Function::new_native_with_env(store, env.clone(), _log),
    );
    namespace
}

pub fn _import_primitive_bool(
    env: &RuntimeInstanceData,
    arg: <bool as WasmAbi>::AbiType,
) -> <bool as WasmAbi>::AbiType {
    let arg = WasmAbi::from_abi(arg);
    let result = super::import_primitive_bool(arg);
    result.to_abi()
}

pub fn _import_primitive_f32(
    env: &RuntimeInstanceData,
    arg: <f32 as WasmAbi>::AbiType,
) -> <f32 as WasmAbi>::AbiType {
    let arg = WasmAbi::from_abi(arg);
    let result = super::import_primitive_f32(arg);
    result.to_abi()
}

pub fn _import_primitive_f64(
    env: &RuntimeInstanceData,
    arg: <f64 as WasmAbi>::AbiType,
) -> <f64 as WasmAbi>::AbiType {
    let arg = WasmAbi::from_abi(arg);
    let result = super::import_primitive_f64(arg);
    result.to_abi()
}

pub fn _import_primitive_i16(
    env: &RuntimeInstanceData,
    arg: <i16 as WasmAbi>::AbiType,
) -> <i16 as WasmAbi>::AbiType {
    let arg = WasmAbi::from_abi(arg);
    let result = super::import_primitive_i16(arg);
    result.to_abi()
}

pub fn _import_primitive_i32(
    env: &RuntimeInstanceData,
    arg: <i32 as WasmAbi>::AbiType,
) -> <i32 as WasmAbi>::AbiType {
    let arg = WasmAbi::from_abi(arg);
    let result = super::import_primitive_i32(arg);
    result.to_abi()
}

pub fn _import_primitive_i64(
    env: &RuntimeInstanceData,
    arg: <i64 as WasmAbi>::AbiType,
) -> <i64 as WasmAbi>::AbiType {
    let arg = WasmAbi::from_abi(arg);
    let result = super::import_primitive_i64(arg);
    result.to_abi()
}

pub fn _import_primitive_i8(
    env: &RuntimeInstanceData,
    arg: <i8 as WasmAbi>::AbiType,
) -> <i8 as WasmAbi>::AbiType {
    let arg = WasmAbi::from_abi(arg);
    let result = super::import_primitive_i8(arg);
    result.to_abi()
}

pub fn _import_primitive_u16(
    env: &RuntimeInstanceData,
    arg: <u16 as WasmAbi>::AbiType,
) -> <u16 as WasmAbi>::AbiType {
    let arg = WasmAbi::from_abi(arg);
    let result = super::import_primitive_u16(arg);
    result.to_abi()
}

pub fn _import_primitive_u32(
    env: &RuntimeInstanceData,
    arg: <u32 as WasmAbi>::AbiType,
) -> <u32 as WasmAbi>::AbiType {
    let arg = WasmAbi::from_abi(arg);
    let result = super::import_primitive_u32(arg);
    result.to_abi()
}

pub fn _import_primitive_u64(
    env: &RuntimeInstanceData,
    arg: <u64 as WasmAbi>::AbiType,
) -> <u64 as WasmAbi>::AbiType {
    let arg = WasmAbi::from_abi(arg);
    let result = super::import_primitive_u64(arg);
    result.to_abi()
}

pub fn _import_primitive_u8(
    env: &RuntimeInstanceData,
    arg: <u8 as WasmAbi>::AbiType,
) -> <u8 as WasmAbi>::AbiType {
    let arg = WasmAbi::from_abi(arg);
    let result = super::import_primitive_u8(arg);
    result.to_abi()
}

pub fn _import_void_function(env: &RuntimeInstanceData) {
    let result = super::import_void_function();
}

pub fn _import_void_function_empty_result(env: &RuntimeInstanceData) -> FatPtr {
    let result = super::import_void_function_empty_result();
    export_to_guest(env, &result)
}

pub fn _import_void_function_empty_return(env: &RuntimeInstanceData) {
    let result = super::import_void_function_empty_return();
}

pub fn _log(env: &RuntimeInstanceData, message: FatPtr) {
    let message = import_from_guest::<String>(env, message);
    let result = super::log(message);
}
