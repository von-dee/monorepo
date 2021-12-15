#[link(wasm_import_module = "w3")]
extern "C" {
    /// Get Abort Arguments
    pub fn __w3_debug_log(ptr: u32, len: u32);
}

#[allow(unused_unsafe)]
pub fn w3_debug_log(msg: &str) {
    let msg_buf = msg.as_bytes();

    unsafe { __w3_debug_log(msg_buf.as_ptr() as u32, msg_buf.len() as u32) };
}