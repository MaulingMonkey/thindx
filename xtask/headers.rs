use maulingmonkey_windows_sdk_scanner::*;
use mmrbi::*;

use std::collections::*;
use std::fmt::{self, Display, Formatter};
use std::io::Write;
use std::path::*;



fn headers() -> impl Iterator<Item = &'static str> {
    include_str!("../thindx/doc/headers.txt").lines().map(|l| l.split_once('#').map_or(l, |l| l.0).trim()).filter(|l| !l.is_empty())
}

pub fn update() {
    let cpp2rust = &*CPP2RUST;

    use maulingmonkey_windows_sdk_scanner::*;
    let sdk = sdk::WindowsKit::find_latest().unwrap();
    let mut cpp = RootBuilder::new();
    for file in headers() { cpp.add_from_cpp_path(sdk.include.join(file)).unwrap(); }
    let cpp : Root = cpp.finish();
    let headers = headers().map(|h| Header::new(h, &cpp)).collect::<Vec<_>>();

    mmrbi::fs::write_if_modified_with("thindx/src/_headers.rs", |rs|{
        writeln!(rs, "// This file is autogenerated by _xtask.rs")?;
        writeln!(rs)?;
        writeln!(rs, "#![warn(rustdoc::broken_intra_doc_links)]")?;
        writeln!(rs)?;
        writeln!(rs, "//! Rust ⮀ C++ coverage information based on Windows SDK {}", sdk.sdk_version)?;
        writeln!(rs, "//!")?;
        writeln!(rs, "//! ⚠️ Scanned C++ definitions are not yet complete.")?;
        writeln!(rs, "//! Based on [MaulingMonkey/windows-sdk-scanner](https://github.com/MaulingMonkey/windows-sdk-scanner).")?;
        writeln!(rs, "//!")?;



        writeln!(rs, "//! # Headers")?;
        writeln!(rs, "//!")?;
        writeln!(rs, "//! | C++ Header | Interfaces | Structs | Enums | Functions | Constants |")?;
        writeln!(rs, "//! | ---------- | ---------- | ------- | ----- | --------- | --------- |")?;
        for header in headers.iter() {
            write!(rs, "//! |")?;
            write!(rs, " [{name}](#{id}h) |", name = header.name_h, id = header.name)?;
            for (mapped, total) in [
                (header.interfaces .iter().filter(|t| cpp2rust.get(t.id.as_str()).is_some()).count(),   header.interfaces   .len()),
                (header.structs    .iter().filter(|t| cpp2rust.get(t.id.as_str()).is_some()).count(),   header.structs      .len()),
                (header.enums      .iter().filter(|t| cpp2rust.get(t.id.as_str()).is_some()).count(),   header.enums        .len()),
                (header.functions  .iter().filter(|t| cpp2rust.get(t.id.as_str()).is_some()).count(),   header.functions    .len()),
                (header.constants  .iter().filter(|t| cpp2rust.get(t.   as_str()).is_some()).count(),   header.constants    .len()),
                // (header.classes    .iter().filter(|t| cpp2rust.get(t.id.as_str()).is_some()).count(),   header.classes      .len()),
                // (header.unions     .iter().filter(|t| cpp2rust.get(t.id.as_str()).is_some()).count(),   header.unions       .len()),
            ].into_iter() {
                write!(rs, " {} |", ProgressBadge(mapped, total))?;
            }
            writeln!(rs)?;
        }
        writeln!(rs, "//!")?;



        for header in headers.iter() {
            let interfaces  = header.interfaces .iter().map(|t| (t, cpp2rust.get(t.id.as_str())));
            let structs     = header.structs    .iter().map(|t| (t, cpp2rust.get(t.id.as_str())));
            let enums       = header.enums      .iter().map(|t| (t, cpp2rust.get(t.id.as_str())));
            let functions   = header.functions  .iter().map(|t| (t, cpp2rust.get(t.id.as_str())));
            // let classes     = header.classes    .iter().map(|t| (t, cpp2rust.get(t.id.as_str())));
            // let unions      = header.unions     .iter().map(|t| (t, cpp2rust.get(t.id.as_str())));

            writeln!(rs, "//!")?;
            writeln!(rs, "//! <br>")?;
            writeln!(rs, "//!")?;
            writeln!(rs, "//! # {}", header.name_h)?;
            writeln!(rs, "//!")?;

            for (idx, (cpp, rust)) in interfaces.enumerate() {
                if idx == 0 {
                    writeln!(rs, "//! ### C++ Interfaces → Rust Types")?;
                    writeln!(rs, "//!")?;
                }
                write!(rs, "//! {}", CppLink(&cpp.id))?;
                for (idx, rust) in rust.into_iter().flat_map(|p| p.iter()).enumerate() { write!(rs, "{}[`{}`]", if idx == 0 { "&nbsp;→ " } else { ", " }, rust)?; }
                if rust.is_none() { write!(rs, " →&nbsp;❌")?; }
                writeln!(rs, " <br>")?;
                for method in cpp.methods() {
                    let cpp     = format!("{}::{}", method.ty, method.f.id);
                    let rust    = cpp2rust.get(&*cpp);
                    write!(rs, "//! * {}", CppLink(&cpp))?;
                    for (idx, rust) in rust.into_iter().flat_map(|p| p.iter()).enumerate() { write!(rs, "{}[`{}`]", if idx == 0 { "&nbsp;→ " } else { ", " }, rust)?; }
                    if rust.is_none() { write!(rs, " →&nbsp;❌")?; }
                    writeln!(rs, " <br>")?;
                }
                writeln!(rs, "//!")?;
            }

            for (idx, (cpp, rust)) in structs.enumerate() {
                if idx == 0 {
                    writeln!(rs, "//! ### C++ Structs -> Rust Structs")?;
                    writeln!(rs, "//!")?;
                }
                write!(rs, "//! {}", CppLink(&cpp.id))?;
                for (idx, rust) in rust.into_iter().flat_map(|p| p.iter()).enumerate() { write!(rs, "{}[`{}`]", if idx == 0 { "&nbsp;→ " } else { ", " }, rust)?; }
                if rust.is_none() { write!(rs, " →&nbsp;❌")?; }
                writeln!(rs, " <br>")?;
            }

            for (idx, (cpp, rust)) in enums.enumerate() {
                if idx == 0 {
                    writeln!(rs, "//! ### C++ Enums → Rust Structs")?;
                    writeln!(rs, "//!")?;
                }
                write!(rs, "//! {}", CppLink(&cpp.id))?;
                for (idx, rust) in rust.into_iter().flat_map(|p| p.iter()).enumerate() { write!(rs, "{}[`{}`]", if idx == 0 { "&nbsp;→ " } else { ", " }, rust)?; }
                if rust.is_none() { write!(rs, " →&nbsp;❌")?; }
                writeln!(rs, " <br>")?;
                for constant in cpp.values.keys() {
                    let cpp     = if cpp.class { format!("{}::{}", cpp.id, constant) } else { format!("{}", constant) };
                    if CPP2IGNORE.contains(cpp.as_str()) { continue }
                    if cpp.ends_with("_FORCE_DWORD") { continue }
                    let rust    = cpp2rust.get(&*cpp);
                    write!(rs, "//! * {}", CppLink(&cpp))?;
                    for (idx, rust) in rust.into_iter().flat_map(|p| p.iter()).enumerate() { write!(rs, "{}[`{}`]", if idx == 0 { "&nbsp;→ " } else { ", " }, rust)?; }
                    if rust.is_none() { write!(rs, " →&nbsp;❌")?; }
                    writeln!(rs, " <br>")?;
                }
                writeln!(rs, "//!")?;
            }

            for (idx, (cpp, rust)) in functions.enumerate() {
                if idx == 0 {
                    writeln!(rs, "//! ### C++ Functions → Rust Fns")?;
                    writeln!(rs, "//!")?;
                }
                write!(rs, "//! {}", CppLink(&cpp.id))?;
                for (idx, rust) in rust.into_iter().flat_map(|p| p.iter()).enumerate() { write!(rs, "{}[`{}`]", if idx == 0 { "&nbsp;→ " } else { ", " }, rust)?; }
                if rust.is_none() { write!(rs, " →&nbsp;❌")?; }
                writeln!(rs, " <br>")?;
            }

            // classes, unions
        }



        writeln!(rs, "use crate::*;")?;
        Ok(())
    }).unwrap_or_else(|err| fatal!("unable to create `thindx/src/_headers.rs`: {}", err));
}

