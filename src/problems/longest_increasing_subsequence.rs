use crate::input;

pub fn solve() {
    let n = input!("list size: ", usize);

    let mut vec = vec![0; n];
    let mut subsets = vec![1; n];

    for i in 0..n {
        vec[i] = input!(format!("value {i}: "), usize);
    }

    for i in 0..n {
        for j in 0..i {
            if vec[j] <= vec[i] {
                subsets[i] = std::cmp::max(subsets[i], subsets[j] + 1);
            }
        }
    }

    print!("{subsets:?}");
}
