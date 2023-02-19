use crate::Sorter;

pub struct Quicksort;

fn quicksort<T: Ord>(slice: &mut [T]) {
    match slice.len() {
        0 | 1 => return,
        2 => {
            if slice[0] > slice[1] {
                slice.swap(0, 1);
            }

            return;
        },
        _ => {},
    }

    let (pivot, rest) = slice.split_first_mut().expect("slice not empty");

    let mut left = 0;
    let mut right = rest.len() - 1;

    while left <= right {
        if &rest[left] <= pivot {
            left += 1;
        }  else if &rest[right] > pivot {
            right -= 1;
        } else {
            rest.swap(left, right);

            left += 1;
            right -= 1;
        }
    }

    let left = left + 1;

    slice.swap(0, left - 1);

    let (left, right) = slice.split_at_mut(left - 1);

    quicksort(left);
    quicksort(&mut right[1..]);
}

impl Sorter for Quicksort {
    fn sort<T>(&self, slice: &mut [T]) where T: Ord {
        quicksort(slice);
    }
}

#[test]
fn it_works() {
    let mut things = vec![4, 5, 2, 3, 1];

    Quicksort.sort(&mut things);

    assert_eq!(things, &[1, 2, 3, 4, 5]);
}
