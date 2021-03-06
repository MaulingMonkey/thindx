extern crate proc_macro;

use proc_macro::{Ident, Span, TokenStream, TokenTree};



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

#[proc_macro_attribute]
pub fn if_stable(condition: TokenStream, item: TokenStream) -> TokenStream {
    let mut condition = condition.into_iter();
    let key = condition.next().expect("expected `key` in e.g. `#[stable(key)]`");

    let unstable = std::env::var_os("CARGO_PKG_VERSION_PRE").map_or(false, |v| !v.is_empty());

    match key.to_string().as_str() {
        "unsafe" => {
            if !unstable {
                let mut item = item.into_iter().peekable();
                let mut out = TokenStream::new();
                let mut n = 0;
                while let Some(tt) = item.next() {
                    if tt.is_ident_str("fn") {
                        out.extend(Some(TokenTree::Ident(Ident::new("unsafe", tt.span()))));
                        n += 1;
                    }
                    out.extend(Some(tt));
                }
                if n == 0 {
                    compile_error_at("failed to mark any fns unsafe", key.span(), out)
                } else {
                    out
                }
            } else {
                item
            }
        },
        "!" => {
            let key = condition.next().expect("expected `key` in e.g. `#[stable(!key)]`");
            match key.to_string().as_str() {
                "pub" => {
                    let mut item = item.into_iter().peekable();
                    let mut out = TokenStream::new();
                    let mut n = 0;
                    while let Some(tt) = item.next() {
                        let is_pub = tt.is_ident_str("pub");
                        out.extend(Some(tt));

                        if is_pub {
                            let tt2 = item.next();
                            if !tt2.as_ref().map_or(false, |tt2| tt2.is_group()) {
                                out.extend(Some("(crate)".parse::<TokenStream>().unwrap()));
                                n += 1;
                            }
                            out.extend(tt2);
                        }
                    }
                    if n == 0 {
                        compile_error_at("failed to convert any `pub`s to `pub(crate)`", key.span(), out)
                    } else {
                        out
                    }
                },
                other   => compile_error_at(format!("expected `!pub` or another known key, got {:?}", other), key.span(), item),
            }
        },
        other => compile_error_at(format!("expected `unsafe` or another known key, got {:?}", other), key.span(), item),
    }
}

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
