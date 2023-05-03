use wasmtime_provider::WasmtimeEngineProvider; // Or Wasm3EngineProvider
use wapc::WapcHost;
use std::error::Error;
pub fn main() -> Result<(), Box<dyn Error>> {

  // Sample host callback that prints the operation a WASM module requested.
  let host_callback = |id: u64, bd: &str, ns: &str, op: &str, payload: &[u8]| {
    println!("Guest {} invoked '{}->{}:{}' with a {} byte payload",
    id, bd, ns, op, payload.len());
    // Return success with zero-byte payload.
    Ok(vec![])
  };

  let file = "target/wasm32-unknown-unknown/debug/examples/add_functions.wasm";
  let module_bytes = std::fs::read(file)?;

  let engine = WasmtimeEngineProvider::new(&module_bytes, None)?;
  let host = WapcHost::new(Box::new(engine), Some(Box::new(host_callback)))?;

  let res = host.call("ping", b"payload bytes")?;
  assert_eq!(res, b"payload bytes");
  let res_z = host.call("z",b"")?;
  assert_eq!(res_z,b"z");
  Ok(())
}