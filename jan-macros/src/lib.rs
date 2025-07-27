use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr, Token, Ident, Lit};
use syn::parse::{Parse, ParseStream, Result as ParseResult};
use syn::punctuated::Punctuated;

mod ast;
use ast::*;

/// Janet code evaluation macro
/// 
/// Usage: 
/// - janet!("(print \"Hello from Janet!\")") // String form
/// - janet!{ (print "Hello from Janet!") }      // Token form
#[proc_macro]
pub fn janet(input: TokenStream) -> TokenStream {
    janet_impl(input, false)
}

/// Janet code evaluation macro with shared runtime
/// 
/// This creates a single persistent Janet runtime that maintains state
/// across multiple macro invocations within the same scope.
#[proc_macro]
pub fn janet_shared(input: TokenStream) -> TokenStream {
    janet_impl(input, true)
}

fn janet_impl(input: TokenStream, shared: bool) -> TokenStream {
    // Try to parse as a string literal first (janet!("code"))
    if let Ok(input_str) = syn::parse::<LitStr>(input.clone()) {
        let janet_code = input_str.value();
        
        let expanded = if shared {
            quote! {
                {
                    use jan_core::eval_shared;
                    eval_shared(#janet_code).expect("Failed to execute Janet code");
                }
            }
        } else {
            quote! {
                {
                    use jan_core::JanetRuntime;
                    let runtime = JanetRuntime::new().expect("Failed to initialize Janet runtime");
                    runtime.eval(#janet_code).expect("Failed to execute Janet code");
                }
            }
        };
        
        return TokenStream::from(expanded);
    }
    
    // Otherwise try to parse as Janet AST (janet!{ (code) })
    match syn::parse::<JanetExpr>(input) {
        Ok(expr) => {
            let janet_code = expr.to_janet_code();
            
            let expanded = if shared {
                quote! {
                    {
                        use jan_core::eval_shared;
                        eval_shared(#janet_code).expect("Failed to execute Janet code");
                    }
                }
            } else {
                quote! {
                    {
                        use jan_core::JanetRuntime;
                        let runtime = JanetRuntime::new().expect("Failed to initialize Janet runtime");
                        runtime.eval(#janet_code).expect("Failed to execute Janet code");
                    }
                }
            };
            
            TokenStream::from(expanded)
        }
        Err(e) => {
            let error_msg = format!("Failed to parse Janet expression: {}", e);
            let expanded = quote! {
                compile_error!(#error_msg);
            };
            TokenStream::from(expanded)
        }
    }
}