pub trait Hei {
    fn hei(&self);

    fn weird()
    where
        Self: Sized, // Opt out from the vtable
    {
    }
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
    "J".hei();

    barr(&["J", "Jon"]);
    barr(&[String::from("J"), String::from("Jon")]);
}

// Dynamic (dynamic dispatch)
pub fn bar_dynamic(s: &dyn Hei) {
    s.hei();
    // s.weird();
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

// dyn Trait -> * -> (*mut data, *mut vtable)
// [u8]      -> * -> (*mut data, usize length)
// str       -> * -> (*mut data, usize length)

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
