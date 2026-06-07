use wasmtriggers_core::executor::{ASYNC_EXECUTOR, futures::wait_next_gametick};

use crate::log::info;

#[unsafe(no_mangle)]
pub extern "C" fn init_handler__async() {
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

#[unsafe(no_mangle)]
pub extern "C" fn on_tick__async() {
    ASYNC_EXECUTOR.poll_all();
}
