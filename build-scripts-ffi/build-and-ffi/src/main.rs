mod foo {
    include!(concat!(env!("OUT_DIR"), "/foo.rs"));
}

// #[cfg(feature = "foo")]
// fn foo() {}
//
// #[cfg(feature = "openssl_1_1_0")]
// fn bar() {}
//
// rustc --cfg=feature=foo --cfg=openssl_1_1_0

fn main() {
    println!("Hello, world!");
    println!("{}", env!("OUT_DIR"));
    println!("{}", env!("OUT_DIR"));

    foo::foo();

    r#static();
}

// static statik
// crate krate
fn r#static() {}
