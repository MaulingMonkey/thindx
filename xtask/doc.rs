use crate::*;
use mmrbi::*;
use std::io;



pub fn build(_args: std::env::Args, help: bool) {
    copy_thindx_files();
    run("cargo build --examples");
    examples::update();
    headers::update();
    run(r"cargo doc --no-deps --workspace --all-features --target-dir=target\all-features");
    fixup();
    if !help { return }
    run(r"cargo doc --no-deps -p thindx   --all-features --target-dir=target\all-features --open");
}

fn fixup() {
    status!("Fixing", "{}", "target/all-features/doc/**/*.{css, html}");
    let _ = fixup_path(Path::new("target/all-features/doc")).map_err(|err| warning!("error fixing up docs: {}", err));
}

fn fixup_path(path: &Path) -> io::Result<()> {
    if path.is_dir() && !".git target vs".split(' ').any(|skippable| path.ends_with(skippable)) {
        for e in std::fs::read_dir(path)? {
            let e = e?;
            let path = e.path();
            fixup_path(&path)?;
        }
    }

    if path.is_file() {
        let path_str = path.to_string_lossy();
        let path_str = path_str.replace("\\", "/");
        let path_lower = path_str.to_ascii_lowercase();
        let name_lower = path_lower.rsplit_once('/').map_or(path_lower.as_str(), |s| s.1);

        if name_lower.ends_with(".html") {
            rewrite_file(&path, |html| html.replace(r#"<span class="kw">unsafe</span>"#, r#"<span class="kw unsafe">unsafe</span>"#))?;
        } else if name_lower.ends_with(".css") {
            let (find, replace) = match name_lower {
                "ayu.css"   => (".lifetime{color:#ff7733;}",       ".lifetime{color:#ff7733} pre.rust .kw.unsafe{font-weight: bold; color: #F00;}"),
                "light.css" => ("pre.rust .kw{color:#8959A8;}", "pre.rust .kw{color:#8959A8} pre.rust .kw.unsafe{font-weight: bold; color: #F00;}"),
                "dark.css"  => ("pre.rust .kw{color:#ab8ac1;}", "pre.rust .kw{color:#ab8ac1} pre.rust .kw.unsafe{font-weight: bold; color: #F00;}"),
                _           => return Ok(()),
            };
            rewrite_file(&path, |css| css.replacen(find, replace, 1))?;
        }
        // else ignored file
    }

    Ok(())
}

fn rewrite_file(path: &Path, f: impl FnOnce(String) -> String) -> io::Result<()> {
    let text = std::fs::read_to_string(&path)?;
    let text = f(text);
    std::fs::write(&path, text)?;
    Ok(())
}
