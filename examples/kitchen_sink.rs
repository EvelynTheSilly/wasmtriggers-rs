#![no_main]

use std::{slice, str};
use wasmtriggers_macros::init_function;
use wasmtriggers_rs::{chat_message_handler, log::*};

#[init_function]
fn init_handler() {
    debug("debug");
    info("info");
    warn("warn");
    error("error");
}
/*
pub extern "C" fn chat_message_handler(ptr1: u32, len1: u32, ptr2: u32, len2: u32) {
    unsafe {
        let name =
            str::from_utf8_unchecked(slice::from_raw_parts(ptr1 as *const u8, len1 as usize));
        let message =
            str::from_utf8_unchecked(slice::from_raw_parts(ptr2 as *const u8, len2 as usize));
        __internal_chat_message_handler(name, message);
    }
}

fn __internal_chat_message_handler(name: &str, msg: &str) {
    unsafe {
        info("chat message:");
        info(name);
        info(msg);
    }
}
*/

#[chat_message_handler]
fn yamum() {}
