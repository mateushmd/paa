use crate::input;

use crate::common::Interval;

pub fn solve() {
    let n: usize = input!("Number of intervals: ", usize);
    
    let mut r = vec![Interval::default(); n]; 

    for i in 0..n {
        r[i].s = input!(format!("start time of {}: ", i), isize);
        r[i].f = input!(format!("finish time of {}: ", i), isize);
    }

    let mut a: Vec<&Interval> = Vec::new();
    
    let mut i: usize = 0;

    while i < n {
        a.push(&r[i]);

        let mut tmp = i + 1;

        while tmp < n && &r[tmp].s < &r[i].f {
            tmp += 1;
        }

        i = tmp;
    }

    print!("{:?}", a);
}
