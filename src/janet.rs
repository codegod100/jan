use std::ffi::{c_char, c_int};

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
}

impl JanetRuntime {
    pub fn new() -> Result<Self, &'static str> {
        unsafe {
            if janet_init() != 0 {
                return Err("Failed to initialize Janet runtime");
            }
        }
        Ok(JanetRuntime { _initialized: true })
    }

    pub fn eval(&self, code: &str) -> Result<(), &'static str> {
        let c_code = std::ffi::CString::new(code).map_err(|_| "Invalid string")?;
        let source_path = std::ffi::CString::new("repl").map_err(|_| "Invalid source path")?;
        unsafe {
            let env = janet_core_env();
            let mut result = Janet { _private: [0; 16] };
            let ret = janet_dostring(env, c_code.as_ptr(), source_path.as_ptr(), &mut result);
            if ret != 0 {
                return Err("Janet evaluation failed");
            }
        }
        Ok(())
    }
}

impl Drop for JanetRuntime {
    fn drop(&mut self) {
        if self._initialized {
            unsafe {
                janet_deinit();
            }
        }
    }
}