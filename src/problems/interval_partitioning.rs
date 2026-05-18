use std::usize;

use crate::{
    input,
    common::Interval
};

pub fn solve() {
    let n = input!("number of intervals: ", usize);

    let mut r = vec![Interval::default(); n];

    for i in 0..n {
        r[i].s = input!(format!("start time of {i}: "), isize);
        r[i].f = input!(format!("finish time of {i}: "), isize);
    }

    r.sort_by_key(|i| i.s);

    let mut labels = vec![usize::MIN; n];

    for i in 0..n {
        let mut l: usize = 0;

        for j in 0..i {
            if r[i].s <= r[j].f {
                l += 1;
            }
        }

        labels[i] = l;
    }

    print!("{:?}", labels);
}
