use wasmedge_sys::{Vm, WasmValue};
use wasmedge_types::error::WasmEdgeError;
// use wasmedge_types::wat2wasm;

pub fn fib(fib_num: i32) -> Result<i32, WasmEdgeError> {
    let mut vm = Vm::create(None, None)?;
    let module_name = "extern-module";
    
    let file = "wasm/fibonacci.wasm";
    vm.register_wasm_from_file(module_name, file)?;
    
    let func_name = "fib";
    let result = vm.run_registered_function(module_name, func_name, [WasmValue::from_i32(fib_num)])?;

    Ok(result[0].to_i32())
}