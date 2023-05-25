use proc_macro2::{TokenStream as TokenStream2, TokenTree};

use quote::ToTokens;
use syn::{
	parse::{Parse, ParseStream},
	punctuated::Punctuated,
	Token,
};

#[derive(Debug, Clone)]
pub struct PipeOperator();

impl Parse for PipeOperator {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		_ = input.parse::<Token![|]>()?;
		_ = input.parse::<Token![>]>()?;

		Ok(PipeOperator {})
	}
}
// impl Peek for PipeOperator {

// }

#[derive(Debug, Clone)]
pub struct PipeLine {
	pub expressions: Vec<TokenStream2>,
}

impl Parse for PipeLine {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let mut expressions: Vec<TokenStream2> = vec![];
		'expressions_loop: while !input.is_empty() {
			// Collect all `TokenTree`s until a `|>`.
			let mut parts = vec![];
			'parts_loop: while !input.is_empty() {
				// Support comma separation.
				// Stop parsing the whole line when a comma is found.
				if input.peek(Token![,]) {
					// Don't forget to join the token streams first.
					expressions.push(TokenStream2::from_iter(parts));
					break 'expressions_loop;
				}

				// TODO: `impl Peek for PipeOperator`.
				if input.peek(Token![|]) && input.peek2(Token![>]) {
					_ = input.parse::<PipeOperator>()?;
					break 'parts_loop;
				}

				let part = input.parse::<TokenTree>()?;
				parts.push(part.to_token_stream());
			}
			// Join all token stream parts into one.
			// That's an expression.
			expressions.push(TokenStream2::from_iter(parts));
		}

		Ok(PipeLine { expressions })
	}
}

#[derive(Debug, Clone)]
pub struct PipeStatement {
	pub lines: Vec<PipeLine>,
}

impl Parse for PipeStatement {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		// Parse all of the pipelines.
		let punctuated_lines =
			Punctuated::<PipeLine, Token![,]>::parse_terminated(input)?;

		// Convert the Punctuated to a vector.
		let mut lines = vec![];
		for op in punctuated_lines {
			lines.push(op);
		}

		Ok(PipeStatement { lines })
	}
}
