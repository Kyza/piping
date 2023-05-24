use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::quote;
use syn::parse_macro_input;

use crate::tokens::PipeStatement;

pub fn pipe(input: TokenStream) -> TokenStream {
	let pipes = parse_macro_input!(input as PipeStatement);

	// Convert the expressions to variable assignments.
	let mut operation_quotes: Vec<_> = vec![];
	for (i, op) in pipes.operations.iter().enumerate() {
		let right = op.right.clone();

		if i < pipes.operations.len() - 1 {
			// The variable name and mutability should be in the next lefthand expression.
			let next = pipes.operations.get(i + 1).unwrap().clone();
			let mutable = {
				if next.mutable {
					quote! {mut}
				} else {
					quote! {}
				}
			};
			let left = next.left;

			operation_quotes.push(quote! {
				let #mutable #left = #right;
			});
		} else {
			operation_quotes.push(quote! {
				#right
			});
		}
	}

	// If the left hand side of the input is an expression, assign it with the name of the next lefthand expression.
	let temp_input = {
		if let TokenTree::Ident(_) = pipes.input.clone() {
			quote! {}
		} else {
			let left = &pipes.operations.get(0).unwrap().left;
			let right = pipes.input;
			quote! {
				let #left = #right;
			}
		}
	};

	quote! {
		{
			#temp_input
			#(
				#operation_quotes
			)*
		}
	}
	.into()
}
