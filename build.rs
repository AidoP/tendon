fn main() {
    println!("cargo:rerun-if-changed=src/fb.c");
    cc::Build::new()
        .warnings(true)
        .file("src/fb.c")
        .compile("fb");
}
