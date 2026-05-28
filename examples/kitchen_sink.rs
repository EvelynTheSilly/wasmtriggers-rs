#![no_main]

use wasmtriggers_core::chat::{click_event::ClickEvent, color::Color};
use wasmtriggers_rs::{
    chat::show_chat_message,
    core::chat::{ChatType, component::literal},
    log::*,
    macros::{chat_message_handler, init_function},
    title::set_title,
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
    if chat.get_message().contains("67") {
        show_chat_message(literal("funny number spotted", Color::new(227, 28, 121)));
        set_title(literal("SIX SEVENNNNNN", Color::new(67, 67, 67)));
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
pub extern "C" fn on_keypress(_ptr: u32, _len: u32) {
    // this happens on any key press event
}
