use std::{thread::sleep, time::Duration};

const CLEAR: &str = "\x1B[2J\x1B[1;1H";

struct Progress<Iter> {
    iter: Iter,
    i: usize,
    bound: Option<usize>,
}

impl<Iter> Progress<Iter> {
    fn new(iter: Iter) -> Self {
        Progress {
            iter: iter,
            i: 0,
            bound: None,
        }
    }
}

impl<Iter> Progress<Iter>
where
    Iter: ExactSizeIterator,
{
    fn with_bound(mut self) -> Self {
        self.bound = Some(self.iter.len());
        self
    }
}

impl<Iter> Iterator for Progress<Iter>
where
    Iter: Iterator,
{
    type Item = Iter::Item;
    fn next(&mut self) -> Option<Self::Item> {
        match self.bound {
            None => println!("{}{}", CLEAR, "*".repeat(self.i)),
            Some(bound) => {
                println!(
                    "{}<{}{}>",
                    CLEAR,
                    "*".repeat(self.i),
                    " ".repeat(bound - self.i)
                )
            }
        }
        self.i += 1;
        self.iter.next()
    }
}

fn expensive_computation(_i: &usize) {
    sleep(Duration::from_secs(1));
}

trait ProgressIteratorExt: Sized {
    fn progress(self) -> Progress<Self>;
}

impl<Iter> ProgressIteratorExt for Iter
where
    Iter: Iterator,
{
    fn progress(self) -> Progress<Self> {
        Progress::new(self)
    }
}

fn main() {
    let count = vec![1, 2, 3];

    for i in count.iter().progress().with_bound() {
        expensive_computation(i)
    }
}
