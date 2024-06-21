use dioxus::dioxus_core;
use std::sync::Arc;

use dioxus::hooks::use_signal;
use dioxus::prelude::{Element, spawn, rsx, Writable};
// use dioxus::prelude::{Element, rsx, Scope, to_owned, UnboundedReceiver, use_coroutine};
use tokio::sync::RwLock;
use tokio::task::spawn_local;

use crate::runtime::frame::Frame;
use crate::vm;
use crate::vm::main::Debugger;

#[derive(Debug, Clone)]
pub struct DebuggerState {
    pub frames: Vec<Arc<RwLock<Frame>>>,
    pub current_frame: Option<Arc<RwLock<Frame>>>,
}

#[derive(Debug, Clone)]
pub enum DebuggerCommand {
    StepOver,
    StepBack,
}

struct Unit;

pub(crate) fn App() -> Element {
    let debugger = Debugger::new();
    let debugger_side = debugger.debugger_side;
    let vm_side = debugger.vm_side;
    spawn_local(async move {
        vm::main::start(Some(vm_side)).await;
        // debugger_side.start().await;
    });
    // tokio::spawn();

    let mut state = use_signal::<Option<Arc<DebuggerState>>>(|| None);

    spawn(async move {
        let mut debugger_side = debugger_side;
        let mut state_receiver = debugger_side.state_receiver;
        while let Some(debugger) = state_receiver.recv().await {
            state.set(Some(debugger));
        }
    });
    
    rsx! {
        "getete"
    }
}