use tokio::runtime::{self, Builder, Runtime};
use std::ops::{Deref, DerefMut};
use std::sync::{Mutex, Arc};

pub struct RuntimeWrapper{
    runtime: Arc<Mutex<Runtime>>
}

impl Deref for RuntimeWrapper {
    type Target = Arc<Mutex<Runtime>>;
    fn deref(&self) -> &Self::Target {
        &self.runtime
    }
}

impl RuntimeWrapper {
    pub fn new() -> Self {
        println!("new runtime is being created");
        RuntimeWrapper {
            runtime: Arc::new(Mutex::new(Builder::new().threaded_scheduler().build().expect("could not create tokio runtime")))
        }
    }
}