struct Header<'cpp> {
    name_h:     &'static str,
    name:       &'static str,
    interfaces: Vec<&'cpp cpp::Interface>,
    structs:    Vec<&'cpp cpp::Struct>,
    enums:      Vec<&'cpp cpp::Enum>,
    functions:  Vec<&'cpp cpp::Function>,
    constants:  Vec<String>,
    // classes:    Vec<&'cpp cpp::Class>,
    // unions:     Vec<&'cpp cpp::Union>,
}
impl<'cpp> Header<'cpp> {
    fn new(rel_path: &'static str, cpp: &'cpp Root) -> Self {
        let ignore = &*CPP2IGNORE;
        let name_h = rel_path.rfind('\\').map_or(rel_path, |s| &rel_path[s+1..]);
        let name   = name_h.strip_suffix(".h").unwrap_or(name_h);
        Self {
            name_h, name,
            interfaces: cpp.interfaces  .values_by_key().filter(move |t| !ignore.contains(t.id.as_str()) && t.defined_at.iter().any(move |at| at.path.ends_with(rel_path))).collect(),
            structs:    cpp.structs     .values_by_key().filter(move |t| !ignore.contains(t.id.as_str()) && t.defined_at.iter().any(move |at| at.path.ends_with(rel_path))).collect(),
            enums:      cpp.enums       .values_by_key().filter(move |t| !ignore.contains(t.id.as_str()) && t.defined_at.iter().any(move |at| at.path.ends_with(rel_path))).collect(),
            functions:  cpp.functions   .values_by_key().filter(move |t| !ignore.contains(t.id.as_str()) && t.defined_at.iter().any(move |at| at.path.ends_with(rel_path))).collect(),
            constants:  cpp.enums       .values_by_key().filter(move |t| !ignore.contains(t.id.as_str()) && t.defined_at.iter().any(move |at| at.path.ends_with(rel_path)))
                .flat_map(|e| e.values.keys().filter(|c| !c.ends_with("_FORCE_DWORD")).map(|c| if e.class { format!("{}::{}", e.id, c) } else { format!("{}", c) })).collect()
            // classes:    cpp.classes     .values_by_key().filter(move |t| !ignore.contains(t.id.as_str()) && t.defined_at.iter().any(move |at| at.path.ends_with(rel_path))).collect(),
            // unions:     cpp.unions      .values_by_key().filter(move |t| !ignore.contains(t.id.as_str()) && t.defined_at.iter().any(move |at| at.path.ends_with(rel_path))).collect(),
        }
    }
}

