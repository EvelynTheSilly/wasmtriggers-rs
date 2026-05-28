use wasmtriggers_core::chat::{color::Color, message::ChatMessage};

use crate::log::info;

#[link(wasm_import_module = "chat_lib")]
unsafe extern "C" {
    #[link_name = "show_chat_message"]
    unsafe fn extern_show_chat_message(ptr: u32, len: u32);
    #[link_name = "send_chat_message"]
    unsafe fn extern_send_chat_message(ptr: u32, len: u32);
    #[link_name = "send_command"]
    unsafe fn extern_send_command(ptr: u32, len: u32);
}

pub fn show_chat_message(msg: impl Into<ChatMessage>) {
    let msg: ChatMessage = msg.into();
    let mut vec = Vec::<AbiChatComponent>::new();

    for component in &msg.components {
        let hover = {
            component.hover.as_ref().map_or((0, 0, 0), |hover| {
                (
                    hover.0.as_bytes().as_ptr() as u32,
                    hover.0.len() as u32,
                    pack_color(&hover.1),
                )
            })
        };
        let on_click = {
            component.on_click.as_ref().map_or((0, 0, 0), |on_click| {
                (
                    on_click.action_id(),
                    on_click.text().as_bytes().as_ptr() as u32,
                    on_click.text().len() as u32,
                )
            })
        };
        vec.push(AbiChatComponent {
            text_pointer: component.text.0.as_bytes().as_ptr() as u32,
            text_len: component.text.0.len() as u32,
            text_color: pack_color(&component.text.1),
            hover_pointer: hover.0,
            hover_len: hover.1,
            hover_color: hover.2,
            click_action: on_click.0,
            click_pointer: on_click.1,
            click_len: on_click.2,
        });
    }
    info(&format!(
        "sending out ptr {} len {}",
        vec.as_ptr() as u32,
        vec.len() as u32
    ));

    unsafe {
        extern_show_chat_message(vec.as_ptr() as u32, vec.len() as u32);
    }
}

pub fn send_chat_message(msg: &str) {
    unsafe {
        extern_send_chat_message(msg.as_ptr() as u32, msg.len() as u32);
    }
}

pub fn send_command(msg: &str) {
    unsafe {
        extern_send_command(msg.as_ptr() as u32, msg.len() as u32);
    }
}

#[repr(C)]
struct AbiChatComponent {
    text_pointer: u32,
    text_len: u32,
    /// packed rbg 8bit per color, last byte ignored
    text_color: u32,
    hover_pointer: u32,
    hover_len: u32,
    /// packed rbg 8bit per color, last byte ignored
    hover_color: u32,
    click_action: u32,
    click_pointer: u32,
    click_len: u32,
}

const fn pack_color(color: &Color) -> u32 {
    let mut acc = 0u32;
    acc += (color.r as u32) << 16;
    acc += (color.g as u32) << 8;
    acc += color.b as u32;
    acc
}
