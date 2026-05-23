#[link(wasm_import_module = "chat_lib")]
unsafe extern "C" {
    #[link_name = "show_chat_message"]
    unsafe fn extern_show_chat_message(ptr: u32, len: u32);
}

pub fn show_chat_message(msg: &str) {
    unsafe {
        extern_show_chat_message(msg.as_ptr() as u32, msg.len() as u32);
    }
}
