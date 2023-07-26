// Ref: https://cplusplus.com/reference/cstring/strtok/
pub fn strtok<'a, 'b>(s: &'a mut &'b str, delimiter: char) -> &'b str {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut x = "hello world";

        let z = &mut x; // &'x mut -> &'until-ZZZ mut
                        // until-ZZZ: borrow of x stops here

        // strtok<'a, 'b> (&'a mut &'b str)      -> &'b str
        // strtok         (&'a mut &'static str) -> &'static str
        let hello = strtok(&mut x, ' ');

        assert_eq!(hello, "hello");
        assert_eq!(x, "world");
    }
}
