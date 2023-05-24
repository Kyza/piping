use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

mod pipe;
mod tokens;
mod utils;

#[proc_macro]
#[proc_macro_error]
/// Devilman crybaby.
pub fn pipe(input: TokenStream) -> TokenStream {
	pipe::pipe(input)
}
