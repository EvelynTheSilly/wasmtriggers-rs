#![no_main]
#![feature(str_from_raw_parts)]

use std::str;

use wasmtriggers_macros::init_function;
use wasmtriggers_rs::log::*;

#[init_function]
pub fn init_handler() {
    debug("debug");
    info("info");
    warn("warn");
    error("error");
}

#[unsafe(no_mangle)]
pub extern "C" fn chat_message_handler(ptr: u32, len: u32) {
    unsafe {
        info("chat message:");
        info(str::from_raw_parts(ptr as *const u8, len as usize));
    }
}
