#[derive(Debug)]
pub struct StrSplit<'haystack, D> {
    remainder: Option<&'haystack str>,
    delimiter: D,
}

impl<'haystack, D> StrSplit<'haystack, D> {
    pub fn new(haystack: &'haystack str, delimiter: D) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

// Ref: https://doc.rust-lang.org/std/str/struct.Split.html
pub trait Delimiter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

impl<'haystack, D> Iterator for StrSplit<'haystack, D>
where
    D: Delimiter,
{
    type Item = &'haystack str;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut remainder) = self.remainder {
            if let Some((delim_start, delim_end)) = self.delimiter.find_next(remainder) {
                let until_delimiter = &remainder[..delim_start];
                *remainder = &remainder[delim_end..];
                Some(until_delimiter)
            } else {
                self.remainder.take()
            }
        } else {
            None
        }
    }
}

impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start, start + self.len()))
    }
}

impl Delimiter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices()
            .find(|(_, c)| c == self)
            .map(|(start, _)| (start, start + self.len_utf8()))
    }
}

fn until_char<'a>(s: &'a str, c: char) -> &'a str {
    StrSplit::new(s, c)
        .next()
        .expect("StrSplit always gives at least one result")
}

fn until_char2<'a>(s: &'a str, c: char) -> &'a str {
    let delim = format!("{}", c);
    StrSplit::new(s, &*delim)
        .next()
        .expect("StrSplit always gives at least one result")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let haystack = "a b c d e";
        let letters: Vec<_> = StrSplit::new(haystack, " ").collect();

        assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
    }

    #[test]
    fn tail_works() {
        let haystack = "a b c d ";
        let letters: Vec<_> = StrSplit::new(haystack, " ").collect();

        assert_eq!(letters, vec!["a", "b", "c", "d", ""]);
    }

    #[test]
    fn one_string_works() {
        let haystack = "a";
        let letters: Vec<_> = StrSplit::new(haystack, " ").collect();

        assert_eq!(letters, vec!["a"]);
    }

    #[test]
    fn empty_string_works() {
        let haystack = "";
        let letters: Vec<_> = StrSplit::new(haystack, " ").collect();

        assert_eq!(letters, vec![""]);
    }

    #[test]
    fn until_char_test() {
        assert_eq!(until_char("hello world", 'o'), "hell");
    }
}
