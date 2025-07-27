mod janet;

use janet::JanetRuntime;

fn main() {
    let runtime = JanetRuntime::new().expect("Failed to initialize Janet");
    
    runtime.eval("(print \"Hello from Janet!\")").expect("Failed to run Janet code");
    
    println!("Janet integration successful!");
}
