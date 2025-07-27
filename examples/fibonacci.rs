use jan_core::JanetRuntime;
use jan_macros::{janet, janet_shared};

fn main() {
    println!("=== Fibonacci Examples in Janet ===\n");

    // Example 1: Simple recursive Fibonacci (inefficient but clear)
    println!("1. Recursive Fibonacci (using shared runtime):");
    janet_shared!(r#"
        (defn fib-recursive [n]
            "Simple recursive Fibonacci - exponential time complexity"
            (if (< n 2) 
                n 
                (+ (fib-recursive (- n 1)) 
                   (fib-recursive (- n 2)))))
    "#);

    println!("Computing fib(10) recursively...");
    janet_shared!("(print \"fib(10) = \" (fib-recursive 10))");

    // Example 2: Iterative Fibonacci (efficient)
    println!("\n2. Iterative Fibonacci:");
    janet_shared!(r#"
        (defn fib-iterative [n]
            "Efficient iterative Fibonacci - linear time complexity"
            (if (< n 2) 
                n
                (do
                    (var a 0)
                    (var b 1)
                    (for i 2 (+ n 1)
                        (let [temp (+ a b)]
                            (set a b)
                            (set b temp)))
                    b)))
    "#);

    println!("Computing first 15 Fibonacci numbers iteratively:");
    janet_shared!(r#"
        (for i 0 15
            (print "fib(" i ") = " (fib-iterative i)))
    "#);

    // Example 3: Memoized Fibonacci (efficient recursive)
    println!("\n3. Memoized Fibonacci:");
    janet_shared!("(def fib-memo @{})");
    janet_shared!(r#"
        (defn fib-memoized [n]
            "Memoized Fibonacci - efficient recursive with caching"
            (if (< n 2)
                n
                (if (has-key? fib-memo n)
                    (fib-memo n)
                    (do
                        (def result (+ (fib-memoized (- n 1)) 
                                      (fib-memoized (- n 2))))
                        (put fib-memo n result)
                        result))))
    "#);

    println!("Computing fib(50) with memoization...");
    janet_shared!("(print \"fib(50) = \" (fib-memoized 50))");

    // Example 4: Fibonacci sequence as an array
    println!("\n4. Fibonacci sequence generator:");
    janet_shared!(r#"
        (defn fib-sequence [count]
            "Generate array of first 'count' Fibonacci numbers"
            (def result @[])
            (for i 0 count
                (array/push result (fib-iterative i)))
            result)
    "#);

    janet_shared!("(def fib-20 (fib-sequence 20))");
    janet_shared!("(print \"First 20 Fibonacci numbers: \" fib-20)");

    // Example 5: Fibonacci with functional programming
    println!("\n5. Functional programming approach:");
    janet_shared!(r#"
        (defn fib-stream [a b]
            "Create infinite Fibonacci stream using generators"
            [a (fn [] (fib-stream b (+ a b)))])
    "#);
    janet_shared!(r#"
        (defn take-fib [n stream]
            "Take first n elements from Fibonacci stream"
            (if (<= n 0)
                @[]
                (do
                    (def [val next-fn] stream)
                    (array/concat @[val] (take-fib (- n 1) (next-fn))))))
    "#);

    janet_shared!("(def fib-stream-10 (take-fib 10 (fib-stream 0 1)))");
    janet_shared!("(print \"Functional Fibonacci (10): \" fib-stream-10)");

    // Example 6: Golden ratio approximation using Fibonacci
    println!("\n6. Golden ratio approximation:");
    janet_shared!(r#"
        (defn golden-ratio-approx [n]
            "Approximate golden ratio using Fibonacci ratio"
            (def f1 (fib-iterative n))
            (def f2 (fib-iterative (- n 1)))
            (if (= f2 0) 1 (/ f1 f2)))
    "#);

    janet_shared!("(print \"Golden ratio approximation:\")");
    janet_shared!(r#"
        (for i 10 21
            (print "φ ≈ " (golden-ratio-approx i) 
                   " (using fib(" i ")/fib(" (- i 1) "))"))
    "#);

    // Example 7: Direct Janet runtime for comparison
    println!("\n7. Direct runtime comparison:");
    let runtime = JanetRuntime::new().expect("Failed to create runtime");
    
    runtime.eval(r#"
        (defn fib-direct [n]
            (if (< n 2) n (+ (fib-direct (- n 1)) (fib-direct (- n 2)))))
        (print "Direct runtime fib(12) = " (fib-direct 12))
    "#).expect("Failed to execute");

    // Example 8: Performance timing (simple)
    println!("\n8. Performance comparison:");
    janet_shared!(r#"
        (print "Timing fib(30):")
        (def start-time (os/clock))
        (def result (fib-iterative 30))
        (def end-time (os/clock))
        (print "Result: " result)
        (print "Time (iterative): " (- end-time start-time) " seconds")
    "#);

    println!("\n🎉 Fibonacci examples complete!");
    println!("These examples demonstrate:");
    println!("  • Recursive vs iterative algorithms");
    println!("  • Memoization for optimization");
    println!("  • Functional programming patterns");
    println!("  • Mathematical applications");
    println!("  • Performance considerations");
    println!("  • Janet's expressive syntax in Rust");
}