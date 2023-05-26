use proc_macro2::{
	Group, Ident, Span, TokenStream as TokenStream2, TokenTree,
};

pub struct ConstIdent(pub &'static str);

impl ConstIdent {
	pub fn to_ident(&self, span: Option<Span>) -> Ident {
		Ident::new(
			self.0,
			if let Some(span) = span {
				span
			} else {
				Span::call_site()
			},
		)
	}
}

impl quote::ToTokens for ConstIdent {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		self.to_ident(None).to_tokens(tokens)
	}
}

pub const PIPELINE_IDENT: ConstIdent = ConstIdent("__pipeline_value__");
pub const PIPELINE_SHORT: ConstIdent = ConstIdent("_");
pub const PIPELINE_UNDERSCORE: ConstIdent = ConstIdent("__");

pub fn replace_pipe_symbol(stream: TokenStream2) -> TokenStream2 {
	stream
		.into_iter()
		.map(|tree| match tree {
			TokenTree::Ident(ident) => {
				// Only replace `_` with the PIPELINE_IDENT.
				if ident == PIPELINE_SHORT.to_ident(None) {
					return TokenTree::Ident(
						PIPELINE_IDENT.to_ident(Some(ident.span())),
					);
				}
				// Only replace `__` with the PIPELINE_SHORT.
				else if ident == PIPELINE_UNDERSCORE.to_ident(None) {
					return TokenTree::Ident(
						PIPELINE_SHORT.to_ident(Some(ident.span())),
					);
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
