fn main() {
    let unstable = std::env::var_os("CARGO_PKG_VERSION_PRE").map_or(false, |v| !v.is_empty());
    if unstable {
        println!("cargo:rustc-cfg={}", "unstable");
    } else {
        println!("cargo:rustc-cfg={}", "stable");
    }
}
