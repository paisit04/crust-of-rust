fn main() {}

pub fn flattern<I>(iter: I) -> Flatten<I> {
    Flatten::new(iter)
}

pub struct Flatten<O> {
    outer: O,
}

impl<O> Flatten<O> {
    fn new(iter: O) -> Self {
        Flatten { outer: iter }
    }
}

impl<O> Iterator for Flatten<O>
where
    O: Iterator,
    // <O as Iterator>::Item: IntoIterator,
    O::Item: IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        // self.outer.next().and_then(|inner| inner.into_iter().next())

        let inner_item = self.outer.next()?;
        let mut inner_iter = inner_item.into_iter();
        inner_iter.next()

        // Problem
        // It only return the first inner item
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
        assert_eq!(
            flattern(vec![Vec::<()>::new(), vec![], vec![]].into_iter()).count(),
            0
        );
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
        assert_eq!(flattern(vec![vec!["a"], vec!["b"]].into_iter()).count(), 2);
    }
}
