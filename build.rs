
fn main() {
    let mut build = cc::Build::new();
    
    build
        .file("janet.c")
        .include(".")
        .define("JANET_ENTRY_POINT", None)
        .compile("janet");
    
    println!("cargo:rustc-link-lib=static=janet");
    
    // Re-run if janet.c or janet.h changes
    println!("cargo:rerun-if-changed=janet.c");
    println!("cargo:rerun-if-changed=janet.h");
}