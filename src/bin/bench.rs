use sorting::{algorithms::*, Sorter};

use std::cmp::Ordering;
use std::cell::Cell;
use std::rc::Rc;
use rand::prelude::*;

#[derive(Clone)]
struct SortEvaluator<T> {
    t: T,
    cmps: Rc<Cell<usize>>,
}

impl<T: PartialEq> PartialEq for SortEvaluator<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cmps.set(self.cmps.get() + 1);

        self.t == other.t
    }
}

impl<T: Eq> Eq for SortEvaluator<T> {}

impl<T: PartialOrd> PartialOrd for SortEvaluator<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cmps.set(self.cmps.get() + 1);

        self.t.partial_cmp(&other.t)
    }
}

impl<T: Ord> Ord for SortEvaluator<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cmps.set(self.cmps.get() + 1);

        self.t.cmp(&other.t)
    }
}

fn main() {
    let mut rand = rand::thread_rng();

    let counter = Rc::new(Cell::new(0));

    for &n in &[0, 1, 10, 100, 1000] {
        let mut values = Vec::with_capacity(n);

        for _ in 0..n {
            values.push(SortEvaluator {
                t: rand.gen::<usize>(),
                cmps: Rc::clone(&counter),
            });
        }

        for _ in 0..10 {
            values.shuffle(&mut rand);

            let took = bench(Bubblesort, &values, &counter);
            println!("{} {} {}", "bubble", n, took);

            let took = bench(Insertionsort, &values, &counter);
            println!("{} {} {}", "insertion", n, took);

            let took = bench(Selectionsort { enumerate: false }, &values, &counter);
            println!("{} {} {}", "selection (without enumerate)", n, took);

            let took = bench(Selectionsort { enumerate: true }, &values, &counter);
            println!("{} {} {}", "selection (with enumerate)", n, took);

            let took = bench(Quicksort, &values, &counter);
            println!("{} {} {}", "quick", n, took);
        }
    }
}

fn bench<T:Ord + Clone, S: Sorter>(
    sorter: S,
    values: &[SortEvaluator<T>],
    counter: &Cell<usize>,
) -> usize {
    let mut values = values.to_vec();

    counter.set(0);
    sorter.sort(&mut values);

    let count = counter.get();

    // assert!(values.is_sorted());
    for i in 1..values.len() {
        assert!(values[i] >= values[i - 1]);
    }

    count
}
