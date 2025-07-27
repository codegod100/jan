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
    janet_shared!("(def fibonacci (fn [n] (if (< n 2) n (+ (fibonacci (- n 1)) (fibonacci (- n 2))))))");
    janet_shared!("(print \"  Fibonacci sequence:\")");
    janet_shared!("(for i 0 10 (print \"    fib(\" i \") = \" (fibonacci i)))");
    
    println!("\n5. Complex Janet data structures:");
    janet_shared!{
        (def person @{
            :name "Alice"
            :age 30
            :skills ["Janet" "Rust" "Lisp"]
        })
    };
    janet_shared!("(print \"  Person: \" person)");
    janet_shared!("(print \"  Skills: \" (person :skills))");
    
    println!("\n🎉 Janet procedural macro integration successful!");
}