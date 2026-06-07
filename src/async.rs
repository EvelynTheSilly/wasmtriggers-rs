use wasmtriggers_core::executor::{ASYNC_EXECUTOR, futures::wait_next_gametick};
use wasmtriggers_macros::{init_function, on_tick};

use crate::log::info;

#[init_function]
pub fn async_init() {
    info("initing async");
    ASYNC_EXECUTOR.spawn(async {
        info("spawned");
        wait_next_gametick().await;
        info("yamum");
        loop {
            for _ in 0..20 {
                wait_next_gametick().await;
            }
            info("every second");
        }
    });
}

#[on_tick]
pub fn on_tick_async(_: u32) {
    ASYNC_EXECUTOR.poll_all();
}
