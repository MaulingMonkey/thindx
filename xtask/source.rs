use mmrbi::*;

use std::path::Path;
use std::str::FromStr;



pub fn scan() -> Result<(), ()> {
    scan_dir(Path::new("thindx/src"))
}

fn scan_dir(path: &Path) -> Result<(), ()> {
    let mut errors = false;
    for e in fs::DirPathType::read_by_alphanumeric(path).unwrap() {
        if e.is_dir() { errors |= scan_dir(&e.path).is_err(); }
        if e.is_file() { errors |= scan_file(&e.path).is_err(); }
    }
    if errors { Err(()) } else { Ok(()) }
}

fn scan_file(path: &Path) -> Result<(), ()> {
    let file = std::fs::read_to_string(path).unwrap_or_else(|err| fatal!("failed to read {}: {}", path.display(), err));

    let errors =
        scan_file_doc_comments(path, &file).is_err() |
        false;

    if errors { Err(()) } else { Ok(()) }
}



fn scan_file_doc_comments(path: &Path, text: &str) -> Result<(), ()> {
    // skip validating comments of these free-form / generated files
    let skip = ["_examples.rs", "_headers.rs", "_lib.rs"];
    let skip = path.file_name().map_or(false, |file_name| skip.iter().copied().any(|n| file_name == n));
    if skip { return Ok(()) }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)] enum Mode {
        Default,
        ExpectNonBlankLine,
        InCodeBlock,
        InStyleBlock,
    }

    impl Default for Mode {
        fn default() -> Mode { Mode::Default }
    }

    #[derive(Debug, Default)] struct State {
        current_comment_is_mod: Option<bool>,
        current_h3:             Option<H3>,
        errors:                 bool,
        mode:                   Mode,
        arguments_line_no:      Option<usize>,
        safety_line_no:         Option<usize>,
        arguments:              Vec<String>,
    }

    impl State {
        pub fn on_comment_end(&mut self, path: &Path, last_comment_line_no: usize) {
            if !self.current_comment_is_mod.is_some() { return }

            match self.mode {
                Mode::Default => {},
                Mode::ExpectNonBlankLine => {
                    if let Some(h3) = self.current_h3 {
                        error!(at: path, line: last_comment_line_no, "Expected non-blank line after `### {}`", h3.as_str());
                        self.errors = true;
                    } else {
                        panic!("bug: should've had current_h3 set?");
                    }
                },
                Mode::InCodeBlock => {
                    error!(at: path, line: last_comment_line_no, "Expected end of code block before end of doc comment");
                    self.errors = true;
                },
                Mode::InStyleBlock => {
                    error!(at: path, line: last_comment_line_no, "Expected end of style block before end of doc comment");
                    self.errors = true;
                },
            }

            if !self.arguments.is_empty() {
                error!(at: path, line: last_comment_line_no, "Expected fn describing `### Arguments`");
                self.errors = true;
            }

            self.current_comment_is_mod = None;
            self.current_h3             = None;
            self.mode                   = Mode::Default;
            self.arguments_line_no      = None;
            self.safety_line_no         = None;
            self.arguments.clear();
        }
    }

    let mut s = State::default();

    let mut lines = text.lines().enumerate();

    let mut last_line_no = 0;
    while let Some((idx, line)) = lines.next() {
        let no = idx+1;
        last_line_no = no;
        let trimmed = line.trim();

        let comment = if let Some(comment) = trimmed.strip_prefix("//!") {
            let comment = comment.strip_prefix(" ").unwrap_or(comment); // XXX: Warn if no space?
            if s.current_comment_is_mod != Some(true) { s.on_comment_end(path, idx); }
            s.current_comment_is_mod = Some(true);
            comment
        } else if let Some(comment) = trimmed.strip_prefix("///") {
            let comment = comment.strip_prefix(" ").unwrap_or(comment); // XXX: Warn if no space?
            if s.current_comment_is_mod != Some(false) { s.on_comment_end(path, idx); }
            s.current_comment_is_mod = Some(false);
            comment
        } else if trimmed.starts_with("//") {
            continue // ignore regular comment lines
        } else if s.current_comment_is_mod == Some(true) {
            s.on_comment_end(path, idx);
            continue
        } else if s.current_comment_is_mod == Some(false) {
            if trimmed.is_empty() {
                error!(at: path, line: no, "unexpected blank line after doc comment");
                s.errors = true;
                continue;
            }

            let mut rest = trimmed;

            loop {
                if let Some(r) = rest.strip_prefix("#[repr(transparent)]") {
                    rest = r.trim_start();
                    //s.repr = Some(Transparent);
                } else if let Some(r) = rest.strip_prefix("#[repr(C)]") {
                    rest = r.trim_start();
                    //s.repr = Some(C);
                } else if let Some(r) = rest.strip_prefix("#[allow(") {
                    let end = match r.find(")]") {
                        Some(n) => n,
                        None => {
                            error!(at: path, line: no, "#[allow(...)] expected to be a single line");
                            s.errors = true;
                            continue;
                        },
                    };
                    rest = r[end+2..].trim_start();
                } else if let Some(r) = rest.strip_prefix("#[derive(") {
                    let end = match r.find(")]") {
                        Some(n) => n,
                        None => {
                            error!(at: path, line: no, "#[derive(...)] expected to be a single line");
                            s.errors = true;
                            continue;
                        },
                    };
                    rest = r[end+2..].trim_start();
                } else if let Some(r) = rest.strip_prefix("#[requires(") {
                    let end = match r.find(")]") {
                        Some(n) => n,
                        None => {
                            error!(at: path, line: no, "#[requires(...)] expected to be a single line");
                            s.errors = true;
                            continue;
                        },
                    };
                    rest = r[end+2..].trim_start();
                } else if rest.starts_with("//") {
                    rest = "";
                } else {
                    break;
                }
            }

            if rest.is_empty() {
                continue; // not an error
            }

            let (_vis, rest) = if let Some(rest) = rest.strip_prefix("pub(crate) ") {
                (Visibility::PubCrate, rest.trim_start())
            } else if let Some(rest) = rest.strip_prefix("pub ") {
                (Visibility::Pub, rest.trim_start())
            } else {
                (Visibility::Private, rest)
            };

            let (is_unsafe, rest) = if let Some(rest) = rest.strip_prefix("unsafe ") {
                (true, rest.trim_start())
            } else {
                (false, rest)
            };

            let (dik, rest) = if let Some(rest) = rest.strip_prefix("fn ")  { (DocItemKind::Fn,     rest.trim_start()) }
            else if let Some(rest) = rest.strip_prefix("const ")            { (DocItemKind::Const,  rest.trim_start()) }
            else if let Some(rest) = rest.strip_prefix("impl ")             { (DocItemKind::Impl,   rest.trim_start()) }
            else if let Some(rest) = rest.strip_prefix("macro_rules! ")     { (DocItemKind::Macro,  rest.trim_start()) }
            else if let Some(rest) = rest.strip_prefix("mod ")              { (DocItemKind::Mod,    rest.trim_start()) }
            else if let Some(rest) = rest.strip_prefix("struct ")           { (DocItemKind::Struct, rest.trim_start()) }
            else if let Some(rest) = rest.strip_prefix("trait ")            { (DocItemKind::Trait,  rest.trim_start()) }
            else if let Some(rest) = rest.strip_prefix("type ")             { (DocItemKind::Type,   rest.trim_start()) }
            else if rest.contains(":") && rest.contains(",") {
                // struct item
                s.on_comment_end(path, idx);
                continue;
            } else if rest.ends_with(r#": Option<unsafe extern "system" fn ("#) {
                // struct fn item
                s.on_comment_end(path, idx);
                continue;
            } else {
                error!(at: path, line: no, "expected const, fn, impl, mod, or struct for documented item");
                s.errors = true;
                s.arguments.clear();
                s.on_comment_end(path, idx);
                continue;
            };

            if is_unsafe && !s.safety_line_no.is_some() {
                error!(at: path, line: idx, "doc comment missing `### Safety` section");
                s.errors = true;
            }

            match dik {
                DocItemKind::Fn     => s.arguments.clear(), // XXX
                DocItemKind::Macro  => s.arguments.clear(), // XXX
                _                   => {},
            }

            // XXX
            let _ = rest;

            s.on_comment_end(path, idx);
            continue
        } else {
            continue
        };

        match s.mode {
            Mode::Default | Mode::ExpectNonBlankLine => {
                let expect_non_blank = s.mode == Mode::ExpectNonBlankLine;
                s.mode = Mode::Default;

                if let Some(h3) = comment.strip_prefix("### ") {
                    if expect_non_blank {
                        error!(at: path, line: no, "Unexpected header `{}`: expected content after previous header first", comment);
                        s.errors = true;
                    }
                    let h3 = match H3::from_str(h3) {
                        Ok(h3) => h3,
                        Err(err) => {
                            error!(at: path, line: no, "{}", err);
                            s.errors = true;
                            continue;
                        },
                    };

                    match h3 {
                        H3::Arguments   => s.arguments_line_no  = Some(no),
                        H3::Safety      => s.safety_line_no     = Some(no),
                        _other          => {},
                    }

                    if let Some(prev_h3) = s.current_h3 {
                        if prev_h3 >= h3 {
                            error!(at: path, line: no, "Expected `{}` to come before `### {}`", comment, prev_h3.as_str());
                            s.errors = true;
                        }
                    }
                    s.current_h3 = Some(h3);
                } else if let Some(_) = comment.strip_prefix("#") {
                    error!(at: path, line: no, "Unexpected header `{}`: expected h3 comments only", comment);
                    s.errors = true;
                } else if comment == "" && expect_non_blank {
                    error!(at: path, line: no, "Don't add a blank line after headers");
                    s.errors = true;
                } else if comment.starts_with("```") {
                    s.mode = Mode::InCodeBlock;
                } else if comment == "<style>" {
                    s.mode = Mode::InStyleBlock;
                } else if let Some(li) = comment.strip_prefix("*   ") {
                    match s.current_h3 {
                        Some(H3::Arguments) => {
                            if let Some(li) = li.strip_prefix("`") {
                                let name_end = match li.bytes().position(|b| b == b'`') {
                                    Some(n) => n,
                                    None => {
                                        error!(at: path, line: no, "Argument name missing closing backtick");
                                        s.errors = true;
                                        continue;
                                    },
                                };
                                let (name, li) = li.split_at(name_end);
                                let li = &li[1..].trim_start();
                                let li = li.strip_prefix("-").unwrap_or(li);
                                let _desc = li.trim_start();
                                s.arguments.push(name.into());
                            } else {
                                error!(at: path, line: no, "Quote argument names with `backticks`");
                                s.errors = true;
                            }
                        },
                        _other => {},
                    }
                } else if let Some(_li) = comment.strip_prefix("* ") {
                    error!(at: path, line: no, "Tab-indent line items");
                    s.errors = true;
                }
            },
            Mode::InCodeBlock if comment == "```" => s.mode = Mode::Default,
            Mode::InCodeBlock => {},
            Mode::InStyleBlock if comment == "</style>" => s.mode = Mode::Default,
            Mode::InStyleBlock => {},
        }
    }

    s.on_comment_end(path, last_line_no);

    if s.errors { Err(()) } else { Ok(()) }
}



