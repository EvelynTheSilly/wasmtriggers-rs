#![no_main]

use std::slice;

use wasmtriggers_core::chat::ChatType;
use wasmtriggers_macros::init_function;
use wasmtriggers_rs::{
    chat::{send_chat_message, show_chat_message},
    chat_message_handler,
    log::*,
};

#[init_function]
fn init() {
    debug("debug");
    info("info");
    warn("warn");
    error("error");
}

#[chat_message_handler]
fn funny_number_detector(chat: &ChatType) {
    if chat.get_message().contains("67") || chat.get_message().contains("69") {
        show_chat_message("funny number detected!");
        send_chat_message("aaaaa so funny hahahahahah");
    }
}

#[unsafe(no_mangle)]
#[allow(unused)]
pub extern "C" fn on_keyboard_input(action: u32, key_ptr: u32, key_len: u32) {
    // this happens on any keyboard input
}

#[unsafe(no_mangle)]
pub extern "C" fn on_keypress_a() {
    // this happens when a is pressed
}

#[unsafe(no_mangle)]
pub extern "C" fn on_keyrelease_a() {
    // this happens when a is released
}

#[unsafe(no_mangle)]
pub extern "C" fn on_keypress(ptr: u32, len: u32) {
    // this happens on any key press event
}
