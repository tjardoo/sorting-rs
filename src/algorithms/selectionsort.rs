use crate::Sorter;

pub struct Selectionsort {
    pub enumerate: bool,
}

impl Sorter for Selectionsort {
    fn sort<T>(&self, slice: &mut [T]) where T: Ord {
        for unsorted in 0..slice.len() {
            if self.enumerate {
                let smallest_in_rest = slice[unsorted..]
                    .iter()
                    .enumerate()
                    .min_by_key(|&(_, v) | v)
                    .map(|(i, _)| unsorted + i)
                    .expect("slice is empty");

                if unsorted != smallest_in_rest {
                    slice.swap(unsorted, smallest_in_rest);
                 }
            } else {
                let mut smallest_in_rest = unsorted;

                for i in (unsorted + 1)..slice.len() {
                    if slice[i] < slice[smallest_in_rest] {
                        smallest_in_rest = i;
                    }
                }

                if unsorted != smallest_in_rest {
                    slice.swap(unsorted, smallest_in_rest);
                }
            }
        }
    }
}

#[test]
fn it_works_without_enumerate() {
    let mut things = vec![4, 5, 2, 3, 1];

    Selectionsort {
        enumerate: false,
    }.sort(&mut things);

    assert_eq!(things, &[1, 2, 3, 4, 5]);
}

#[test]
fn it_works_with_enumerate() {
    let mut things = vec![4, 5, 2, 3, 1];

    Selectionsort {
        enumerate: true,
    }.sort(&mut things);

    assert_eq!(things, &[1, 2, 3, 4, 5]);
}
