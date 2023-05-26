use proc_macro2::{Ident, Span, TokenStream as TokenStream2};

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
