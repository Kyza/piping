use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::parse_macro_input;

use crate::{
	tokens::PipeStatement,
	utils::{replace_pipe_symbol, PIPELINE_IDENT},
};

pub fn pipe(input: TokenStream) -> TokenStream {
	let pipes = parse_macro_input!(input as PipeStatement);

	let mut lines = vec![];
	for (i, line) in pipes.lines.iter().enumerate() {
		let mut expressions = vec![];
		for (j, expression) in line.expressions.iter().enumerate() {
			let expr = expression.clone();

			// Ensure the first one isn't a `_`.
			if i == 0 && j == 0 {
				// If it isn't a `_` then assign it to the placeholder variable.
				if expr.to_string() != "_" {
					let expr = replace_pipe_symbol(expr);

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
				let expr = replace_pipe_symbol(expr);

				expressions.push(quote! {
					#expr
				});
			}
			// If it's not the last line, but it's the last expression in a line, combine the destructure of the next line.
			else if i < pipes.lines.len() - 1
				&& j == line.expressions.len() - 1
			{
				let expr = replace_pipe_symbol(expr);
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
				let expr = replace_pipe_symbol(expr);

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
