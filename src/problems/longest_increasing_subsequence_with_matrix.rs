use crate::{ input_vec, halt, common::PrettyVec };

pub fn solve() {
    let v = input_vec!(isize);

    let mut mat = vec![vec![0; v.len()]; v.len()];

    let mut ordered = v.clone();
    ordered.sort();
    let ordered = ordered;

    for i in 1..v.len() {
        for j in 0..v.len() {
            mat[i][j] = mat[i - 1][j] + {
                if v[i] <= ordered[j] { 1 } else { 0 }
            };

            print!("{:?}", PrettyVec::from_2d(mat.clone()));
            println!();
            halt!();
        }
    }
}
