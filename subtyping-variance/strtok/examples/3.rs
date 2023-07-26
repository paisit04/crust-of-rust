use std::marker::PhantomData;

// Generic over T, but not own or contain T
struct Deserializer<T> {
    // some fields
    _t: PhantomData<T>,
}

struct Deserializer2<T> {
    // some fields
    _t: PhantomData<fn() -> T>, // covariance, don't drop T
}

struct Deserializer3<T> {
    // some fields
    _t: PhantomData<fn(T)>, // contravariance
}

struct Deserializer32<T> {
    // some fields
    _t: PhantomData<*const T>, // contravariance
}

// T is invariance
struct Deserializer4<T> {
    // some fields
    _t1: PhantomData<fn(T)>,     // contravariance
    _t2: PhantomData<fn() -> T>, // covariance, don't drop T
}

struct Deserializer5<T> {
    // some fields
    _t: PhantomData<*mut T>, // invariance
}

// Ref: https://doc.rust-lang.org/stable/std/ptr/struct.NonNull.html
// Unlike *mut T, NonNull<T> was chosen to be covariant over T.

struct TouchDrop<T: std::fmt::Debug>(T);

impl<T: std::fmt::Debug> Drop for TouchDrop<T> {
    fn drop(&mut self) {
        println!("{:?}", self.0);
    }
}

fn main() {
    let x = String::new();
    // let z = vec![&x];
    let z = vec![TouchDrop(&x)];
    drop(x);
    // drop(z);
}
