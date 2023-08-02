fn main() {
    println!("cargo:warning=Generating foo.rs");
    std::fs::write(
        std::path::Path::new(&std::env::var("OUT_DIR").unwrap()).join("foo.rs"),
        r"pub fn foo() {}",
    )
    .unwrap();
    println!("cargo:include=foo");
}
