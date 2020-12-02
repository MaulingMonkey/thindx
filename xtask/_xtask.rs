// xtasks.rs

use std::process::*;

fn main() {
    let mut args = std::env::args();
    let _exe = args.next();

    let cmd = args.next().expect("command");
    match cmd.as_str() {
        "check" => {
            let orig_path = args.next().expect("path");
            let mut path = orig_path.split(|ch| "/\\".contains(ch)).collect::<Vec<_>>();

            let (package, mut path) = match path.get_mut(0..3) {
                Some(["thindx", "src", "headers"])  => ("thindx", &mut path[3..]),
                Some([package, "src", ..])          => (*package, &mut path[2..]),
                _other                              => panic!("unable to check `{}`: not an expected package src folder", orig_path),
            };

            if let Some(last) = path.last_mut() {
                if !last.ends_with(".rs") {
                    panic!("unable to check `{}`: only rust-lang files supported", orig_path);
                }
                *last = &last[..last.len()-3];
                if last.starts_with("_") {
                    let n = path.len();
                    path = &mut path[..n-1]; // "foo\_foo.rs" => "foo"
                }
            }
            let pattern = path.join("::").replace(".", "_"); // "d3d11shader.h" => "d3d11shader_h"
            //panic!("{:?} => {:?}", orig_path, pattern);

            let mut cmd = Command::new("cargo");
            cmd.arg("test");
            cmd.arg("-p").arg(package);
            cmd.arg("--").arg(pattern);
            assert_eq!(0, cmd.status().expect("cargo test").code().expect("code"));
        },
        other => panic!("unknown command {:?}", other),
    }
}
