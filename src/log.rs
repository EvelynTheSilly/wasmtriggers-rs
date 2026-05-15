#[link(wasm_import_module = "logging")]
unsafe extern "C" {
    #[link_name = "debug"]
    unsafe fn extern_debug(ptr: u32, len: u32);

    #[link_name = "info"]
    unsafe fn extern_info(ptr: u32, len: u32);

    #[link_name = "warn"]
    unsafe fn extern_warn(ptr: u32, len: u32);

    #[link_name = "error"]
    unsafe fn extern_error(ptr: u32, len: u32);
}

pub fn debug(msg: &str) {
    unsafe {
        extern_debug(msg.as_ptr() as u32, msg.len() as u32);
    }
}

pub fn info(msg: &str) {
    unsafe {
        extern_info(msg.as_ptr() as u32, msg.len() as u32);
    }
}

pub fn warn(msg: &str) {
    unsafe {
        extern_warn(msg.as_ptr() as u32, msg.len() as u32);
    }
}

pub fn error(msg: &str) {
    unsafe {
        extern_error(msg.as_ptr() as u32, msg.len() as u32);
    }
}
