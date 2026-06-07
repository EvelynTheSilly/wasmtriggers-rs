use wasmtriggers_core::executor::ASYNC_EXECUTOR;
use wasmtriggers_macros::{init_function, on_tick};

#[init_function]
pub fn async_init() {}

#[on_tick]
pub fn on_tick_async(_: u32) {
    ASYNC_EXECUTOR.poll_all();
}
