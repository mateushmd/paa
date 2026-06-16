use crate::{ input, halt, common::PrettyVec };

pub fn solve() {
    let bytes1 = input!("text 1:").as_bytes().to_owned();
    let bytes2 = input!("text 2:").as_bytes().to_owned();

    assert!(bytes1.len() == bytes2.len());

    let mut mat = vec![vec![0; bytes1.len()]; bytes2.len()];

    for i in 0..bytes1.len() {
        mat[i][0] = i;
        mat[0][i] = i;
    }

    for i in 1..bytes2.len() {
        for j in 1..bytes1.len() {
            mat[i][j] = if bytes1[j - 1] == bytes2[i - 1] { 
                mat[i - 1][j - 1]
            } else { 
                mat[i][j - 1] + 1
            };

            print!("{:?}", PrettyVec::from_2d(mat.clone()));
            println!();
            halt!();
        }
    }
}