lazy_static::lazy_static! {
    static ref TEXTFILES    : BTreeMap<&'static str, String>            = collect_text_files();
    static ref CPP2IGNORE   : BTreeSet<&'static str>                    = collect_cpp2ignore();
    static ref CPP2RUST     : BTreeMap<&'static str, Vec<&'static str>> = collect_cpp2rust();
    static ref CPP2URL      : BTreeMap<&'static str, Vec<&'static str>> = collect_cpp2url();
}



/// Markdownify a coverage bage.
///
/// *   `❌ 0 of 24`
/// *   `⚠️ 23 of 24`
/// *   `✔️ 24 of 24`
struct ProgressBadge(usize, usize);

impl Display for ProgressBadge {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match *self {
            ProgressBadge(0, 0)             => write!(fmt, " "),
            ProgressBadge(0, n)             => write!(fmt, "❌ 0 of {}", n),
            ProgressBadge(i, n) if i == n   => write!(fmt, "✔️ {} of {}", i, n),
            ProgressBadge(i, n)             => write!(fmt, "⚠️ {} of {}", i, n),
        }
    }
}



/// Markdownify a C++ identifier into a (possibly hyperlinked) code block
struct CppLink<'s>(&'s str);

impl Display for CppLink<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match CPP2URL.get(self.0) {
            Some(url)   => write!(fmt, "[`{}`]({})", self.0, url[0]),
            None        => write!(fmt, "`{}`", self.0),
        }
    }
}



fn collect_text_files() -> BTreeMap<&'static str, String> {
    let cwd = std::env::current_dir().expect("unable to get CWD");
    let mut files = BTreeMap::<&'static str, String>::new();
    imp(&mut files, &cwd, &cwd);
    return files;

    fn imp(files: &mut BTreeMap<&'static str, String>, root: &Path, path: &Path) {
        if path.is_dir() {
            for e in std::fs::read_dir(path).expect("collect_text_files: failed to enumerate directory") {
                let e = e.expect("collect_text_files: error enumerating directory");
                imp(files, root, &e.path());
            }
        }
        if path.is_file() {
            let name = path.file_name().expect("collect_text_files: file_name()");
            let name = name.to_string_lossy();
            if ".rs .md .txt".split(' ').any(|ext| name.ends_with(ext)) {
                let text = std::fs::read_to_string(&path).expect("collect_text_files: error reading text file");
                let rel = path.strip_prefix(root).expect("collect_text_files: determining rel path");
                files.insert(Box::leak(Box::<str>::from(rel.to_string_lossy())), text);
            }
        }
    }
}

