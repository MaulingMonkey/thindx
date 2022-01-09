mod examples;
mod headers;
mod scan;
mod vsc;

use std::path::*;
use mmrbi::*;



fn main() {
    let mut args = std::env::args();
    let _exe = args.next();

    let cmd = args.next().expect("command");
    match cmd.as_str() {
        "b" | "build"   => build(args),
        "c" | "check"   => check(args),
        "d" | "doc"     => doc(args, false),
        "h" | "help"    => doc(args, true),
        "vsc"           => vsc::vsc(args),
        other           => fatal!("unknown command {:?}", other),
    }
}

fn build(_args: std::env::Args) {
    copy_thindx_files();
    scan::src().unwrap_or_else(|()| std::process::exit(1));
    run("cargo fetch");
    run("cargo check --frozen --workspace --all-targets");
    run("cargo build --frozen --workspace --all-targets");
    run("cargo build --frozen --workspace --all-targets --all-features");
    examples::update();
    headers::update();
    run("cargo   doc --frozen --workspace --all-features");
    run("cargo  test --frozen --workspace --all-features");
}

fn check(mut args: std::env::Args) {
    let orig_path = args.next().expect("path");
    let mut path = orig_path.split(|ch| "/\\".contains(ch)).collect::<Vec<_>>();

    let (package, mut path) = match &mut path[..] {
        ["xtask", ..]               => return, // already tested by building xtask
        ["thindx", "Cargo.toml"]    => return run("cargo check -p thindx"),

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

fn doc(_args: std::env::Args, help: bool) {
    copy_thindx_files();
    run("cargo build --examples");
    examples::update();
    headers::update();
    if help {
        run("cargo doc -p thindx --all-features --open");
    } else {
        run("cargo doc -p thindx --all-features");
    }
}



fn run(command: &str) {
    let mut c = cmd(command);
    status!("Running", "{:?}", c);
    c.status0().unwrap_or_else(|err| fatal!("{}", err));
}

fn cmd(original: &str) -> Command { Command::parse(original).unwrap_or_else(|err| fatal!("{}", err)) }

fn copy_thindx_files() {
    for file in "LICENSE-APACHE LICENSE-MIT LICENSE.md Readme.md".split(' ') {
        let from = Path::new(file);
        let to = Path::new("thindx").join(file);
        if from.metadata().and_then(|m| m.modified()).ok() > to.metadata().and_then(|m| m.modified()).ok() {
            if let Err(err) = std::fs::copy(&from, &to) {
                error!("unable to copy {} to thindx/{}: {}", from.display(), to.display(), err);
            }
        }
    }
}
