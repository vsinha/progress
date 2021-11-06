use std::{thread::sleep, time::Duration};

// TODO vsinha: instead of clearing the whole screen, just clear the last line we printed
const CLEAR: &str = "\x1B[2J\x1B[1;1H";

// ascii 'height' order: _,.-~+*=<%^'"`

struct Unbounded;
struct Bounded<'a> {
    bound: usize,
    brackets: (&'a str, &'a str),
}

struct SimpleIndicator<'a> {
    string: &'a str,
}

struct RotatingIndicator<'a> {
    i: usize,
    strings: Vec<&'a str>,
}

struct Progress<Iter, BoundMode, IndicatorMode> {
    iter: Iter,
    i: usize,
    indicator: IndicatorMode,
    bound: BoundMode,
}

trait Indicator {
    fn curr(&self) -> &str;

    fn advance(&mut self);

    fn as_str(&self, i: usize) -> String;
}

impl<'a> Indicator for SimpleIndicator<'a> {
    fn curr(&self) -> &str {
        self.string
    }

    fn advance(&mut self) {
        // do nothing
    }

    fn as_str(&self, i: usize) -> String {
        self.string.repeat(i)
    }
}

impl<'a> Indicator for RotatingIndicator<'a> {
    fn curr(&self) -> &str {
        self.strings[self.i]
    }

    fn advance(&mut self) {
        self.i += 1;
        self.i %= self.strings.len();
    }

    fn as_str(&self, i: usize) -> String {
        self.strings[self.i].repeat(i)
    }
}

impl<'a, Iter, Ind> Progress<Iter, Unbounded, Ind>
where
    Ind: Indicator,
{
    fn new(iter: Iter, indicator: Ind) -> Self {
        Progress {
            iter: iter,
            i: 0,
            indicator: indicator,
            bound: Unbounded,
        }
    }
}

trait ProgressDisplay: Sized {
    fn display<Iter, IndicatorMode>(&self, progress: &Progress<Iter, Self, IndicatorMode>)
    where
        IndicatorMode: Indicator;
}

impl ProgressDisplay for Bounded<'_> {
    fn display<Iter, IndicatorMode>(&self, progress: &Progress<Iter, Bounded<'_>, IndicatorMode>)
    where
        IndicatorMode: Indicator,
    {
        let (l, r) = self.brackets;
        println!(
            "{}{}{}{}{}",
            CLEAR,
            l,
            progress.indicator.as_str(progress.i),
            " ".repeat(progress.indicator.curr().len() * (self.bound - progress.i)),
            r
        )
    }
}

impl ProgressDisplay for Unbounded {
    fn display<Iter, IndicatorMode>(&self, progress: &Progress<Iter, Unbounded, IndicatorMode>)
    where
        IndicatorMode: Indicator,
    {
        println!("{}{}", CLEAR, progress.indicator.as_str(progress.i),)
    }
}

impl<Iter, IndicatorMode> Progress<Iter, Unbounded, IndicatorMode>
where
    Iter: ExactSizeIterator,
{
    fn with_bound(self) -> Progress<Iter, Bounded<'static>, IndicatorMode> {
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

impl<'a, Iter, IndicatorMode> Progress<Iter, Bounded<'a>, IndicatorMode>
where
    Iter: ExactSizeIterator,
{
    fn with_brackets(mut self, brackets: (&'a str, &'a str)) -> Self {
        self.bound.brackets = brackets;
        self
    }
}

impl<Iter, Bound, IndicatorMode> Iterator for Progress<Iter, Bound, IndicatorMode>
where
    Iter: Iterator,
    Bound: ProgressDisplay,
    IndicatorMode: Indicator,
{
    type Item = Iter::Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.bound.display(self);
        self.indicator.advance();
        self.i += 1;
        self.iter.next()
    }
}

fn expensive_computation(_i: &usize) {
    sleep(Duration::from_millis(250));
}

trait ProgressIteratorExt: Sized {
    fn progress(self) -> Progress<Self, Unbounded, SimpleIndicator<'static>>;
    fn progress_with_indicator<Ind: Indicator>(
        self,
        indicator: Ind,
    ) -> Progress<Self, Unbounded, Ind>;
}

impl<Iter: Iterator> ProgressIteratorExt for Iter {
    fn progress(self) -> Progress<Iter, Unbounded, SimpleIndicator<'static>> {
        Progress::new(self, SimpleIndicator { string: "*" })
    }

    fn progress_with_indicator<Ind: Indicator>(
        self,
        indicator: Ind,
    ) -> Progress<Iter, Unbounded, Ind> {
        Progress::new(self, indicator)
    }
}

fn main() {
    let count = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let indicator = RotatingIndicator {
        i: 0,
        strings: vec![",", "~", "`"],
    };

    for i in count
        .iter()
        // .progress()
        .progress_with_indicator(indicator)
        .with_bound()
        .with_brackets(("<|", "|>"))
    {
        expensive_computation(i)
    }
}