#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum H3 {
    Safety,
    Usage,
    Arguments,
    Panics,
    Errors,
    Returns,
    Examples,
    Output,
    SeeAlso,
    Remarks,
}

impl FromStr for H3 {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Safety"    => Ok(H3::Safety),
            "Usage"     => Ok(H3::Usage),
            "Arguments" => Ok(H3::Arguments),
            "Panics"    => Ok(H3::Panics),
            "Errors"    => Ok(H3::Errors),
            "Returns"   => Ok(H3::Returns),
            "Example"   => Ok(H3::Examples),
            "Examples"  => Ok(H3::Examples),
            "Output"    => Ok(H3::Output),
            "See Also"  => Ok(H3::SeeAlso),
            "Remarks"   => Ok(H3::Remarks),
            other       => Err(format!("`### {}` not an expected h3 header", other)),
        }
    }
}

impl H3 {
    pub fn as_str(&self) -> &'static str {
        match *self {
            H3::Safety      => "Safety",
            H3::Usage       => "Usage",
            H3::Arguments   => "Arguments",
            H3::Panics      => "Panics",
            H3::Errors      => "Errors",
            H3::Returns     => "Returns",
            H3::Examples    => "Examples",
            H3::Output      => "Output",
            H3::SeeAlso     => "See Also",
            H3::Remarks     => "Remarks",
        }
    }
}



#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Visibility {
    Pub,
    PubCrate,
    Private,
}



#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum DocItemKind {
    Const,
    Fn,
    Impl,
    Macro,
    Mod,
    Struct,
    Trait,
    Type, // possibly an associated type, possibly a free type
}
