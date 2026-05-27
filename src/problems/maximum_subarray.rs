use crate::input_vec;

pub fn solve() {
    let v = input_vec!(isize);

    let mut max = v[0];
    let mut prev = v[0];

    for i in 1..v.len() {
        prev = v[i].max(v[i] + prev);
        max = max.max(prev);
    }

    print!("{max}");
}
