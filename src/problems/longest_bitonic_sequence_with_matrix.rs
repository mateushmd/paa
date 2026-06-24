use crate::{ input_vec };

pub fn solve() {
    let v = input_vec!(isize);

    let mut lis = vec![vec![0usize; v.len() + 1]; v.len() + 1];
    let mut lds = vec![vec![0usize; v.len() + 1]; v.len() + 1];

    let v_ord = {
        let mut vv = v.clone();
        vv.sort();
        vv
    };

    for i in 0..v.len() {
        for j in 0..v.len() {
            lis[i + 1][j + 1] = if v[i] == v_ord[j] {
                lis[i][j] + 1
            } else {
                std::cmp::max(lis[i][j + 1], lis[i + 1][j])
            }
        }
    }

    for i in (0..v.len()).rev() {
        for j in (0..v.len()).rev() {
            lds[i][j] = if v[i] == v_ord[j] {
                lds[i + 1][j + 1] + 1
            } else {
                std::cmp::max(lds[i][j + 1], lis[i + 1][j])
            }
        }
    }

    let max_lis: Vec<_> = lis.iter().map(|row| row.iter().max()).collect();
    let max_lds: Vec<_> = lds.iter().map(|row| row.iter().max()).collect();

    let bitonic: Vec<_> = max_lis
        .iter()
        .enumerate()
        .map(|(idx, &val)| val.unwrap() + max_lds[idx].unwrap())
        .collect();

    print!("{:?}", bitonic);
}
