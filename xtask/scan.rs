use mmrbi::*;

use syn::*;
use syn::spanned::Spanned;

use std::path::Path;
use std::result::Result;
use std::str::FromStr;

const MAX_UNDOCUMENTED_ARGS : usize = 10;



pub fn all() -> Result<(), ()> {
    dir(&std::env::current_dir().expect("current_dir"))
}

fn dir(path: &Path) -> Result<(), ()> {
    if ".cargo .git .notes .worktree assets target vs".split(' ').any(|dir| path.ends_with(dir)) { return Ok(()) } // skip
    let mut errors = false;
    for e in fs::DirPathType::read_by_alphanumeric(path).unwrap() {
        if e.is_dir() { errors |= dir(&e.path).is_err(); }
        if e.is_file() { errors |= file(&e.path).is_err(); }
    }
    if errors { Err(()) } else { Ok(()) }
}

fn file(path: &Path) -> Result<(), ()> {
    let name = path.file_name().unwrap_or_default().to_string_lossy();
    let name = &*name;

    if name.ends_with(".rs") {
        file_rs(path)
    } else if ".cmd .hlsl .json .md .natvis .toml .txt .yml".split(' ').any(|ext| name.ends_with(ext)) {
        Ok(()) // no linting yet
    } else if ".git .gitignore Cargo.lock LICENSE-APACHE LICENSE-MIT".split(' ').any(|exact| name == exact) {
        Ok(()) // no linting yet
    } else {
        panic!("xtask::scan::file: unexpected extension for `{}`", path.display());
    }
}

fn file_rs(path: &Path) -> Result<(), ()> {
    let text = std::fs::read_to_string(path).map_err(|err| fatal!("failed to read {}: {}", path.display(), err))?;

    // skip validating comments of these free-form / generated files
    let skip = ["_examples.rs", "_headers.rs", "_lib.rs"];
    let skip = path.file_name().map_or(false, |file_name| skip.iter().copied().any(|n| file_name == n));
    if skip { return Ok(()) }

    let mut ctx = Context { path, errors: false };
    let ctx = &mut ctx;

    let file = syn::parse_file(&text).map_err(|err| error!("unable to parse {}: {}", path.display(), err))?;
    let _docs = Docs::parse(ctx, &file.attrs);
    file.items.iter().for_each(|i| item(ctx, i));
    if ctx.errors { Err(()) } else { Ok(()) }
}

fn item(ctx: &mut Context, item: &Item) {
    match item {
        Item::Const(g)          => item_generic(ctx, &g.attrs),
        Item::Enum(g)           => item_generic(ctx, &g.attrs),
        Item::ExternCrate(g)    => item_generic(ctx, &g.attrs),
        Item::Fn(f)             => item_fn(ctx, f),
        Item::ForeignMod(g)     => item_generic(ctx, &g.attrs),
        Item::Impl(i)           => item_impl(ctx, i),
        Item::Macro(g)          => item_generic(ctx, &g.attrs),
        Item::Macro2(g)         => item_generic(ctx, &g.attrs),
        Item::Mod(m)            => item_mod(ctx, m),
        Item::Static(g)         => item_generic(ctx, &g.attrs),
        Item::Struct(s)         => item_struct(ctx, s),
        Item::Trait(t)          => item_trait(ctx, t),
        Item::TraitAlias(g)     => item_generic(ctx, &g.attrs),
        Item::Type(g)           => item_generic(ctx, &g.attrs),
        Item::Union(u)          => item_union(ctx, u),
        Item::Use(g)            => item_generic(ctx, &g.attrs),
        Item::Verbatim(_) => {
            warning!(path: ctx.path, line: item.span().start().line, "failed to parse item: verbatim starts here");
            warning!(path: ctx.path, line: item.span().end()  .line, "failed to parse item: verbatim ends here");
        },
        _                       => warning!(path: ctx.path, line: item.span().start().line, "failed to parse item"),
    }
}

fn item_generic(ctx: &mut Context, attrs: &[Attribute]) {
    let _docs = Docs::parse(ctx, &attrs);
}

/// `fn f() { ... }`
fn item_fn(ctx: &mut Context, f: &ItemFn) {
    let docs = Docs::parse(ctx, &f.attrs);
    validate_docs_vs_signature(ctx, &docs, &f.sig);
}

/// `impl i { ... }`
fn item_impl(ctx: &mut Context, i: &ItemImpl) {
    item_generic(ctx, &i.attrs);
    i.items.iter().for_each(|i| match i {
        ImplItem::Method(m) => {
            let docs = Docs::parse(ctx, &m.attrs);
            validate_docs_vs_signature(ctx, &docs, &m.sig);
        },
        ImplItem::Const(c)  => item_generic(ctx, &c.attrs),
        ImplItem::Type(t)   => item_generic(ctx, &t.attrs),
        ImplItem::Macro(m)  => item_generic(ctx, &m.attrs),
        _                   => warning!(path: ctx.path, line: i.span().start().line, "failed to parse impl item"),
    })
}

