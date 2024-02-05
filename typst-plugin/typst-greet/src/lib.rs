use wasm_minimal_protocol::*;

initiate_protocol!();

#[wasm_func]
pub fn greet(name: &[u8]) -> Vec<u8> {
    [b"Hello, ", name, b"!"].concat()
}
