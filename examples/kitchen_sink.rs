#![no_main]

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