/// `mod m { ... }`
fn item_mod(ctx: &mut Context, m: &ItemMod) {
    let _docs = Docs::parse(ctx, &m.attrs);
    m.content.iter().flat_map(|(_, items)| items).for_each(|i| item(ctx, i));
}

/// `struct s { ... }`
fn item_struct(ctx: &mut Context, s: &ItemStruct) {
    let _docs = Docs::parse(ctx, &s.attrs);
    match &s.fields {
        Fields::Named(f)    => f.named  .iter().for_each(|f| item_field(ctx, f)),
        Fields::Unnamed(f)  => f.unnamed.iter().for_each(|f| item_field(ctx, f)),
        Fields::Unit => {},
    }
}

fn item_field(ctx: &mut Context, f: &Field) {
    let _docs = Docs::parse(ctx, &f.attrs);
}

/// `trait t { ... }`
fn item_trait(ctx: &mut Context, t: &ItemTrait) {
    let _docs = Docs::parse(ctx, &t.attrs);
    t.items.iter().for_each(|i| match i {
        TraitItem::Method(m) => {
            let docs = Docs::parse(ctx, &m.attrs);
            validate_docs_vs_signature(ctx, &docs, &m.sig);
        },
        TraitItem::Const(c) => item_generic(ctx, &c.attrs),
        TraitItem::Type(t)  => item_generic(ctx, &t.attrs),
        TraitItem::Macro(m) => item_generic(ctx, &m.attrs),
        _                   => warning!(path: ctx.path, line: i.span().start().line, "failed to parse trait item"),
    })
}

/// `union u { ... }`
fn item_union(ctx: &mut Context, u: &ItemUnion) {
    let _docs = Docs::parse(ctx, &u.attrs);
    u.fields.named.iter().for_each(|f| item_field(ctx, f));
}

fn validate_docs_vs_signature(ctx: &mut Context, docs: &Docs, sig: &Signature) {
    let line = sig.ident.span().start().line;

    macro_rules! error { ( $($tt:tt)* ) => {{ ctx.errors = true; super::error!(path: ctx.path, line: line, $($tt)*) }} }

    if sig.unsafety.is_some() && !docs.has_safety_docs                  { error!("unsafe fn missing `### ⚠️ Safety ⚠️` docs") }
    if sig.inputs.len() > MAX_UNDOCUMENTED_ARGS && docs.args.is_empty() { error!("fn missing `### Arguments` docs") }

    if !docs.args.is_empty() {
        let mut doc_args = docs.args.iter().fuse();
        let mut code_args = sig.inputs.iter().filter_map(|i| match i {
            FnArg::Receiver(_) => None, // self
            FnArg::Typed(PatType { pat, .. }) => match &**pat {
                Pat::Ident(PatIdent{ident,..}) => Some(ident),
                pat => { warning!("unexpected argument type {:?}", pat); None },
            },
        }).fuse();

        let mut arg_no = 0;
        loop {
            arg_no += 1;
            match (doc_args.next(), code_args.next()) {
                (Some(doc_arg), Some(code_arg)) => {
                    let code_arg_name = code_arg.to_string();
                    if doc_arg.name != code_arg_name {
                        super::error!(path: ctx.path, line: doc_arg.line, "argument {} was mis-documented to be `{}` but was actually `{}`", arg_no, doc_arg.name, code_arg_name);
                        ctx.errors = true;
                    }
                },
                (Some(doc_arg), None) => {
                    super::error!(path: ctx.path, line: doc_arg.line, "argument {} does not exist, but was documented to be `{}`", arg_no, doc_arg.name);
                    ctx.errors = true;
                },
                (None, Some(code_arg)) => {
                    let code_arg_name = code_arg.to_string();
                    super::error!(path: ctx.path, line: code_arg.span().start().line, "argument {} (`{}`) is undocumented", arg_no, code_arg_name);
                    ctx.errors = true;
                },
                (None, None) => break,
            }
        }
    }
}



#[derive(Default)]
struct Docs {
    pub has_safety_docs:    bool,
    pub args:               Vec<Arg>,
}

struct Arg {
    pub line:   usize,
    pub name:   String,
}

