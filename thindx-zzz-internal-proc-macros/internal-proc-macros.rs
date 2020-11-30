extern crate proc_macro;

use proc_macro::{Span, TokenStream, TokenTree};



/// Indicate an API requires a certain d3dcompiler version range
///
/// ```ignore
/// #[requires(d3dcompiler = 47)]   // d3dcompiler_47.dll minimum required
/// #[requires(!store)]             // desktop app only
/// ```
#[proc_macro_attribute]
pub fn requires(condition: TokenStream, item: TokenStream) -> TokenStream {
    let mut condition = condition.into_iter();
    let key = condition.next().expect("expected `key` in e.g. `#[requires(key = ...)]`");

    match key.to_string().as_str() {
        "d3dcompiler" => {
            let eq = match condition.next() { Some(eq) => eq, None => return compile_error_at("expected `=` after `d3dcompiler`", key.span(), item) };
            if eq.to_string() != "=" { return compile_error_at("expected `=` after `d3dcompiler`", eq.span(), item); }

            let ver = match condition.next() { Some(ver) => ver, None => return compile_error_at("expected version number after `=`", eq.span(), item) };
            let ver_n : u32 = match ver.to_string().parse() { Ok(n) => n, Err(_) => return compile_error_at("expected version number", ver.span(), item) };

            if let Some(unexpected) = condition.next() { return compile_error_at("unexpected token after version number", unexpected.span(), item); }

            if ver_n == 0 {
                postpend_comment(item, "\n<div class=\"note\"><b>Note:</b> This requires a special test version of d3dcompiler.</div>")
            } else {
                postpend_comment(item, &format!("\n<div class=\"note\"><b>Note:</b> This was introduced by d3dcompiler_{}.dll, and is unavailable in earlier versions.</div>", ver_n))
            }
        },
        "!" => {
            let key = condition.next().expect("expected `key` in e.g. `#[requires(!key = ...)]`");
            match key.to_string().as_str() {
                "store" => {
                    if let Some(unexpected) = condition.next() { return compile_error_at("unexpected token after version number", unexpected.span(), item); }

                    postpend_comment(item, "\n<div class=\"note\"><b>Note:</b> You can use this API to develop your Windows Store apps, but you can't use it in apps that you submit to the Windows Store.</div>")
                },
                other   => compile_error_at(format!("expected `store` or another known key, got {:?}", other), key.span(), item),
            }
        },
        other => compile_error_at(format!("expected `d3dcompiler` or another known key, got {:?}", other), key.span(), item),
    }
}

fn compile_error_at(message: impl std::fmt::Debug, at: Span, item: TokenStream) -> TokenStream {
    let mut out = TokenStream::new();
    let err : TokenStream = format!("compile_error!({:?})", message).parse().unwrap();
    out.extend(err.into_iter().map(|mut t| { t.set_span(at); t }));
    out.extend(item);
    out
}

fn postpend_comment(item: TokenStream, text: &str) -> TokenStream {
    let mut item = item.into_iter().peekable();
    let mut out = TokenStream::new();
    while item.peek().map_or(false, |tt| tt.is_punct_str("#")) {
        out.extend(item.next()); // #
        if item.peek().map_or(false, |tt| tt.is_punct_str("!")) {
            out.extend(item.next());
        }
        out.extend(item.next()); // [..]
    }

    out.extend(text.lines().map(|line| format!("/// {}\r\n", line.trim_end()).parse::<TokenStream>().unwrap()));
    out.extend(item);
    out
}

trait TokenTreeExt {
    fn is_group(&self)      -> bool;
    fn is_ident(&self)      -> bool;
    fn is_punct(&self)      -> bool;
    fn is_literal(&self)    -> bool;

    fn is_punct_str(&self, s: &str) -> bool;
}

impl TokenTreeExt for TokenTree {
    fn is_group(&self)      -> bool { match self { TokenTree::Group(_) => true, _ => false } }
    fn is_ident(&self)      -> bool { match self { TokenTree::Ident(_) => true, _ => false } }
    fn is_punct(&self)      -> bool { match self { TokenTree::Punct(_) => true, _ => false } }
    fn is_literal(&self)    -> bool { match self { TokenTree::Literal(_) => true, _ => false } }

    fn is_punct_str(&self, s: &str) -> bool { self.is_punct() && self.to_string() == s }
}
