fn main() {
    // XXX: Might get rid of this, might configure this...
    for ver in 1..=47 {
        println!("cargo:rustc-cfg=d3dcompiler=\"{}\"", ver);
    }
    println!("cargo:rustc-cfg=d3dcompiler=\"test\"");
}
