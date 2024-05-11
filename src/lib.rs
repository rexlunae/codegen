//! A simple interface for building compilers within the Rust pre-processor. Generally implemented on the node types
//! of an abstract syntax tree (AST), this crate defines a uniform interface that a compiler can call to generate rust
//! code. Output will be a TokenStream, which can be converted to a string (with .to_string()) or used in a macro to
//! generate rust code.

pub use proc_macro2::TokenStream;

/// A trait for an object that can be converted to Rust code. Any data structure implementing this trait can be converted into a proc_macro2::TokenStream.
pub trait CodeGen: std::fmt::Debug + Clone {
    /// A type, generally an enum, that passes the code generator the context of the node.
    type Context;

    /// A struct representing the set of compilation options.
    type Options;

    /// A trait for a symbol table
    type SymbolTable;

    /// A default implementation for find_symbols(), which simply returns the input.
    /// Language nodes that modify the symbol table should override this method.
    fn find_symbols(self, symbols_in: Self::SymbolTable) -> Self::SymbolTable {
        symbols_in
    }

    /// A trait method to output Rust code in a general sense. The output should be stream of Rust tokens,
    /// however, it is not guaranteed that it will fully compile because of scoping errors and other checks
    /// that don't occur until later.
    fn to_rust(
        self,
        ctx: Self::Context,
        options: Self::Options,
        symbols: Self::SymbolTable,
    ) -> Result<TokenStream, Box<dyn std::error::Error>>;

    /// A trait method for extracting a docstring from an object that can have a docstring.
    fn get_docstring(&self) -> Option<String> {
        None
    }
}
