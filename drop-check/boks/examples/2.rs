use std::marker::PhantomData;

pub struct Boks<T> {
    p: *mut T, // invariant
    _t: PhantomData<T>,
}

// Cannot treat Boks<&'static str> as Boks<&'some_shorter_lifetime str>
// even though &'static str as &'some_shorter_lifetime str
// and can treat Box<&'static str> as Box<&'some_shorter_lifetime str>

impl<T> Drop for Boks<T> {
    fn drop(&mut self) {
        // Safety: p was constructed from a Box in the first place, and has not been freed.
        // otherwise since self still exists (otherwise, drop could not be called)
        unsafe { Box::from_raw(self.p) };
        // unsafe { std::ptr::drop_in_place(self.p) }; // drop the p, but not free the box
    }
}

impl<T> Boks<T> {
    pub fn ny(t: T) -> Self {
        Boks {
            p: Box::into_raw(Box::new(t)),
            _t: PhantomData,
        }
    }
}

impl<T> std::ops::Deref for Boks<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // Safety: is valid since it was constructed from a valid T, and turned into a pointer
        // through Box which creates aligned pointers, and hasn't been freed, since self is alive.
        unsafe { &*self.p }
    }
}

impl<T> std::ops::DerefMut for Boks<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Safety: is valid since it was constructed from a valid T, and turned into a pointer
        // through Box which creates aligned pointers, and hasn't been freed, since self is alive.
        // Also, since we have &mut self, no other mutable reference has been given out to p.
        unsafe { &mut *self.p }
    }
}

use std::fmt::Debug;

struct Oisann<T: Debug>(T);

impl<T: Debug> Drop for Oisann<T> {
    fn drop(&mut self) {
        println!("{:?}", self.0);
    }
}

fn main() {
    let x = 42;
    let b = Boks::ny(x);
    println!("{:?}", *b);

    let mut y = 42;
    let b = Boks::ny(&mut y);
    // println!("{:?}", y);
    // y = 43;
    // drop(b);

    // let mut z = 42;
    // let b = Boks::ny(Oisann(&mut z));
    // println!("{:?}", z);

    let s = String::from("hei");
    let mut box1 = Box::new(&*s);
    let box2: Box<&'static str> = Box::new("heisan");
    box1 = box2;

    // let s = String::from("hei");
    // let mut boks1 = Boks::ny(&*s);
    // let boks2: Boks<&'static str> = Boks::ny("heisan");
    // boks1 = boks2;
}
