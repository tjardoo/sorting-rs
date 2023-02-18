pub trait Sorter {
    fn sort<T>(slice: &mut [T]) where T: Ord;
}

pub fn sort<T, S>(slice: &mut [T])
    where
        T: Ord,
        S: Sorter,
{
    S::sort(slice)
}

mod algorithms;

#[cfg(test)]
mod tests {
    use crate::{Sorter, sort};

    struct StdSorter;

    impl Sorter for StdSorter {
        fn sort<T>(slice: &mut [T]) where T: Ord {
            slice.sort()
        }
    }

    #[test]
    fn std_sorter_works() {
        let mut things = vec![4, 2, 3, 1];

        sort::<_, StdSorter>(&mut things);

        assert_eq!(things, &[1, 2, 3, 4]);
    }
}
