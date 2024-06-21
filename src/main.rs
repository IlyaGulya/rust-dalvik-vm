use tokio::runtime::UnhandledPanic;

use crate::debug::is_debug;
use crate::dex::dex_file;

mod dex;
mod bytecode;
mod runtime;
mod testing;
mod entrypoint;
mod vm;
mod debug;

// #[tokio::main]
fn main() {
    if is_debug() {
        println!("Debug mode enabled. Starting debugger GUI.");
        dioxus::launch(vm::debugger::App);
    } else {
        tokio::runtime::Builder::new_current_thread()
            .unhandled_panic(UnhandledPanic::ShutdownRuntime)
            .build()
            .unwrap()
            .block_on(vm::main::start(None));
    }
}