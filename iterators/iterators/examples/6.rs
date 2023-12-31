fn main() {}

// Remove into_iter from tests

pub fn flattern<I>(iter: I) -> Flatten<I::IntoIter>
where
    I: IntoIterator,
    I::Item: IntoIterator,
{
    Flatten::new(iter.into_iter())
}

pub struct Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    outer: O,
    inner: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    fn new(iter: O) -> Self {
        Flatten {
            outer: iter,
            inner: None,
        }
    }
}

impl<O> Iterator for Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut inner_iter) = self.inner {
                if let Some(i) = inner_iter.next() {
                    return Some(i);
                }
                self.inner = None
            }

            let next_inner_iter = self.outer.next()?.into_iter();
            self.inner = Some(next_inner_iter);
        }
    }
}

impl<O> DoubleEndedIterator for Flatten<O>
where
    // O: Iterator + DoubleEndedIterator,
    O: DoubleEndedIterator, // DoubleEndedIterator implies Iterator
    O::Item: IntoIterator,
    <O::Item as IntoIterator>::IntoIter: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut inner_iter) = self.inner {
                if let Some(i) = inner_iter.next_back() {
                    return Some(i);
                }
                self.inner = None
            }

            let next_inner_iter = self.outer.next_back()?.into_iter();
            self.inner = Some(next_inner_iter);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(flattern(std::iter::empty::<Vec<()>>()).count(), 0);
    }

    #[test]
    fn empty_wide() {
        assert_eq!(flattern(vec![Vec::<()>::new(), vec![], vec![]]).count(), 0);
    }

    #[test]
    fn one() {
        assert_eq!(flattern(std::iter::once(vec!["a"])).count(), 1);
    }

    #[test]
    fn two() {
        assert_eq!(flattern(std::iter::once(vec!["a", "b"])).count(), 2);
    }

    #[test]
    fn two_wide() {
        assert_eq!(flattern(vec![vec!["a"], vec!["b"]]).count(), 2);
    }

    #[test]
    fn three_wide() {
        assert_eq!(
            flattern(vec![vec!["a"], vec!["b"], vec!["c", "d"]]).count(),
            4
        );
    }

    #[test]
    fn reverse() {
        assert_eq!(
            flattern(std::iter::once(vec!["a", "b"]))
                .rev()
                .collect::<Vec<_>>(),
            vec!["b", "a"]
        );
    }

    #[test]
    fn reverse_wide() {
        assert_eq!(
            flattern(vec![vec!["a"], vec!["b"]])
                .rev()
                .collect::<Vec<_>>(),
            vec!["b", "a"]
        );
    }

    #[test]
    fn both_ends() {
        let mut iter = flattern(vec![vec!["a", "b"], vec!["c", "d"]]);
        assert_eq!(iter.next(), Some("a"));
        assert_eq!(iter.next_back(), Some("d")); // failed: we get b
        assert_eq!(iter.next(), Some("b"));
        assert_eq!(iter.next_back(), Some("c"));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }
}
