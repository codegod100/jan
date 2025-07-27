# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust project that integrates the Janet programming language (a Lisp dialect) with Rust using C FFI bindings. The project demonstrates embedding Janet as a scripting engine within a Rust application.

**Key Architecture:**
- **Janet Integration**: Uses Janet 1.38.0 via C bindings through an amalgamated build
- **FFI Layer**: Custom Rust bindings in `src/janet.rs` that wrap Janet's C API
- **Build System**: Uses `cc` crate to compile Janet C code during Rust build process
- **Runtime**: Safe Rust wrapper around Janet runtime with proper initialization/cleanup

## Common Commands

### Build and Run
```bash
# Check for compilation errors
cargo check

# Build the project
cargo build

# Build with optimizations
cargo build --release

# Run the application
cargo run
```

### Development
```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Run tests (if any exist)
cargo test

# Clean build artifacts
cargo clean
```

## Project Structure

```
/home/nandi/proj/jan/
├── Cargo.toml          # Rust package manifest
├── build.rs            # Build script for compiling Janet C code
├── janet.c             # Janet language amalgamated source (~55k lines)
├── janet.h             # Janet C API header file
└── src/
    ├── main.rs         # Main application entry point
    └── janet.rs        # Rust FFI bindings for Janet
```

## Architecture Details

### C/Rust FFI Integration

The project uses a multi-layered approach to integrate Janet with Rust:

1. **C Layer** (`janet.c` + `janet.h`):
   - Contains the complete Janet language implementation (amalgamated build)
   - Exposes C API functions like `janet_init()`, `janet_dostring()`, etc.
   - Version 1.38.0 with build hash 73334f3

2. **FFI Bindings** (`src/janet.rs`):
   - Defines extern C function declarations matching Janet's API
   - Provides opaque struct representations for Janet types (`Janet`, `JanetTable`)
   - Implements safe Rust wrapper (`JanetRuntime`) with RAII semantics

3. **Safe Rust Interface** (`JanetRuntime`):
   - Handles Janet initialization/deinitialization automatically
   - Provides safe `eval()` method for executing Janet code
   - Uses Drop trait for proper cleanup

### Build System Integration

The `build.rs` script:
- Compiles `janet.c` using the `cc` crate during Rust build
- Links the resulting static library with the Rust binary
- Defines `JANET_ENTRY_POINT` for proper Janet initialization
- Sets up rebuild triggers when Janet C files change

### Key Components

**JanetRuntime** (`src/janet.rs`):
- Core wrapper around Janet runtime
- Manages lifecycle: initialization → evaluation → cleanup
- Error handling for Janet operations
- Memory-safe interface to unsafe Janet C API

**Main Application** (`src/main.rs`):
- Demonstrates basic Janet integration
- Creates runtime instance and evaluates Janet code
- Shows "Hello from Janet!" example

## Important Build Considerations

### C Compilation
- Janet is compiled as a static library during Rust build
- Requires C compiler (gcc/clang) to be available
- Build process is deterministic and cached by Cargo

### Linking
- Static linking ensures no external Janet dependencies at runtime
- The `cc` crate handles cross-platform C compilation
- No need for separate Janet installation

### Memory Management
- Janet runtime is properly initialized once per `JanetRuntime` instance
- RAII pattern ensures `janet_deinit()` is called on drop
- C strings are properly managed with null termination

### Platform Compatibility
- Uses POSIX feature flags for Unix-like systems
- BSD-specific handling for macOS and BSD variants
- Should compile on Linux, macOS, and Windows with appropriate toolchain

## Development Notes

### Janet Language Features
- Full Janet 1.38.0 feature set available
- Supports core environment with standard library
- Can evaluate arbitrary Janet expressions and statements

### Error Handling
- Janet evaluation errors are propagated as Rust `Result` types
- C string conversion errors are handled safely
- Runtime initialization failures are caught and reported

### Extending the Integration
To add more Janet functionality:
1. Add new extern declarations in `src/janet.rs`
2. Implement safe Rust wrappers around unsafe C calls
3. Handle Janet value types appropriately (currently uses opaque `Janet` struct)

### Testing
- No tests currently present in the codebase
- Consider adding integration tests for Janet evaluation
- Unit tests for error handling scenarios would be valuable

## Dependencies

- **Runtime**: None (Janet is statically linked)
- **Build**: `cc = "1.0"` (for compiling C code)
- **Rust Edition**: 2024

This architecture provides a solid foundation for embedding Janet as a scripting language within Rust applications while maintaining memory safety and proper resource management.