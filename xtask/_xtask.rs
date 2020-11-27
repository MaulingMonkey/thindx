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
            if let Some(first) = path.first() {
                if *first == "src" {
                    path.remove(0);
                }
            }
            if let Some(last) = path.last_mut() {
                if !last.ends_with(".rs") {
                    panic!("only rust files supported");
                }
                *last = &last[..last.len()-3];
                if last.starts_with("_") {
                    path.pop(); // "foo\_foo.rs" => "foo"
                }
            }
            let pattern = path.join("::");
            assert_eq!(0, Command::new("cargo").arg("test").arg("--").arg(pattern).status().expect("cargo test").code().expect("code"));
        },
        other => panic!("unknown command {:?}", other),
    }
}
