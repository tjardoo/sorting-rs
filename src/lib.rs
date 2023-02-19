pub trait Sorter {
    fn sort<T>(&self, slice: &mut [T]) where T: Ord;
}

pub mod algorithms;

pub struct StdSorter;

impl Sorter for StdSorter {
    fn sort<T>(&self, slice: &mut [T]) where T: Ord {
        slice.sort();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn std_sorter_works() {
        let mut things = vec![4, 2, 3, 1];

        StdSorter.sort(&mut things);

        assert_eq!(things, &[1, 2, 3, 4]);
    }
}
