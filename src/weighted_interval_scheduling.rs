use crate::input;

struct Interval {
    start: isize,
    end: isize
}

impl Default for Interval {
    fn default() -> Self {
        Interval {
            start: 0,
            end: 0
        }
    }
}

impl Clone for Interval {
    fn clone(&self) -> Self {
        Interval {
            start: self.start,
            end: self.end
        }
    }
}

pub fn solve() {
    let n = input!("number of intervals: ").trim().parse::<usize>().expect("expected number");
    
    let mut s = vec![Interval::default(); n];

    for i in 0..n {
        s[i].start = input!(format!("start value of {i}: ")).trim().parse::<isize>().expect("expected number");
        s[i].start = input!(format!("end value of {i}: ")).trim().parse::<isize>().expect("expected number");
    }
}
