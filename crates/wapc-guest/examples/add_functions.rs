use wapc_guest as wapc;

#[no_mangle]
pub fn wapc_init() {
  wapc::register_function("ping", ping);
}

fn ping(msg: &[u8]) -> wapc::CallResult {
  wapc::register_function("z",z);
  wapc::console_log(&format!(
    "IN_WASM: Received request for `ping` operation with payload : {}",
    std::str::from_utf8(msg).unwrap()
  ));
  let _res = wapc::host_call("binding", "sample:namespace", "pong", msg)?;
  Ok(msg.to_vec())
}
fn z(msg: &[u8]) -> wapc::CallResult{
  Ok(b"z".to_vec())
}
fn main(){

}