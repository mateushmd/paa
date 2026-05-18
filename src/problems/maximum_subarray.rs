use crate::input_vec;

pub fn solve() {
    let v = input_vec!(isize);

    let mut max = 0isize;
    let mut prev = 0isize;

    for i in 0..v.len() {
        prev = v[i].max(v[i] + prev);
        max = max.max(prev);
    }

    print!("{max}");
}
