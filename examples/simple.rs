#![no_main]

use wasmtriggers_rs::log::{debug, error, info, warn};

#[unsafe(no_mangle)]
pub extern "C" fn init_handler() {
    debug("debug");
    info("info");
    warn("warn");
    error("error");
}
