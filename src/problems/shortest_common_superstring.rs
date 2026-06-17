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

    println!("{b1:?}");
    println!("{b2:?}");

    let mut mat = vec![vec![0usize; b2.len()]; b1.len()];

    for i in 0..b1.len() {
        for j in 0..b2.len() {
            if b1[i] == b2[j] {
                mat[i][j] = 1 + {
                    if i > 0 && j > 0 { mat[i - 1][j - 1] } else { 0 }
                };
            }

            print!("{:?}", PrettyVec::from_2d(mat.clone()));
            println!();
            halt!();
        }
    }

    let mut max_valid_overlap = 0usize;

    for i in 0..b1.len() {
        let overlap = mat[i][b2.len() - 1];
        if overlap != 0 && (overlap - 1 == i || overlap == b2.len()) {
            max_valid_overlap = if overlap > max_valid_overlap {
                overlap
            } else {
                max_valid_overlap
            }
        }
    }

    for i in 0..b2.len() {
        let overlap = mat[b1.len() - 1][i];
        if overlap != 0 && (overlap - 1 == i || overlap == b1.len()) {
            max_valid_overlap = if overlap > max_valid_overlap {
                overlap
            } else {
                max_valid_overlap
            }
        }
    }

    print!("{max_valid_overlap}");
}
