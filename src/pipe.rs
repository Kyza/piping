use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned};

use crate::{tokens::PipeStatement, utils::ConstIdent};

pub const PIPELINE_IDENT: ConstIdent = ConstIdent("__");

pub fn pipe(input: TokenStream) -> TokenStream {
	let pipes = parse_macro_input!(input as PipeStatement);

	let mut lines = vec![];
	for (i, line) in pipes.lines.iter().enumerate() {
		let mut expressions = vec![];
		for (j, expression) in line.expressions.iter().enumerate() {
			let expr = expression.clone();

			// Ensure the first one isn't a PIPELINE_IDENT.
			if i == 0 && j == 0 {
				// If it isn't a PIPELINE_IDENT then assign it to the placeholder variable.
				if PIPELINE_IDENT.to_ident(Some(expr.span()))
					!= expr.to_string()
				{
					expressions.push(quote! {
						let #PIPELINE_IDENT = #expr;
					});
				}
			}
			// If it's the start of a line that isn't the first line, it's for destructuring.
			// This is handled below, so it can be ignored.
			else if i > 0 && j == 0 {
				// Do nothing...
			}
			// If it's the final expression, return it instead of assigning.
			else if i == pipes.lines.len() - 1
				&& j == line.expressions.len() - 1
			{
				expressions.push(quote! {
					#expr
				});
			}
			// If it's not the last line, but it's the last expression in a line, combine the destructure of the next line.
			else if i < pipes.lines.len() - 1
				&& j == line.expressions.len() - 1
			{
				let next = pipes
					.lines
					.get(i + 1)
					.expect("a pipeline is missing") // We know it exists already.
					.expressions
					.get(0)
					.expect("a pipeline has no expressions");

				expressions.push(quote! {
					let #next = #expr;
				});
			} else {
				expressions.push(quote! {
					let #PIPELINE_IDENT = #expr;
				});
			}
		}
		lines.push(TokenStream2::from_iter(expressions));
	}

	quote! {
		{
			#(#lines)*
		}
	}
	.into()
}
