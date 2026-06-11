use crate::{ input, input_vec, halt, common::PrettyVec };

pub fn solve() {
    let c = input_vec!(usize);

    let s = input!("S: ", usize);
    
    let mut mat = vec![vec![usize::MAX; s + 1]; c.len() + 1];

    for i in 0..=c.len() {
        mat[i][0] = 0;
    }

    print!("{:?}", PrettyVec::from_2d(mat.clone()));
    println!();
    halt!();

    for i in 1..=c.len() {
        for j in 1..=s {
            mat[i][j] = std::cmp::min(
                match mat[i].get(j.wrapping_sub(c[i - 1])) {
                    Some(x) => x.saturating_add(1),
                    None => usize::MAX
                },
                if i > 1 { mat[i - 1][j] } else { usize::MAX }
            );

            print!("{:?}", PrettyVec::from_2d(mat.clone()));
            println!();
            halt!();
        }
    }
}
