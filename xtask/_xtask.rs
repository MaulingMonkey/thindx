// xtasks.rs

use std::collections::BTreeSet;
use std::path::*;
use std::sync::atomic::{AtomicBool, Ordering::Relaxed};

use mmrbi::*;



fn main() {
    let mut args = std::env::args();
    let _exe = args.next();

    let cmd = args.next().expect("command");
    match cmd.as_str() {
        "check" => {
            let orig_path = args.next();
            let orig_path = orig_path.as_ref().map(|s| &**s);

            let mut errors = false;
            if let Some(orig_path) = orig_path {
                errors |= run_tests(orig_path).is_err();
            }
            errors |= audit(orig_path.map(Path::new)).is_err();
            if errors {
                std::process::exit(1);
            }
        },
        other => fatal!("unknown command {:?}", other),
    }
}

fn run_tests(orig_path: &str) -> Result<(), ()> {
    let mut path = orig_path.split(|ch| "/\\".contains(ch)).collect::<Vec<_>>();

    let (package, mut path) = match path.get_mut(0..3) {
        Some(["thindx", "src", "headers"])  => ("thindx", &mut path[3..]),
        Some([package, "src", ..])          => (*package, &mut path[2..]),
        _other => {
            error!("unable to check `{}`: not an expected package src folder", orig_path);
            return Err(());
        }
    };

    if let Some(last) = path.last_mut() {
        if !last.ends_with(".rs") {
            error!("unable to check `{}`: only rust-lang files supported", orig_path);
            return Err(());
        }
        *last = &last[..last.len()-3];
        if last.starts_with("_") {
            let n = path.len();
            path = &mut path[..n-1]; // "foo\_foo.rs" => "foo"
        }
    }
    let pattern = path.join("::").replace(".", "_"); // "d3d11shader.h" => "d3d11shader_h"

    let mut cmd = Command::new("cargo");
    cmd.arg("test");
    cmd.arg("-p").arg(package);
    cmd.arg("--").arg(pattern);
    if let Err(err) = cmd.status0(){
        error!("tests failed: {}", err);
        return Err(());
    }

    Ok(())
}


static ERRORS   : AtomicBool = AtomicBool::new(false);
static WARNINGS : AtomicBool = AtomicBool::new(false);

macro_rules! error {
    ( $($tt:tt)* ) => {{
        mmrbi::error!($($tt)*);
        ERRORS.store(true, Relaxed);
    }};
}

macro_rules! warning {
    ( $($tt:tt)* ) => {{
        mmrbi::warning!($($tt)*);
        WARNINGS.store(true, Relaxed);
    }};
}


fn audit(orig_path: Option<&Path>) -> Result<(), ()> {
    match orig_path {
        Some(p) => audit_files(Some(p).into_iter()),
        None => {
            let paths = glob::glob("thindx/src/**/*.rs").unwrap_or_else(|err| fatal!("unable to get files: {}", err));
            let paths = paths.map(|e| e.unwrap_or_else(|err| fatal!("unable to enumerate files: {}", err))).collect::<Vec<_>>();
            let paths = paths.iter().map(|p| &**p);
            audit_files(paths)
        }
    }
}

#[allow(unused_variables)] // found_*
fn audit_files<'p>(paths: impl Iterator<Item = &'p Path>) -> Result<(), ()> {
    for path in paths {
        let data = std::fs::read_to_string(&path).unwrap_or_else(|err| fatal!("unable to read `{}`: {}", path.display(), err));

        let mut file_audit = BTreeSet::new();
        for line in data.lines() {
            let line = line.trim();
            let line = match line.strip_prefix("///").or(line.strip_prefix("//!")).or(line.strip_prefix("//")) { Some(l) => l, None => break };
            let line = line.trim_start();

            if let Some(tokens) = line.strip_prefix("AUDIT:") {
                for t in tokens.split(",") { file_audit.insert(t.trim().to_string()); }
            }
        }

        let mut found_docs_ms = false;
        let mut found_unsafe = false;
        let mut found_enumish = false; // XXX: check per-line?
        let mut found_struct = false;
        let mut found_impl = false;
        for line in data.lines() {
            found_docs_ms   |= line.contains("[docs.microsoft.com]");
            found_unsafe    |= line.contains("unsafe");
            found_enumish   |= line.contains("enumish!");
            found_struct    |= line.contains("struct");
            found_impl      |= line.contains("impl");
        }

        if found_docs_ms && !file_audit.contains("verified-ms-docs-links") {
            warning!(at: path, code: "verified-ms-docs-links", "verify docs.microsoft.com links and add `// AUDIT: verified-ms-docs-links");
        }

        // ...
    }

    if ERRORS.load(Relaxed) {
        error!("audit failed");
        Err(())
    } else if WARNINGS.load(Relaxed) {
        println!("⚠️   audit warnings");
        Ok(())
    } else {
        println!("✔️   audit passed");
        Ok(())
    }
}
