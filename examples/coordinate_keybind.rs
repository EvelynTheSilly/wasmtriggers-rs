#![no_main]

//! this example shows how to implement a system where at a press of a button, the users coordinates can be shared in chat.
//! this can be useful to quickly share your location without needing to copy it down

use wasmtriggers_rs::{chat::send_chat_message, player::get_player_block_pos};

#[unsafe(no_mangle)]
pub extern "C" fn on_keypress_u() {
    let pos = get_player_block_pos();
    if let Some(pos) = pos {
        send_chat_message(format!("x: {}, y: {}, z: {}", pos.0, pos.1, pos.2).as_str());
    }
}
