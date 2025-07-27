use jan_core::JanetRuntime;

pub fn test_basic_runtime() {
    println!("Testing basic Janet runtime creation...");
    
    match JanetRuntime::new() {
        Ok(runtime) => {
            println!("✓ Runtime created successfully");
            
            match runtime.eval("(print \"Hello\")") {
                Ok(_) => println!("✓ Basic eval works"),
                Err(e) => println!("✗ Basic eval failed: {}", e),
            }
        }
        Err(e) => println!("✗ Runtime creation failed: {}", e),
    }
}