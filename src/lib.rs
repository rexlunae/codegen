pub use proc_macro2::TokenStream;

/// A trait for an object that can be converted to Rust code. Any data structure implementing this trait can be converted into a proc_macro2::TokenStream.
pub trait CodeGen: std::fmt::Debug {

    /// A type, generally an enum, that passes the code generator the context of the node.
    type Context;

    /// A struct representing the set of compilation options.
    type Options;

    /// A trait method to input Rust code in a general sense. The output should be stream of Rust tokens,
    /// however, it is not guaranteed that it will fully compile because of scoping errors and other checks
    /// that don't occur until later.
    fn to_rust(self, ctx: Self::Context, options: Self::Options) -> Result<TokenStream, Box<dyn std::error::Error>>;

    /// A trait method for extracting a docstring from an object that can have a docstring.
    fn get_docstring(&self) -> Option<String> {
        None
    }
}
