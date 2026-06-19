use crate::{ input, halt, common::PrettyVec };

pub fn solve() {
    let bytes1: Vec<u8> = input!("string 1: ")
        .trim()
        .bytes()
        .filter(|&b| matches!(b, b'A'..=b'Z' | b'a'..=b'z'))
        .collect();

    let bytes2: Vec<u8> = input!("string 2: ")
        .trim()
        .bytes()
        .filter(|&b| matches!(b, b'A'..=b'Z' | b'a'..=b'z'))
        .collect();
        
    let mut mat = vec![vec![0; bytes2.len() + 1]; bytes1.len() + 1]; 

    for i in 1..=bytes1.len() {
        for j in 1..=bytes2.len() {
            let max = std::cmp::max(mat[i - 1][j], mat[i][j - 1]);
            mat[i][j] = max + if bytes1[i - 1] == bytes2[j - 1] {
                1 
            } else {
                0
            };

            println!("{:?}", PrettyVec::from_2d(mat.clone()));
            halt!();
        }
    }
}
