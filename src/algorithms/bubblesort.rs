use crate::Sorter;

pub struct Bubblesort;

impl Sorter for Bubblesort {
    fn sort<T>(slice: &mut [T]) where T: Ord {
        for i in 0..slice.len() {
            for j in 0..slice.len() - 1 - i {
                if slice[j] > slice[j + 1] {
                    slice.swap(j, j + 1);
                }
            }
        }
    }
}

#[test]
fn it_works() {
    use crate::sort;

    let mut things = vec![4, 5, 2, 3, 1];

    sort::<_, Bubblesort>(&mut things);

    assert_eq!(things, &[1, 2, 3, 4, 5]);
}
