fn strlen(s: impl AsRef<str>) -> usize {
    s.as_ref().len()
}

// Monomorphization
pub fn strlen_refstr(s: &str) -> usize {
    s.len()
}
pub fn strlen_string(s: String) -> usize {
    s.len()
}

pub fn bool_then<T>(b: bool, f: impl FnOnce() -> T) -> Option<T> {
    if b {
        Some(f())
    } else {
        None
    }
}

fn strlen2<S>(s: S) -> usize
where
    S: AsRef<str>,
{
    s.as_ref().len()
}

pub trait Hei {
    fn hei(&self);
}

impl Hei for &str {
    fn hei(&self) {
        println!("hei {}", self);
    }
}

impl Hei for String {
    fn hei(&self) {
        println!("hei {}", self);
    }
}

pub fn foo() {
    strlen("hello world"); // &'static str
    strlen(String::from("hei verden")); // String: AsRef<str>

    "J".hei();

    barr(&["J", "Jon"]);
    barr(&[String::from("J"), String::from("Jon")]);
}

// Dynamic (dynamic dispatch)
pub fn bar_dynamic(s: &dyn Hei) {
    // store in &
    // 1. a pointer to the actual, concrete, implementing type
    // 2. a pointer to a vtable for the referenced trait
    //
    // What is a vtable?
    //
    // dyn Hei, vtable:
    //   struct HeiVtable {
    //      hei: *mut Fn(*mut ())
    //   }
    //
    // &str -> &dyn Hei
    //
    // 1. pointer to &str
    // 2. &HeiVtable {
    //   hei: &<str as Hei>::hei
    // }
    //
    // &String -> &dyn Hei
    //
    // 1. pointer to the String
    // 2. &HeiVtable {
    //   hei: &<String as Hei>::hei
    // }
    s.hei();
    // s.vtable.hei(s.pointer)
}

// Static
// The same as
// pub fn bar<H: Hei>(h: H) {}
pub fn bar(h: impl Hei) {
    h.hei();
}

// static dispatch at compile time
// generic at one type
pub fn bar_str(h: &str) {
    h.hei();
}

pub fn barr(s: &[impl Hei]) {
    for h in s {
        h.hei();
    }
}

pub fn strlen_dyn(s: Box<dyn AsRef<str>>) -> usize {
    s.as_ref().as_ref().len()
}

pub fn strlen_dyn2(s: &dyn AsRef<str>) -> usize {
    s.as_ref().len()
}

pub trait HeiAsRef: Hei + AsRef<str> {}

pub fn baz(s: &dyn HeiAsRef) {
    s.hei();
    let s = s.as_ref();
    s.len();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let x = Box::new(String::from("hello"));
        let y: Box<dyn AsRef<str>> = x;
        strlen_dyn(y);

        let yy: &dyn AsRef<str> = &"world";
        strlen_dyn2(yy);
    }
}
