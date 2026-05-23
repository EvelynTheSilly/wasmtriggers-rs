#![no_main]

use std::{slice, str};
use wasmtriggers_core::chat::ChatType;
use wasmtriggers_macros::init_function;
use wasmtriggers_rs::{chat::show_chat_message, chat_message_handler, log::*};

#[init_function]
fn init() {
    debug("debug");
    info("info");
    warn("warn");
    error("error");
}

#[chat_message_handler]
fn chat_message_logger(chat: &ChatType) {
    show_chat_message(format!("{:?}", chat).as_str());
}
