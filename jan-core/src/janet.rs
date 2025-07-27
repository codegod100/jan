use std::ffi::{c_char, c_int};
use std::sync::{Mutex, OnceLock};

static JANET_INITIALIZED: OnceLock<Mutex<bool>> = OnceLock::new();

#[repr(C)]
pub struct Janet {
    _private: [u8; 16], // Janet is a union, typically 8-16 bytes
}

#[repr(C)]
pub struct JanetTable {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn janet_init() -> c_int;
    pub fn janet_deinit();
    pub fn janet_dostring(env: *mut JanetTable, str: *const c_char, sourcePath: *const c_char, out: *mut Janet) -> c_int;
    pub fn janet_core_env() -> *mut JanetTable;
}

pub struct JanetRuntime {
    _initialized: bool,
    env: *mut JanetTable,
}

// SAFETY: Janet runtime is designed to be used from a single thread
// We ensure thread safety through the Mutex in the shared module
unsafe impl Send for JanetRuntime {}
unsafe impl Sync for JanetRuntime {}

impl JanetRuntime {
    pub fn new() -> Result<Self, &'static str> {
        let init_guard = JANET_INITIALIZED.get_or_init(|| Mutex::new(false));
        let mut initialized = init_guard.lock().unwrap();
        
        if !*initialized {
            unsafe {
                if janet_init() != 0 {
                    return Err("Failed to initialize Janet runtime");
                }
            }
            *initialized = true;
        }
        
        unsafe {
            let env = janet_core_env();
            if env.is_null() {
                return Err("Failed to get Janet core environment");
            }
            
            Ok(JanetRuntime { 
                _initialized: false, // We don't own the global init
                env
            })
        }
    }

    pub fn eval(&self, code: &str) -> Result<(), &'static str> {
        let c_code = std::ffi::CString::new(code).map_err(|_| "Invalid string")?;
        let source_path = std::ffi::CString::new("repl").map_err(|_| "Invalid source path")?;
        unsafe {
            let mut result = Janet { _private: [0; 16] };
            let ret = janet_dostring(self.env, c_code.as_ptr(), source_path.as_ptr(), &mut result);
            if ret != 0 {
                return Err("Janet evaluation failed");
            }
        }
        Ok(())
    }
}

impl Drop for JanetRuntime {
    fn drop(&mut self) {
        // We don't call janet_deinit() here since multiple runtimes
        // share the global Janet state. Janet will clean up on process exit.
    }
}