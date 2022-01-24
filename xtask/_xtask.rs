mod browser;
mod coverage;
mod crlf;
mod doc;
mod examples;
mod headers;
mod natvis;
mod publish;
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
        "d" | "doc"     => doc::build(args, false),
        "h" | "help"    => doc::build(args, true),
        "coverage"      => coverage::from_args(args),
        "publish"       => publish::publish(args),
        "vsc"           => vsc::vsc(args),
        other           => fatal!("unknown command {:?}", other),
    }
}

fn build(args: std::env::Args) {
    let mut args = args.peekable();
    if args.peek().is_none() {
        copy_thindx_files();
        scan::src().unwrap_or_else(|()| std::process::exit(1));
        run("cargo fetch");
        run("cargo build --frozen --workspace --all-targets");
        run(r"cargo build --frozen --workspace --all-targets --all-features --target-dir=target\all-features");
        doc::update();
        run(r"cargo test          --frozen --workspace --all-features --target-dir=target\all-features");
    } else {
        copy_thindx_files();
        let mut cmd = Command::new("cargo");
        cmd.arg("build");

        let mut examples = false;
        for arg in args {
            match &*arg {
                "--example"     => examples = true,
                "--examples"    => examples = true,
                "--all"         => examples = true,
                "--all-targets" => examples = true,
                "--workspace"   => examples = true,
                _               => {},
            }
            cmd.arg(arg);
        }

        if examples {
            examples::download_extract_assets();
        }

        status!("Running", "{:?}", cmd);
        cmd.status0().unwrap_or_else(|err| fatal!("{}", err));
    }
}

fn check(mut args: std::env::Args) {
    let orig_path = args.next().expect("path");
    let mut path = orig_path.split(|ch| "/\\".contains(ch)).collect::<Vec<_>>();

    let (package, mut path) = match &mut path[..] {
        ["xtask", ..]               => return, // already tested by building xtask
        ["thindx", "Cargo.toml"]    => return run("cargo check -p thindx"),
        ["thindx", "tests", test]   => return run(&format!("cargo test -p thindx --test {}", test.strip_suffix(".rs").unwrap_or(test))),

        ["thindx", "examples", example_rs] => {
            if let Some(example) = example_rs.strip_suffix(".rs") {
                let mut cmd = Command::parse("cargo check --example").unwrap();
                cmd.arg(example);
                status!("Running", "{:?}", cmd);
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
    let pattern1 = path.join("::").replace(".h::", "::");   // "d3d11shader.h" => "d3d11shader"
    let pattern2 = path.join("::").replace(".h::", "_h::"); // "d3d11shader.h" => "d3d11shader_h"

    let mut cmd = Command::new("cargo");
    cmd.arg("test");
    cmd.arg("-p").arg(package);
    cmd.arg("--").arg(pattern1).arg(pattern2);
    status!("Running", "{:?}", cmd);
    cmd.status0().unwrap_or_else(|err| fatal!("{}", err));
}

fn run(command: impl AsRef<str>) {
    let mut c = cmd(command);
    status!("Running", "{:?}", c);
    c.status0().unwrap_or_else(|err| fatal!("{}", err));
}

fn run_in(dir: &Path, command: impl AsRef<str>) {
    let mut c = cmd(command);
    c.current_dir(dir);
    status!("Running", "{:?}", c);
    c.status0().unwrap_or_else(|err| fatal!("{}", err));
}

fn cmd(original: impl AsRef<str>) -> Command { Command::parse(original).unwrap_or_else(|err| fatal!("{}", err)) }

fn copy_thindx_files() {
    natvis::generate_and_merge();
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
