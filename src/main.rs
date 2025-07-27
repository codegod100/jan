use jan_core::JanetRuntime;
use jan_macros::{janet, janet_shared};

fn main() {
    println!("=== Janet-Rust Integration Demo ===\n");
    
    println!("1. Direct Janet runtime (maintains state within same instance):");
    let runtime = JanetRuntime::new().expect("Failed to initialize Janet");
    runtime.eval("(defn test-fn [x] (+ x 10))").expect("Failed to define function");
    runtime.eval("(print \"  Result: \" (test-fn 5))").expect("Failed to call function");
    
    println!("\n2. Janet macro - string form (isolated runtime):");
    janet!("(print \"  Hello from string macro!\")");
    
    println!("\n3. Janet macro - token form with AST parsing (isolated runtime):");
    janet!{ (print "  Hello from token macro!") };
    
    println!("\n4. Shared Janet runtime (persistent state across macro calls):");
    janet_shared!("(var counter 0)");
    janet_shared!("(defn increment [] (set counter (+ counter 1)) counter)");
    janet_shared!("(print \"  Counter: \" (increment))");
    janet_shared!("(print \"  Counter: \" (increment))");
    janet_shared!("(print \"  Counter: \" (increment))");
    
    println!("\n5. Complex Janet data structures:");
    janet_shared!("(def nums @[1 2 3 4 5])");
    janet_shared!("(def doubled (map (fn [x] (* x 2)) nums))");
    janet_shared!("(print \"  Original: \" nums)");
    janet_shared!("(print \"  Doubled: \" doubled)");
    
    println!("\n🎉 Janet procedural macro integration successful!");
}