fn collect_cpp2ignore() -> BTreeSet<&'static str> {
    let mut r = BTreeSet::<&'static str>::new();
    for (&path, text) in TEXTFILES.iter() {
        let name = path.rsplit_once(&['/', '\\']).map_or(path, |p| p.1);
        if name == "cpp2ignore.txt" {
            for line in text.lines() {
                let line = line.split_once('#').map_or(line, |s| s.0).trim();
                if line.is_empty() { continue }

                r.insert(line.trim());
            }
        } else if path.ends_with(".rs") {
            for line in text.lines() {
                if let Some(cpp) = line.trim().strip_prefix("//#cpp2ignore ") {
                    r.insert(cpp.trim_start());
                } // else not a directive, ignore
            }
        }
    }
    r
}

fn collect_cpp2rust() -> BTreeMap<&'static str, Vec<&'static str>> {
    let mut r = BTreeMap::<&'static str, Vec<&'static str>>::new();
    for (&path, text) in TEXTFILES.iter() {
        let name = path.rsplit_once(&['/', '\\']).map_or(path, |p| p.1);
        if name == "cpp2rust.txt" {
            for (line_idx, line) in text.lines().enumerate() {
                let line = line.split_once('#').map_or(line, |s| s.0).trim();
                if line.is_empty() { continue }

                if let Some((k, v)) = line.split_once('=') {
                    r.entry(k.trim()).or_default().push(v.trim());
                } else {
                    error!(at: path, line: line_idx+1, code: "cpp2rust", "expected `key = value` pair");
                }
            }
        } else if path.ends_with(".rs") {
            for (line_idx, line) in text.lines().enumerate() {
                if let Some(cpp_rs) = line.trim().strip_prefix("//#cpp2rust ") {
                    let cpp_rs = cpp_rs.trim_start();
                    if let Some((cpp, rs)) = cpp_rs.split_once('=') {
                        let cpp = cpp.trim_end();   // before `=`
                        let rs  = rs .trim_start(); // after `=`
                        let rusts = r.entry(cpp).or_default();
                        if !rusts.contains(&rs) { rusts.push(rs); } // O(NN) but N should always be tiny
                    } else {
                        error!(at: path, line: line_idx+1, code: "cpp2rust", "expected `//#cpp2rust cpp=rs` but missing `=`");
                    }
                } // else not a directive, ignore
            }
        } // else ignore
    }
    r
}

fn collect_cpp2url() -> BTreeMap<&'static str, Vec<&'static str>> {
    let mut r = BTreeMap::<&'static str, Vec<&'static str>>::new();
    for (&path, text) in TEXTFILES.iter() {
        let name = path.rsplit_once(&['/', '\\']).map_or(path, |p| p.1);
        if name == "cpp2url.md" {
            for (line_idx, line) in text.lines().enumerate() {
                let line_no = line_idx + 1;
                let line = line.trim();

                let line = if let Some((before_comment, comment)) = line.split_once("<!--") {
                    if let Some(comment) = comment.strip_suffix("-->") {
                        if comment.contains("--") {
                            error!(at: path, line: line_no, "expected only one <!-- ... --> comment per line");
                            continue;
                        } else {
                            before_comment.trim_end()
                        }
                    } else {
                        error!(at: path, line: line_no, "expected only single line comments");
                        continue;
                    }
                } else {
                    line
                };
                if line.is_empty() { continue }

                if let Some((k, v)) = line.split_once("]:").and_then(|(k,v)| Some((k.strip_prefix("[")?.trim(), v.trim()))) {
                    r.entry(k.trim()).or_default().push(v.trim());
                } else {
                    error!(at: path, line: line_no, "expected `[id]: url` pair");
                }
            }
        } else if path.ends_with(".rs") {
            for (line_idx, line) in text.lines().enumerate() {
                let line = line.trim();
                if let Some(cpp_url) = line.trim().strip_prefix("//#cpp2url ") {
                    let cpp_url = cpp_url.trim_start();
                    if let Some((cpp, url)) = cpp_url.split_once('=') {
                        let cpp = cpp.trim_end();   // before `=`
                        let url = url.trim_start(); // after `=`
                        let urls = r.entry(cpp).or_default();
                        if !urls.contains(&url) { urls.push(url); } // O(NN) but N should always be tiny
                    } else {
                        error!(at: path, line: line_idx+1, code: "cpp2url", "expected `//#cpp2url cpp=url` but missing `=`");
                    }
                } // else not a directive, ignore
            }
        } // else not a cpp2url-bearing file, ignore
    }
    r
}
