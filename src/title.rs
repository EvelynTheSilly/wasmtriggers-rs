use wasmtriggers_core::chat::message::ChatMessage;

use crate::chat::make_abi_message;

#[link(wasm_import_module = "chat_lib")]
unsafe extern "C" {
    #[link_name = "set_title"]
    unsafe fn extern_set_title(ptr: u32, len: u32);

    #[link_name = "set_sub_title"]
    unsafe fn extern_set_sub_title(ptr: u32, len: u32);
}

pub fn set_title(msg: impl Into<ChatMessage>) {
    let msg: ChatMessage = msg.into();
    let vec = make_abi_message(&msg);

    unsafe {
        extern_set_title(vec.as_ptr() as u32, vec.len() as u32);
    }
}

pub fn set_sub_title(msg: impl Into<ChatMessage>) {
    let msg: ChatMessage = msg.into();
    let vec = make_abi_message(&msg);

    unsafe {
        extern_set_sub_title(vec.as_ptr() as u32, vec.len() as u32);
    }
}
