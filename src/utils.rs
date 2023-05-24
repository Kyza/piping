use proc_macro2::{Ident, Span, TokenStream};

pub struct ConstIdent(pub &'static str);

impl ConstIdent {
	pub fn to_ident(&self) -> Ident {
		Ident::new(self.0, Span::call_site())
	}
}

impl quote::ToTokens for ConstIdent {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		self.to_ident().to_tokens(tokens)
	}
}
