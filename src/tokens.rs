use proc_macro2::TokenTree;

use syn::{
	parse::{Parse, ParseStream},
	punctuated::Punctuated,
	Expr, Token,
};

#[derive(Clone)]
pub struct PipeOperator();

impl Parse for PipeOperator {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		_ = input.parse::<Token![|]>()?;
		_ = input.parse::<Token![>]>()?;

		Ok(PipeOperator {})
	}
}

#[derive(Clone)]
pub struct PipeOperation {
	pub mutable: bool,
	pub left: TokenTree,
	pub right: Expr,
}

impl Parse for PipeOperation {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let mutable = {
			if input.peek(Token![mut]) {
				_ = input.parse::<Token![mut]>()?;
				true
			} else {
				false
			}
		};

		// The lefthand side can only be a TokenTree because Expr includes `|>`.
		let left = input.parse::<TokenTree>()?;
		_ = input.parse::<PipeOperator>()?;
		let right = input.parse::<Expr>()?;

		Ok(PipeOperation {
			mutable,
			left,
			right,
		})
	}
}

#[derive(Clone)]
pub struct PipeStatement {
	pub input: TokenTree,
	pub operations: Vec<PipeOperation>,
}

impl Parse for PipeStatement {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let (pipe_input, has_loner) = {
			// Fork the stream and try parsing for a loner.
			// A loner is just a provided expression meant to be assigned to the next lefthand expression.
			let input = input.fork();

			// The pipe input falls victim to the same thing as lefthand pipe operation.
			(input.parse::<TokenTree>()?, input.peek(Token![,]))
		};

		// Advance the normal parser.
		if has_loner {
			_ = input.parse::<TokenTree>()?;
			_ = input.parse::<Token![,]>()?;
		}

		// Parse all of the pipe operations.
		let punctuated_operations =
			Punctuated::<PipeOperation, Token![,]>::parse_terminated(input)?;

		// Convert the Punctuated to a vector.
		let mut operations = vec![];
		for op in punctuated_operations {
			operations.push(op);
		}

		Ok(PipeStatement {
			input: pipe_input,
			operations,
		})
	}
}
