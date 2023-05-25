use proc_macro2::{
	Group, Ident, Span, TokenStream as TokenStream2, TokenTree,
};

pub struct ConstIdent(pub &'static str);

impl ConstIdent {
	pub fn to_ident(&self) -> Ident {
		Ident::new(self.0, Span::call_site())
	}
}

impl quote::ToTokens for ConstIdent {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		self.to_ident().to_tokens(tokens)
	}
}

pub const PIPELINE_IDENT: ConstIdent = ConstIdent("__pipeline_value__");

pub fn replace_pipe_symbol(stream: TokenStream2) -> TokenStream2 {
	let stream_iter = stream.into_iter();
	let stream_vec: Vec<_> = stream_iter.clone().collect();
	stream_iter
		.enumerate()
		.map(|(i, tree)| match tree {
			TokenTree::Ident(ident) => {
				// Only replace `_`.
				if ident == "_" {
					// Ensure the next token isn't `=`.
					// `_ =` should not be replaced.
					let next = stream_vec.get(i + 1);
					if let Some(TokenTree::Punct(punct)) = next {
						if punct.to_string() == "=" {
							return TokenTree::Ident(ident);
						}
					}
					// In all other cases, `_` should be replaced.
					// Things such as declared function args named `_` will be replaced, but args starting with `_` shouldn't be used regardless.
					// The identifier that's used to replace `_` starts with a `_` itself, so no warnings should be produced in the generated code.
					return TokenTree::Ident(PIPELINE_IDENT.to_ident());
				}
				TokenTree::Ident(ident)
			}
			// Groups can't be mutated so make a new one with the deeply modified stream.
			TokenTree::Group(group) => TokenTree::Group(Group::new(
				group.delimiter(),
				replace_pipe_symbol(group.stream()),
			)),
			_ => tree,
		})
		.collect()
}
