mod examples;
mod scan;

use std::path::*;
use mmrbi::*;



fn main() {
    let mut args = std::env::args();
    let _exe = args.next();

    let cmd = args.next().expect("command");
    match cmd.as_str() {
        "b" | "build"   => build(args),
        "c" | "check"   => check(args),
        other           => fatal!("unknown command {:?}", other),
    }
}

fn build(_args: std::env::Args) {
    for file in "LICENSE-APACHE LICENSE-MIT LICENSE.md Readme.md".split(' ') {
        let from = Path::new(file);
        let to = Path::new("thindx").join(file);
        if from.metadata().and_then(|m| m.modified()).ok() > to.metadata().and_then(|m| m.modified()).ok() {
            if let Err(err) = std::fs::copy(&from, &to) {
                error!("unable to copy {} to thindx/{}: {}", from.display(), to.display(), err);
            }
        }
    }

    // for some reason we need an abs path here, despite not requiring one when invoking `cargo doc` directly from the CLI
    let css = std::env::current_dir().unwrap().join("thindx/doc/style.css");
    std::env::set_var("RUSTDOCFLAGS", format!(r"--extend-css {css}", css = css.display()));

    scan::src().unwrap_or_else(|()| std::process::exit(1));
    run("cargo fetch"                                );
    run("cargo check --frozen --workspace --all-targets" );
    run("cargo build --frozen --workspace --all-targets --exclude xtask");
    examples::update();
    run("cargo   doc --frozen --workspace --no-deps" );
    run("cargo  test --frozen --workspace"           );
}

fn check(mut args: std::env::Args) {
    let orig_path = args.next().expect("path");
    let mut path = orig_path.split(|ch| "/\\".contains(ch)).collect::<Vec<_>>();

    let (package, mut path) = match &mut path[..] {
        ["thindx", "examples", example_rs] => {
            if let Some(example) = example_rs.strip_suffix(".rs") {
                let mut cmd = Command::parse("cargo check --example").unwrap();
                cmd.arg(example);
                status!("Executing", "{}", cmd);
                cmd.status0().unwrap_or_else(|err| fatal!("{}", err));
                return;
            } else {
                fatal!("unable to check `{}`: not an expected package src folder", orig_path);
            }
        },
        ["thindx", "src", "headers", rest @ ..] => ("thindx", rest),
        [package, "src", rest @ ..]             => (*package, rest),
        _other                                  => fatal!("unable to check `{}`: not an expected package src folder", orig_path),
    };

    if let Some(last) = path.last_mut() {
        if !last.ends_with(".rs") {
            fatal!("unable to check `{}`: only rust-lang files supported", orig_path);
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
    status!("Executing", "{}", cmd);
    cmd.status0().unwrap_or_else(|err| fatal!("{}", err));
}



fn run(command: &str) {
    let mut c = cmd(command);
    status!("Running", "{:?}", c);
    c.status0().unwrap_or_else(|err| fatal!("{}", err));
}

fn cmd(original: &str) -> Command { Command::parse(original).unwrap_or_else(|err| fatal!("{}", err)) }
