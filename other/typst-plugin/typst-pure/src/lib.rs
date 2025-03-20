use std::sync::atomic::{AtomicUsize, Ordering};

use wasm_minimal_protocol::*;

initiate_protocol!();

static COUNTER: AtomicUsize = AtomicUsize::new(0);

#[wasm_func]
pub fn count() -> Vec<u8> {
    let count = COUNTER.fetch_add(1, Ordering::SeqCst);
    format!("{}", count).into_bytes()
}
