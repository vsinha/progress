use std::{thread::sleep, time::Duration};

// TODO vsinha: instead of clearing the whole screen, just clear the last line we printed
const CLEAR: &str = "\x1B[2J\x1B[1;1H";

// ascii 'height' order: _,.-~+*=<%^'"`

struct Unbounded;
struct Bounded<'a> {
    bound: usize,
    brackets: (&'a str, &'a str),
}

struct Progress<'a, Iter, Bounded> {
    iter: Iter,
    i: usize,
    indicator: &'a str,
    bound: Bounded,
}

impl<Iter> Progress<'_, Iter, Unbounded> {
    fn new(iter: Iter) -> Self {
        Progress {
            iter: iter,
            i: 0,
            indicator: "*",
            bound: Unbounded,
        }
    }
}

trait ProgressDisplay: Sized {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>);
}

impl ProgressDisplay for Bounded<'_> {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>) {
        let (l, r) = self.brackets;
        println!(
            "{}{}{}{}{}",
            CLEAR,
            l,
            progress.indicator_as_str(),
            " ".repeat(progress.indicator.len() * (self.bound - progress.i)),
            r
        )
    }
}

impl ProgressDisplay for Unbounded {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>) {
        println!("{}{}", CLEAR, progress.indicator_as_str(),)
    }
}

impl<Iter> Progress<'static, Iter, Unbounded>
where
    Iter: ExactSizeIterator,
{
    fn with_bound(self) -> Progress<'static, Iter, Bounded<'static>> {
        let bound = Bounded {
            bound: self.iter.len(),
            brackets: ("[", "]"),
        };
        Progress {
            iter: self.iter,
            i: self.i,
            indicator: self.indicator,
            bound,
        }
    }
}

impl<'a, Iter, Bound> Progress<'a, Iter, Bound> {
    fn with_indicator(mut self, indicator: &'a str) -> Self {
        self.indicator = indicator;
        self
    }

    fn indicator_as_str(&self) -> String {
        self.indicator.repeat(self.i)
    }
}

impl<'a, Iter> Progress<'a, Iter, Bounded<'a>>
where
    Iter: ExactSizeIterator,
{
    fn with_brackets(mut self, brackets: (&'a str, &'a str)) -> Self {
        self.bound.brackets = brackets;
        self
    }
}

impl<Iter, Bound> Iterator for Progress<'_, Iter, Bound>
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
    fn progress(self) -> Progress<'static, Self, Unbounded>;
}

impl<Iter> ProgressIteratorExt for Iter
where
    Iter: Iterator,
{
    fn progress(self) -> Progress<'static, Self, Unbounded> {
        Progress::new(self)
    }
}

fn main() {
    let count = vec![1, 2, 3];

    for i in count
        .iter()
        .progress()
        .with_indicator("_")
        .with_bound()
        .with_brackets(("<|", "|>"))
    {
        expensive_computation(i)
    }
}
