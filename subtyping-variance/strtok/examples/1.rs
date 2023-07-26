// Ref: https://cplusplus.com/reference/cstring/strtok/
pub fn strtok<'a>(s: &'a mut &'a str, delimiter: char) -> &'a str {
    if let Some(i) = s.find(delimiter) {
        let prefix = &s[..i];
        let suffix = &s[(i + delimiter.len_utf8())..];
        *s = suffix;
        prefix
    } else {
        let prefix = *s;
        *s = "";
        prefix
    }
}

// T: U
// Not a formal explaination.
// T is a subtype of U if T is at least as useful as U.
//
// 'static: 'a
//
// Ref: https://doc.rust-lang.org/nomicon/subtyping.html
// In Java,
// class Animal;
// class Cat: Animal;
//
// Cat: Animal
//
// covariance
// fn foo(&'a str) {}
// foo(&'a str)
// foo(&'static str)
//
// contravariance
// fn foo(bar: Fn(&'a str) -> ()) {
//    bar("" /* 'a */)
// }
// let x = Fn(&'a str) -> ()
// foo(fn(&'static str) {})
//
// &'static str // more useful
// &'a str
//
// 'static <: 'a
// &'static T <: &'a T
//
// Fn(&'static str)
// Fn(&'a str) // more useful
//
// 'static <: 'a
// Fn(&'a T) <: Fn(&'static T)
//
// invariance
//
// fn foo(s: &mut &'a str, x: &'a str) {
//   *s = x;
// }
// let mut x: &'static str = "hello world";
// let z = String::new();
// foo(&mut x, &z);
//   // foo(&mut &'a      str, &'a str)
//   // foo(&mut &'static str, &'a str)
// drop(z);
// println!("{}", x);
//

pub fn bar() {
    let mut y = true;
    let mut z = &mut y; // &'y mut bool

    let x = Box::new(true);
    let x: &'static mut bool = Box::leak(x);

    // ignore this line
    let _ = z;

    z = x; // &'y mut bool = &'static mut bool

    // ignore this line
    drop(z);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        fn check_is_static(_: &'static str) {}

        let mut x = "hello world";

        check_is_static(x);

        // &'a mut &'a str
        // &'a mut &'static str
        let hello = strtok(&mut x, ' ');
        assert_eq!(hello, "hello");
        assert_eq!(x, "world");
    }
}
