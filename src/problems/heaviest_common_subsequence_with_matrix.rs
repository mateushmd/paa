use crate::{ input_vec, halt, common::PrettyVec };

pub fn solve() {
    let x = input_vec!(isize);
    let y = input_vec!(isize);

    let mut mat = vec![vec![0; y.len() + 1]; x.len() + 1];

    for i in 1..=x.len() {
        for j in 1..=y.len() {
            mat[i][j] = std::cmp::max(
                std::cmp::max(mat[i - 1][j], mat[i][j - 1]),
                if x[i - 1] == y[j - 1] {
                    mat[i - 1][j - 1] + x[i - 1]
                } else {
                    isize::MIN
                }
            );

            println!("{:?}", PrettyVec::from_2d(mat.clone()));
            halt!();
        }
    }
}
