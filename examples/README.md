# Janet-Rust Integration Examples

This directory contains examples demonstrating the Janet procedural macro integration with Rust.

## Running Examples

```bash
# Run the Fibonacci example
cargo run --example fibonacci

# Run the main demo
cargo run
```

## Available Examples

### `fibonacci.rs` - Comprehensive Fibonacci Implementations

This example demonstrates various Fibonacci implementations in Janet:

1. **Recursive Fibonacci** - Simple but exponential time complexity
2. **Iterative Fibonacci** - Efficient linear time implementation
3. **Memoized Fibonacci** - Recursive with caching for optimization
4. **Sequence Generator** - Create arrays of Fibonacci numbers
5. **Functional Programming** - Stream-based approach with generators
6. **Golden Ratio Approximation** - Mathematical application showing convergence
7. **Direct Runtime Comparison** - Using `JanetRuntime` directly vs macros
8. **Performance Timing** - Simple benchmarking

### Key Features Demonstrated

- **Two Macro Forms:**
  - `janet!("code")` - String-based for simple expressions
  - `janet!{ (code) }` - Token-based with AST parsing (IDE support)
  - `janet_shared!` - Persistent runtime that maintains state

- **Janet Language Features:**
  - Function definitions with `defn`
  - Variables with `var` (mutable) and `def` (immutable)
  - Control flow: `if`, `for`, `do`
  - Data structures: arrays `@[]`, tables `@{}`
  - Functional programming: higher-order functions, recursion
  - Pattern matching and destructuring

- **Integration Benefits:**
  - Compile-time syntax validation
  - Runtime execution with proper error handling
  - Shared state across macro invocations
  - Zero-overhead when using string literals

## Example Output

```
=== Fibonacci Examples in Janet ===

1. Recursive Fibonacci (using shared runtime):
Computing fib(10) recursively...
fib(10) = 55

2. Iterative Fibonacci:
Computing first 15 Fibonacci numbers iteratively:
fib(0) = 0
fib(1) = 1
fib(2) = 1
fib(3) = 2
fib(4) = 3
fib(5) = 5
...

6. Golden ratio approximation:
φ ≈ 1.61803396316671 (using fib(20)/fib(19))

8. Performance comparison:
Timing fib(30):
Result: 832040
Time (iterative): 0.00111722946166992 seconds
```

## Creating Your Own Examples

1. Create a new `.rs` file in the `examples/` directory
2. Add the Janet macro imports:
   ```rust
   use jan_core::JanetRuntime;
   use jan_macros::{janet, janet_shared};
   ```
3. Use the macros to embed Janet code:
   ```rust
   // For simple expressions
   janet!("(print \"Hello, Janet!\")");
   
   // For complex multi-line code
   janet_shared!(r#"
       (defn my-function [x y]
           (+ (* x x) (* y y)))
   "#);
   
   // For persistent state
   janet_shared!("(def my-var 42)");
   janet_shared!("(print my-var)"); // Still accessible!
   ```

## Tips

- Use `janet_shared!` when you need to maintain state across multiple macro calls
- Use raw strings `r#"..."#` for multi-line Janet code to avoid escaping issues
- The token-based syntax `janet!{ ... }` provides better IDE support but is limited to simple expressions
- String-based syntax `janet!("...")` supports complex multi-statement code
- Janet functions persist in the shared runtime, allowing modular code organization

## Learn More

- [Janet Language Documentation](https://janet-lang.org/)
- [Janet Tutorial](https://janet.guide/)
- [Rust Procedural Macros](https://doc.rust-lang.org/reference/procedural-macros.html)