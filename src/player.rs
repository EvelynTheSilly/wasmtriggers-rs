#[link(wasm_import_module = "player_lib")]
unsafe extern "C" {
    #[link_name = "get_player_block_pos"]
    unsafe fn extern_player_block_pos(bool: *mut i32, x: *mut i32, y: *mut i32, z: *mut i32);
}

pub fn get_player_block_pos() -> Option<(i32, i32, i32)> {
    let mut ret: (i32, i32, i32, i32) = (0, 0, 0, 0);
    unsafe { extern_player_block_pos(&mut ret.0, &mut ret.1, &mut ret.2, &mut ret.3) };
    if ret.0 == 0 {
        None
    } else {
        Some((ret.1, ret.2, ret.3))
    }
}
