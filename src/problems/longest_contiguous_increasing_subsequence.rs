use crate::input_vec;

pub fn solve() {
    let vec = input_vec!(isize);

    let mut s = 1;
    let mut s1 = 1;

    for i in 1..vec.len() {
        if vec[i] > vec[i - 1] {
            s1 = s1 + 1;
            s = s.max(s1);
        } else {
            s1 = 1;
        }
    }

    print!("{s}");
}
