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

            self.current_comment_is_mod  = None;
            self.current_h3              = None;
            self.mode                    = Mode::Default;
        }
    }

    let mut s = State::default();

    let mut lines = text.lines().enumerate();

    while let Some((idx, line)) = lines.next() {
        let no = idx+1;
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
        } else if s.current_comment_is_mod.is_some() {
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
                            if !li.starts_with("`") {
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

    if s.errors { Err(()) } else { Ok(()) }
}



#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum H3 {
    Safety,
    Usage,
    Arguments,
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
            H3::Errors      => "Errors",
            H3::Returns     => "Returns",
            H3::Examples    => "Examples",
            H3::Output      => "Output",
            H3::SeeAlso     => "See Also",
            H3::Remarks     => "Remarks",
        }
    }
}
