#[macro_export]
macro_rules! avec {
    ($($element:expr),* $(,)?) => {{
        #[allow(unused_mut)]
        let mut vs = Vec::new();
        $(vs.push($element);)*
        vs
    }};
    ($element:expr; $count:expr) => {{
        let mut vs = Vec::new();
        let x = $element;
        for _ in 0..$count {
            vs.push(x.clone());
        }
        vs
    }};
}

trait MaxValue {
    fn max_value() -> Self;
}

macro_rules! max_impl {
    ($t:ty) => {
        impl MaxValue for $t {
            fn max_value() -> Self {
                <$t>::MAX
            }
        }
    };
}

max_impl!(u32);
max_impl!(u64);
max_impl!(i32);
max_impl!(i64);

/// ```compile_fail
/// let x: Vec<32> = vecmac::avec![42; "foo"];
/// ```
#[allow(dead_code)]
struct CompileFailTest;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_vec() {
        let x: Vec<u32> = avec![];
        assert!(x.is_empty());
    }

    #[test]
    fn single() {
        let x: Vec<u32> = avec![42];
        assert!(!x.is_empty());
        assert_eq!(x.len(), 1);
        assert_eq!(x[0], 42);
    }

    #[test]
    fn double() {
        let x: Vec<u32> = avec![42, 43];
        assert!(!x.is_empty());
        assert_eq!(x.len(), 2);
        assert_eq!(x[0], 42);
        assert_eq!(x[1], 43);
    }

    #[test]
    fn trailing() {
        let x: Vec<&str> = avec![
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        ];
        assert!(!x.is_empty());
    }

    #[test]
    fn clone_2() {
        let x: Vec<u32> = avec![42; 2];
        assert!(!x.is_empty());
        assert_eq!(x.len(), 2);
        assert_eq!(x[0], 42);
        assert_eq!(x[1], 42);
    }

    #[test]
    fn clone_2_nonliteral() {
        let mut y = Some(42);
        let x: Vec<u32> = avec![y.take().unwrap(); 2];
        assert!(!x.is_empty());
        assert_eq!(x.len(), 2);
        assert_eq!(x[0], 42);
        assert_eq!(x[1], 42);
    }
}
