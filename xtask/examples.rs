use std::io::Write;

use mmrbi::*;



pub fn update() {
    mmrbi::fs::write_if_modified_with("thindx/src/_examples.rs", |o|{
        writeln!(o, "// This file is autogenerated by _xtask.rs")?;
        writeln!(o)?;
        writeln!(o, "//! # Examples")?;
        writeln!(o, "use crate::*;")?;

        for e in mmrbi::fs::DirPathType::read_by_alphanumeric("thindx/examples").unwrap_or_else(|err| fatal!("unable to enumerate thindx/examples: {}", err)) {
            if !e.file_type.is_file() { continue }
            let name = e.path.file_name().unwrap_or_else(|| fatal!("expected file_name for {}", e.path.display()));
            let name = name.to_string_lossy();
            let name = match name.strip_suffix(".rs") { Some(n) => n, None => continue };

            let exe = format!("target/{config}/examples/{name}.exe", config="debug", name=name);
            status!("Running", "{}", exe);
            let out = Command::new(exe).current_dir("thindx").stdout0().unwrap_or_else(|err| fatal!("failed: {}", err));

            let src = std::fs::read_to_string(&e.path).unwrap_or_else(|err| fatal!("unable to read `{}`: {}", e.path.display(), err));
            let mut src = src.lines().peekable();

            writeln!(o)?;

            let mut any = false;
            while src.peek().map_or(false, |l| l.starts_with("//!")) {
                writeln!(o, "///{}", src.next().unwrap()[3..].trim_end())?;
                any = true;
            }
            if !any {
                writeln!(o, "/// {}", name)?;
            }
            writeln!(o, "///")?;
            writeln!(o, "/// <style>")?;
            writeln!(o, "/// #main {{ max-width: none; }}")?;
            writeln!(o, "/// </style>")?;

            while src.peek().map_or(false, |l| l.trim().is_empty()) { let _ = src.next(); }

            writeln!(o, "///")?;
            writeln!(o, "/// ### Source")?;
            writeln!(o, "/// ```no_run")?;
            for line in src {
                assert!(!line.contains("```"));
                writeln!(o, "/// {}", line.trim_end())?;
            }
            writeln!(o, "/// ```")?;
            if !out.is_empty() {
                writeln!(o, "///")?;
                writeln!(o, "/// ### Output")?;
                writeln!(o, "/// ```text")?;
                for line in out.lines() {
                    assert!(!line.contains("```"));
                    writeln!(o, "/// {}", line.trim_end())?;
                }
                writeln!(o, "/// ```")?;
            }
            writeln!(o, "///")?;
            writeln!(o, "/// ### To run this example yourself")?;
            writeln!(o, "/// ```cmd")?;
            writeln!(o, "/// git clone https://github.com/MaulingMonkey/thindx")?;
            writeln!(o, "/// cd thindx/thindx")?;
            writeln!(o, "/// cargo run --example {}", name)?;
            writeln!(o, "/// ```")?;
            writeln!(o, "pub const {} : () = ();", name.replace("-", "_"))?;
        }

        Ok(())
    }).unwrap_or_else(|err| fatal!("unable to create `thindx/src/_examples.rs`: {}", err));
}