impl Docs {
    pub fn parse(ctx: &mut Context, attrs: &[Attribute]) -> Self {
        let mut docs = Docs::default();

        let mut doc_lines = attrs.iter().filter_map(|attr| if !attr.path.is_ident("doc") {
            None
        } else if let Ok(Meta::NameValue(MetaNameValue { lit: Lit::Str(doc), .. })) = attr.parse_meta() {
            Some(doc)
        } else {
            None
        }).peekable();

        let mut current_h3 = None;

        while let Some(doc) = doc_lines.next() {
            let line = doc.span().start().line;
            let text = doc.value();
            let trim = text.trim();
            macro_rules! error      { ( $($tt:tt)* ) => {{ ctx.errors = true; super::error!  (path: ctx.path, line: line, $($tt)*) }} }
            macro_rules! warning    { ( $($tt:tt)* ) => {{                    super::warning!(path: ctx.path, line: line, $($tt)*) }} }

            if !trim.is_empty() && !text.starts_with(" ") { error!("use a space between `///` or `//!` and the doc comment text (avoids breaking markdown tables!)") }

            if let Some(h3) = trim.strip_prefix("### ").map(|s| s.trim_start()) {
                let prev_h3 = current_h3;
                match H3::from_str(h3) {
                    Ok(h3)      => current_h3 = Some(h3),
                    Err(err)    => { current_h3 = None; error!("{}", err); },
                }
                docs.has_safety_docs |= current_h3 == Some(H3::Safety);

                if let (Some(prev_h3), Some(current_h3)) = (prev_h3, current_h3) {
                    if prev_h3 >= current_h3 {
                        //warning!("Expected `{}` to come before `### {}`", trim, prev_h3.as_str())
                    }
                }

                match doc_lines.peek() {
                    None                                        => error!("Expected documentation content after `### {}`", h3),
                    Some(doc) if doc.value().trim().is_empty()  => error!("Unexpected blank line after doc comment"),
                    // TODO: warn if back-to-back headers?
                    _                                           => {},
                }
            } else if trim.starts_with("#") {
                current_h3 = None;
                //warning!("Unexpected header `{}`: expected h3 comments only", text);
            } else if current_h3 == Some(H3::Arguments) {
                // TODO: warn to tab-indent line items
                if let Some(arg) = text.strip_prefix(" *").map(str::trim_start) { // only match top-level line items
                    let arg = arg.strip_prefix('`').unwrap_or_else(|| { warning!("quote arg_name with backquotes ala `arg_name`"); arg });
                    if let Some((arg, _after)) = arg.split_once('`') {
                        if arg.len() != arg.trim().len()        { warning!("unexpected whitespace in arg_name") }
                        //if !_after.trim_start().starts_with("-") { warning!("expected dash after code-quoted arg_name") }
                        docs.args.push(Arg { line, name: arg.into() });
                    } else {
                        warning!("missing closing backquote to arg name");
                    }
                } else {
                    // continued arg text?
                }
            } else { // not a header
                if trim.starts_with("```") { // "```", "```rust", "```text", etc.
                    while doc_lines.next_if(|l| l.value().trim() != "```").is_some() {}
                    if doc_lines.next().is_none() { warning!("Unexpected end of documentation mid code-block") }
                    continue;
                }
            }
        }

        docs
    }
}

struct Context<'p> {
    pub path:   &'p Path,
    pub errors: bool,
}

macro_rules! h3 {
    ( $($label:literal => $ident:ident),* $(,)? ) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        enum H3 { $($ident),* }

        impl FromStr for H3 {
            type Err = String;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                #[allow(unreachable_patterns)] // A few overrides before $label => $ident
                match s {
                    "⚠️ Safety ⚠️"     => Ok(H3::Safety),
                    "Safety"            => Err(format!("`### {}` should be marked `### ⚠️ Safety ⚠️`", s)),
                    "Example"           => Ok(H3::Examples),
                $(  $label              => Ok(H3::$ident),                                                      )*
                    other               => Err(format!("`### {}` not an expected h3 header", other))
                }
            }
        }

        impl H3 {
            #[allow(dead_code)] // used in for-now commented out warnings
            pub fn as_str(&self) -> &'static str { match *self { $(H3::$ident => $label),* } }
        }
    }
}

h3! {
    // General
    "⚠️ Safety ⚠️" => Safety,
    "Usage"         => Usage,

    // Trait specific
    "Methods"       => Methods,

    // Function specific
    "Arguments"     => Arguments,
    "Panics"        => Panics,
    "Errors"        => Errors,
    "Returns"       => Returns,

    // Module specific
    "Enumerations"  => Enumerations,
    "Flags"         => Flags,
    "Functions"     => Functions,
    "Interfaces"    => Interfaces,
    "Traits"        => Traits,
    "Structures"    => Structures,
    "Values"        => Values,
    "Wrappers"      => Wrappers, // XXX
    "Features"      => Features,

    // General
    "Examples"      => Examples,
    "Output"        => Output,
    "See Also"      => SeeAlso,
    "Remarks"       => Remarks,
}
