pub trait Hei {
    type Name;

    fn hei(&self);
}

impl Hei for &str {
    type Name = ();

    fn hei(&self) {
        println!("hei {}", self);
    }
}

impl Hei for String {
    type Name = ();

    fn hei(&self) {
        println!("hei {}", self);
    }
}

pub fn foo() {
    "J".hei();

    barr(&["J", "Jon"]);
    barr(&[String::from("J"), String::from("Jon")]);
}

// Dynamic (dynamic dispatch)
pub fn bar_dynamic(s: &dyn Hei<Name = ()>) {
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

pub trait HeiAsRef: Hei<Name = ()> + AsRef<str> {}

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
