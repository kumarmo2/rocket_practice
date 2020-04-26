use crate::states::runtime::RuntimeWrapper;

use tokio::runtime::Runtime;

use rocket::State;

use std::sync::{Arc, Mutex};

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/dummy")]
pub fn dummy(rw: State<RuntimeWrapper>) -> &'static str {
    method(&rw);
    "kumarmo2"
}

fn method(rt_wrapper: &RuntimeWrapper) {
    let clone = Arc::clone(rt_wrapper);
    let mut rt = clone.lock().unwrap();
    // TODO: remove this dummy endpoint after integrating the
    // rabbitMq client in appropriate end-point.
    rt.block_on(async move {
        println!("inside the async block");
    });
    println!("after block on");
}
