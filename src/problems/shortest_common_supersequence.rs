use crate::{ input, halt, common::PrettyVec };

pub fn solve() {
    let b1: Vec<u8> = input!("string 1: ")
        .trim()
        .bytes()
        .filter(|&b| matches!(b, b'A'..=b'Z' | b'a'..=b'z'))
        .collect();

    let b2: Vec<u8> = input!("string 2: ")
        .trim()
        .bytes()
        .filter(|&b| matches!(b, b'A'..=b'Z' | b'a'..=b'z'))
        .collect();

    let mut mat = vec![vec![0usize; b2.len()]; b1.len()];

    for i in 0..b1.len() {
        for j in 0..b2.len() {
            mat[i][j] = if b1[i] == b2[j] {
                if i > 0 && j > 0 { mat[i - 1][j - 1] + 1 } else { 1 }
            } else {
                std::cmp::max(
                    if i > 0 { mat[i - 1][j] } else { 0 },
                    if j > 0 { mat[i][j - 1] } else { 0 }
                )
            };

            print!("{:?}", PrettyVec::from_2d(mat.clone()));
            println!();
            halt!();
        }
    }

    print!("{}", b1.len() + b2.len() - mat.last().unwrap().last().unwrap());
}
