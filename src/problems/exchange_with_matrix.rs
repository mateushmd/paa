use crate::input;

pub fn solve() {
    let n = input!("number of coins: ", usize);

    let mut c = vec![0; n];

    for i in 0..n {
        c[i] = input!(format!("coin {i}: "), usize);
    }

    let s = input!("S: ", usize);
    
    let mut mat = vec![vec![0; s + 1]; n + 1];

    mat[0].fill(usize::MAX);

    for i in 1..n {
        for j in 1..=s {
            mat[i][j] = std::cmp::min(
                match mat[i].get(j.wrapping_sub(c[i])) {
                    Some(x) => x.wrapping_add(1),
                    None => usize::MAX
                },
                mat[i - 1][j]
            );
        }
    }

    print!("{mat:?}");
}
