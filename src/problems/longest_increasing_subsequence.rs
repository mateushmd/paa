use crate::input_vec;

pub fn solve() {
    let vec = input_vec!(isize);
    let mut subsets = vec![1; vec.len()];

    for i in 0..vec.len() {
        for j in 0..i {
            if vec[j] < vec[i] {
                subsets[i] = std::cmp::max(subsets[i], subsets[j] + 1);
            }
        }
    }

    print!("{subsets:?}");
}
