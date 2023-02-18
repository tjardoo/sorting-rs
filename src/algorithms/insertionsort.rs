use crate::Sorter;

pub struct Insertionsort;

impl Sorter for Insertionsort {
    fn sort<T>(&self, slice: &mut [T]) where T: Ord {
        for unsorted in 1..slice.len() {
            let mut i = unsorted;

            while i > 0 && slice[i - 1] > slice[i] {
                slice.swap(i - 1, i);

                i -= 1;
            }
        }
    }
}

#[test]
fn it_works() {
    let mut things = vec![4, 5, 2, 3, 1];

    Insertionsort.sort(&mut things);

    assert_eq!(things, &[1, 2, 3, 4, 5]);
}
