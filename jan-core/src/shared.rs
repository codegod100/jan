use std::sync::{Mutex, OnceLock};
use crate::JanetRuntime;

static SHARED_JANET_RUNTIME: OnceLock<Mutex<JanetRuntime>> = OnceLock::new();

pub fn with_shared_runtime<F, R>(f: F) -> Result<R, &'static str>
where
    F: FnOnce(&JanetRuntime) -> Result<R, &'static str>,
{
    let runtime = SHARED_JANET_RUNTIME.get_or_init(|| {
        Mutex::new(JanetRuntime::new().expect("Failed to initialize shared Janet runtime"))
    });
    
    let runtime_guard = runtime.lock().unwrap();
    f(&*runtime_guard)
}

pub fn eval_shared(code: &str) -> Result<(), &'static str> {
    with_shared_runtime(|runtime| runtime.eval(code))
}