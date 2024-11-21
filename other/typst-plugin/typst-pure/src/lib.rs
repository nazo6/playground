use wasm_minimal_protocol::*;

initiate_protocol!();

static mut COUNTER: u32 = 0;

#[wasm_func]
pub fn count() -> Vec<u8> {
    unsafe {
        COUNTER += 1;
    }
    format!("{}", unsafe { COUNTER }).into_bytes()
}
