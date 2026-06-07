#![no_main]

use wasmtriggers_core::chat::color::Color;
use wasmtriggers_rs::{
    chat::show_chat_message,
    core::chat::{ChatType, component::literal},
    log::*,
    macros::{chat_message_handler, init_function, on_press, on_release, on_tick},
    title::set_title,
};

#[init_function]
fn init() {
    info("you can initialise here");
}

#[init_function]
fn other_init() {
    info("or do it again in here");
}

#[chat_message_handler]
fn funny_number_detector(chat: &ChatType) {
    if chat.get_message().contains("67") {
        show_chat_message(literal("funny number spotted").with_color(Color::new(227, 28, 121)));
        set_title(literal("SIX SEVENNNNNN").with_color(Color::new(67, 67, 67)));
    }
}

#[unsafe(no_mangle)]
#[allow(unused)]
pub extern "C" fn on_keyboard_input(_action: u32, _key_ptr: u32, _key_len: u32) {}

#[on_press(a)]
fn handle_a_press() {}

#[on_release(a)]
fn handle_a_release() {}

#[on_press]
fn handle_any_press(_key: &str) {}

#[on_tick]
fn on_every_tick(_tick: u32) {
    // this runs every tick
}
