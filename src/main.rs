use std::{thread::sleep, time::Duration};

const CLEAR: &str = "\x1B[2J\x1B[1;1H";

struct Unbounded;
struct Bounded {
    bound: usize,
    brackets: (char, char),
}

struct Progress<Iter, Bounded> {
    iter: Iter,
    i: usize,
    bound: Bounded,
}

impl<Iter> Progress<Iter, Unbounded> {
    fn new(iter: Iter) -> Self {
        Progress {
            iter: iter,
            i: 0,
            bound: Unbounded,
        }
    }
}

trait ProgressDisplay: Sized {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>);
}

impl ProgressDisplay for Bounded {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>) {
        let (l, r) = self.brackets;
        println!(
            "{}{}{}{}{}",
            CLEAR,
            l,
            "*".repeat(progress.i),
            " ".repeat(self.bound - progress.i),
            r
        )
    }
}

impl ProgressDisplay for Unbounded {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>) {
        println!("{}{}", CLEAR, "*".repeat(progress.i))
    }
}

impl<Iter> Progress<Iter, Unbounded>
where
    Iter: ExactSizeIterator,
{
    fn with_bound(self) -> Progress<Iter, Bounded> {
        let bound = Bounded {
            bound: self.iter.len(),
            brackets: ('[', ']'),
        };
        Progress {
            iter: self.iter,
            i: self.i,
            bound,
        }
    }
}

impl<Iter> Progress<Iter, Bounded>
where
    Iter: ExactSizeIterator,
{
    fn with_brackets(mut self, brackets: (char, char)) -> Self {
        self.bound.brackets = brackets;
        self
    }
}

impl<Iter, Bound> Iterator for Progress<Iter, Bound>
where
    Iter: Iterator,
    Bound: ProgressDisplay,
{
    type Item = Iter::Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.bound.display(&self);
        self.i += 1;
        self.iter.next()
    }
}

fn expensive_computation(_i: &usize) {
    sleep(Duration::from_secs(1));
}

trait ProgressIteratorExt: Sized {
    fn progress(self) -> Progress<Self, Unbounded>;
}

impl<Iter> ProgressIteratorExt for Iter
where
    Iter: Iterator,
{
    fn progress(self) -> Progress<Self, Unbounded> {
        Progress::new(self)
    }
}

fn main() {
    let count = vec![1, 2, 3];

    for i in count
        .iter()
        .progress()
        .with_bound()
        .with_brackets(('{', '}'))
    {
        expensive_computation(i)
    }
}
