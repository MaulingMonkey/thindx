use mmrbi::fs::write_if_modified_with;

use std::io::{self, ErrorKind::Other, *};
use std::path::*;



pub fn generate_and_merge() {
    // TODO: generate
    merge();
}

fn merge() {
    scope!("natvis::merge");
    status!("Merging", ".vscode/merged.natvis");
    let r = write_if_modified_with(".vscode/merged.natvis", |nv| {
        writeln!(nv, r#"<?xml version="1.0" encoding="utf-8"?>"#)?;
        writeln!(nv, r#"<AutoVisualizer xmlns="http://schemas.microsoft.com/vstudio/debugger/natvis/2010">"#)?;
        find_merge_natvis(nv, Path::new("."))?;
        writeln!(nv)?;
        writeln!(nv, r#"</AutoVisualizer>"#)?;
        Ok(())
    });
    match r {
        Err(err) if err.kind() == io::ErrorKind::InvalidData    => error!("unable to write .vscode/merged.natvis"), // detail already reported
        Err(err) if err.kind() == io::ErrorKind::Other          => error!("unable to write .vscode/merged.natvis"), // detail already reported
        Err(err)                                                => error!("unable to write .vscode/merged.natvis: {err}"),
        Ok(_) => {},
    }
}

fn find_merge_natvis(nv: &mut impl Write, path: &Path) -> io::Result<()> {
    scope!("natvis::find_merge_natvis(..., {path:?})");

    for skip in ".cargo .git .github .notes .vscode target vs".split(' ') {
        if path.ends_with(skip) { return Ok(()) }
    }

    if path.is_file() {
        let name = path.to_string_lossy();
        if name.ends_with(".natvis") {
            let src = std::fs::read_to_string(path).map_err(|err| { error!("unable to read {}: {}", path.display(), err); err })?;
            let mut lines = src.lines().enumerate();

            macro_rules! expect_line {
                ($line:expr, $expr:expr) => {{
                    let line : usize = $line;
                    let expr : &'static str = $expr;
                    match lines.next() {
                        None => {
                            error!(at: &path, line: line, "missing `{}`", expr);
                            Err(Other)?;
                        },
                        Some((line_idx, line)) => {
                            if line.trim() != expr {
                                error!(at: &path, line: line_idx+1, "expected:\n`{}`, not:\n`{}`", expr, line);
                                Err(Other)?;
                            }
                        },
                    }
                }};
            }

            expect_line!(1, r#"<?xml version="1.0" encoding="utf-8"?>"#);
            expect_line!(2, r#"<AutoVisualizer xmlns="http://schemas.microsoft.com/vstudio/debugger/natvis/2010">"#);

            writeln!(nv)?;
            writeln!(nv, "    <!-- from {} -->", path.display())?;
            writeln!(nv)?;

            while let Some((_line_idx, line)) = lines.next() {
                let trimmed = line.trim();
                if trimmed.is_empty() { continue }
                if trimmed == "</AutoVisualizer>" { break }
                writeln!(nv, "{}", line)?;
            }

            for (line_idx, tail) in lines.next() {
                if !tail.trim().is_empty() {
                    error!(at: &path, line: line_idx+1, "expected nothing after </AutoVisualizer>");
                    Err(Other)?;
                }
            }
        }
    }

    if path.is_dir() && !path.parent().map_or(false, |p| p.ends_with("src")) {
        for e in std::fs::read_dir(path)? {
            find_merge_natvis(nv, &e?.path())?;
        }
    }

    Ok(())
}
