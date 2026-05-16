use wasmtriggers_macros::init_function;

#[init_function]
extern "C" fn should_fail() {}
