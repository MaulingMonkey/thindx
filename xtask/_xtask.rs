mod examples;

use mmrbi::*;



fn main() {
    let mut args = std::env::args();
    let _exe = args.next();

    let cmd = args.next().expect("command");
    match cmd.as_str() {
        "build"     => build(args),
        "check"     => check(args),
        other       => fatal!("unknown command {:?}", other),
    }
}

fn build(_args: std::env::Args) {
    // for some reason we need an abs path here, despite not requiring one when invoking `cargo doc` directly from the CLI
    let css = std::env::current_dir().unwrap().join("thindx/doc/style.css");
    std::env::set_var("RUSTDOCFLAGS", format!(r"--extend-css {css}", css = css.display()));

    run("cargo        fetch"                                );
    run("cargo        check --frozen --workspace --all-targets" );
    run("cargo        build --frozen --workspace --all-targets --exclude xtask");
    examples::update();
    run("cargo +nightly doc --frozen --workspace --no-deps" );
    run("cargo         test --frozen --workspace"           );
}

fn check(mut args: std::env::Args) {
    let orig_path = args.next().expect("path");
    let mut path = orig_path.split(|ch| "/\\".contains(ch)).collect::<Vec<_>>();

    let (package, mut path) = match path.get_mut(0..3) {
        Some(["thindx", "src", "headers"])  => ("thindx", &mut path[3..]),
        Some([package, "src", ..])          => (*package, &mut path[2..]),
        _other                              => fatal!("unable to check `{}`: not an expected package src folder", orig_path),
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
    cmd.status0().unwrap_or_else(|err| fatal!("{}", err));
}



fn run(command: &str) {
    let mut c = cmd(command);
    status!("Running", "{:?}", c);
    c.status0().unwrap_or_else(|err| fatal!("{}", err));
}

fn cmd(original: &str) -> Command {
    let mut args = Vec::new();

    let mut s = original.chars().peekable();
    'parse_next_arg: loop {
        while s.peek() == Some(&' ') { let _space = s.next(); } // ignore leading whitespace
        if s.peek().is_none() { break }

        let mut arg = String::new();
        if s.peek() == Some(&'\"') { // arg is quoted
            let _quote = s.next();
            while let Some(ch) = s.next() {
                if ch == '\\' && s.peek() == Some(&'\"') { // escaped interior quote
                    let _esc_quote = s.next();
                    arg.push('\"');
                } else if ch == '\"' { // ending quote
                    args.push(arg);
                    continue 'parse_next_arg;
                } else {
                    arg.push(ch);
                }
            }
            fatal!("unable to parse `{}`: last quoted arg was never terminated", original);
        } else { // arg is unquoted
            while let Some(ch) = s.next() {
                if ch == ' ' { // ending space
                    args.push(arg);
                    continue 'parse_next_arg;
                } else if ch == '\"' {
                    fatal!("unable to parse `{}`: unquoted arg has interior quotes", original);
                } else {
                    arg.push(ch);
                }
            }
            args.push(arg);
        }
    }

    match &args[..] {
        [ exe, args @ .. ] => {
            let mut cmd = Command::new(exe);
            cmd.args(args);
            cmd
        },
        [] => fatal!("unable to parse `{}`: empty/blank string? no args were parsed", original),
    }
}
