use std::fmt::Debug;

struct Rc<T> {
    inner: *mut Inner<T>,
}

struct Inner<T> {
    count: usize,
    value: T,
}

impl<T> Rc<T> {
    pub fn new(v: T) -> Self {
        Rc {
            inner: Box::into_raw(Box::new(Inner { count: 1, value: v })),
        }
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        unsafe { &mut *self.inner }.count += 1;
        Rc { inner: self.inner }
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let cnt = &mut unsafe { &mut *self.inner }.count;
        if *cnt == 1 {
            let _ = unsafe { Box::from_raw(self.inner) };
        } else {
            *cnt -= 1;
        }
    }
}

impl<T> std::ops::Deref for Rc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &unsafe { &*self.inner }.value
    }
}

// Error related to auto-traits
// fn foo(_: Box<dyn Iterator<Item = ()>> + Debug) {}

trait IteratorAndDebug: Iterator<Item = ()> + Debug {}
fn foo(_: Box<dyn IteratorAndDebug>) {}

fn main() {
    let x = Rc::new(1);
    let y = x.clone();
    std::thread::spawn(move || {
        drop(y);
    });
    drop(x);
}
