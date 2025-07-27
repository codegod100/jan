use syn::parse::{Parse, ParseStream, Result as ParseResult};
use syn::{Token, Ident, Lit, LitStr, LitInt};
use syn::punctuated::Punctuated;

/// Represents a Janet expression
#[derive(Debug, Clone)]
pub enum JanetExpr {
    /// Literal values: numbers, strings, booleans
    Literal(JanetLiteral),
    /// Identifiers and symbols
    Symbol(String),
    /// S-expressions: (func arg1 arg2 ...)
    List(Vec<JanetExpr>),
    /// Arrays: @[elem1 elem2 ...]
    Array(Vec<JanetExpr>),
    /// Tables: @{:key value ...}
    Table(Vec<(JanetExpr, JanetExpr)>),
}

/// Janet literal values
#[derive(Debug, Clone)]
pub enum JanetLiteral {
    String(String),
    Number(f64),
    Integer(i64),
    Boolean(bool),
    Keyword(String),
}

/// Parse Janet expressions from Rust tokens
impl Parse for JanetExpr {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        if input.peek(syn::token::Paren) {
            // Parse S-expression: (expr expr ...)
            let content;
            syn::parenthesized!(content in input);
            let mut exprs = Vec::new();
            while !content.is_empty() {
                exprs.push(content.parse()?);
            }
            Ok(JanetExpr::List(exprs))
        } else if input.peek(Token![@]) {
            // Parse array or table: @[...] or @{...}
            input.parse::<Token![@]>()?;
            if input.peek(syn::token::Bracket) {
                // Array: @[elem1 elem2 ...]
                let content;
                syn::bracketed!(content in input);
                let mut exprs = Vec::new();
                while !content.is_empty() {
                    exprs.push(content.parse()?);
                }
                Ok(JanetExpr::Array(exprs))
            } else if input.peek(syn::token::Brace) {
                // Table: @{key value ...}
                let content;
                syn::braced!(content in input);
                let mut pairs = Vec::new();
                while !content.is_empty() {
                    let key = content.parse()?;
                    let value = content.parse()?;
                    pairs.push((key, value));
                }
                Ok(JanetExpr::Table(pairs))
            } else {
                Err(input.error("Expected '[' or '{' after '@'"))
            }
        } else if input.peek(Lit) {
            // Parse literals
            let lit: Lit = input.parse()?;
            match lit {
                Lit::Str(s) => Ok(JanetExpr::Literal(JanetLiteral::String(s.value()))),
                Lit::Int(i) => {
                    let val = i.base10_parse::<i64>()?;
                    Ok(JanetExpr::Literal(JanetLiteral::Integer(val)))
                }
                Lit::Float(f) => {
                    let val = f.base10_parse::<f64>()?;
                    Ok(JanetExpr::Literal(JanetLiteral::Number(val)))
                }
                Lit::Bool(b) => Ok(JanetExpr::Literal(JanetLiteral::Boolean(b.value))),
                _ => Err(input.error("Unsupported literal type")),
            }
        } else if input.peek(Token![:]) {
            // Parse keyword: :symbol
            input.parse::<Token![:]>()?;
            let ident: Ident = input.parse()?;
            Ok(JanetExpr::Literal(JanetLiteral::Keyword(ident.to_string())))
        } else if input.peek(syn::token::Bracket) {
            // Parse array without @: [elem1 elem2 ...]
            let content;
            syn::bracketed!(content in input);
            let mut exprs = Vec::new();
            while !content.is_empty() {
                exprs.push(content.parse()?);
            }
            Ok(JanetExpr::Array(exprs))
        } else if input.peek(Ident) {
            // Parse symbol/identifier
            let ident: Ident = input.parse()?;
            Ok(JanetExpr::Symbol(ident.to_string()))
        } else {
            Err(input.error("Expected Janet expression"))
        }
    }
}

impl JanetExpr {
    /// Convert AST back to Janet source code string
    pub fn to_janet_code(&self) -> String {
        match self {
            JanetExpr::Literal(lit) => lit.to_janet_code(),
            JanetExpr::Symbol(s) => s.clone(),
            JanetExpr::List(exprs) => {
                let inner = exprs.iter()
                    .map(|e| e.to_janet_code())
                    .collect::<Vec<_>>()
                    .join(" ");
                format!("({})", inner)
            }
            JanetExpr::Array(exprs) => {
                let inner = exprs.iter()
                    .map(|e| e.to_janet_code())
                    .collect::<Vec<_>>()
                    .join(" ");
                format!("[{}]", inner)
            }
            JanetExpr::Table(pairs) => {
                let inner = pairs.iter()
                    .map(|(k, v)| format!("{} {}", k.to_janet_code(), v.to_janet_code()))
                    .collect::<Vec<_>>()
                    .join(" ");
                format!("@{{{}}}", inner)
            }
        }
    }
}

impl JanetLiteral {
    pub fn to_janet_code(&self) -> String {
        match self {
            JanetLiteral::String(s) => format!("\"{}\"", s.replace('"', "\\\"")),
            JanetLiteral::Number(n) => n.to_string(),
            JanetLiteral::Integer(i) => i.to_string(),
            JanetLiteral::Boolean(b) => b.to_string(),
            JanetLiteral::Keyword(k) => format!(":{}", k),
        }
    }
}