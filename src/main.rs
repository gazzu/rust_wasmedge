use rust_wasmedge::fibonacci;
use wasmedge_types::error::WasmEdgeError;

//use wasmedge_sys::{Vm, WasmValue};
//use wasmedge_types::wat2wasm;

// use std::error::Error;
// fn main() -> Result<(), Box<dyn Error>> {

fn main() -> Result<(), WasmEdgeError> {

    let fib_num = 8;
    
    /*
    // load wat module version
    let mut vm = Vm::create(None, None)?;
    let module_name = "extern-module";

    let wasm_bytes = wat2wasm(
        br#"
        (module
            (func $fib (param $n i32) (result i32)
              (if (result i32)
                  (i32.lt_s (get_local $n)
                            (i32.const 2))
                  (then (get_local $n))
                  ;; recursive branch spawns _two_ calls to $fib; not ideal
                  (else (i32.add (call $fib (i32.sub (get_local $n)
                                                     (i32.const 1)))
                                 (call $fib (i32.sub (get_local $n)
                                                     (i32.const 2)))))))
          
            (export "fib" (func $fib)))
        "#,
    )?;
    vm.register_wasm_from_bytes(module_name, &wasm_bytes)?;

    let func_name = "fib";
    let result = vm.run_registered_function(module_name, func_name, [WasmValue::from_i32(fib_num)])?;

    println!("fib({}) = {}", fib_num, result[0].to_i32());
    Ok(())
    */
    
    /*
    // load wasm file version
    let mut vm = Vm::create(None, None)?;
    let module_name = "extern-module";

    let file = "wasm/fibonacci.wasm";
    vm.register_wasm_from_file(module_name, file)?;

    let func_name = "fib";
    let result = vm.run_registered_function(module_name, func_name, [WasmValue::from_i32(fib_num)])?;

    println!("fib({}) = {}", fib_num, result[0].to_i32());
    Ok(())
    */


    let number = match fibonacci::fibonacci::fib(fib_num) {
        Ok(number)  => number,
        Err(e) => return Err(e),
    };

    println!("fib({fib_num}) = {number:?}");
    Ok(())

}