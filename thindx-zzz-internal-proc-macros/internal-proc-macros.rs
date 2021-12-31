extern crate proc_macro;

use proc_macro::{Span, TokenStream, TokenTree};



#[proc_macro_attribute]
pub fn xallow(condition: TokenStream, mut item: TokenStream) -> TokenStream {
    let mut condition = condition.into_iter();
    while let Some(ident) = condition.next() {
        if !ident.is_ident() {
            return compile_error_at(format!("expected ident, got {:?}", ident), ident.span(), item);
        }

        match ident.to_string().as_str() {
            "missing_argument_docs" => {},
            other => {
                item = compile_error_at(format!("unknown xallow identifier {:?}", other), ident.span(), item);
            },
        }

        if let Some(comma) = condition.next() {
            if !comma.is_punct_str(",") {
                return compile_error_at(format!("expected `,` to separate allowed conditions, got {:?}", comma), comma.span(), item);
            }
        }
    }
    item
}


fn compile_error_at(message: impl std::fmt::Debug, at: Span, item: TokenStream) -> TokenStream {
    let mut out = TokenStream::new();
    let err : TokenStream = format!("compile_error!({:?});", message).parse().unwrap();
    out.extend(err.into_iter().map(|mut t| { t.set_span(at); t }));
    out.extend(item);
    out
}

trait TokenTreeExt {
    fn is_group(&self)      -> bool;
    fn is_ident(&self)      -> bool;
    fn is_punct(&self)      -> bool;
    fn is_literal(&self)    -> bool;

    fn is_ident_str(&self, s: &str) -> bool;
    fn is_punct_str(&self, s: &str) -> bool;
}

impl TokenTreeExt for TokenTree {
    fn is_group(&self)      -> bool { match self { TokenTree::Group(_) => true, _ => false } }
    fn is_ident(&self)      -> bool { match self { TokenTree::Ident(_) => true, _ => false } }
    fn is_punct(&self)      -> bool { match self { TokenTree::Punct(_) => true, _ => false } }
    fn is_literal(&self)    -> bool { match self { TokenTree::Literal(_) => true, _ => false } }

    fn is_ident_str(&self, s: &str) -> bool { self.is_ident() && self.to_string() == s }
    fn is_punct_str(&self, s: &str) -> bool { self.is_punct() && self.to_string() == s }
}
