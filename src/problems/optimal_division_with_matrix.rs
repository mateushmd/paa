use crate::{ input_vec, halt, common::PrettyVec };

pub fn solve() {
    let v = input_vec!(f32);

    let mut m1 = vec![vec![0f32; v.len()]; v.len()];
    let mut m2 = vec![vec![0f32; v.len()]; v.len()];

    for i in 0..v.len() {
        m1[i][i] = v[i];
        m2[i][i] = v[i];
    }

    println!("m1");
    print!("{:?}", PrettyVec::from_2d(m1.clone()));
    println!();
    println!("m2");
    print!("{:?}", PrettyVec::from_2d(m2.clone()));
    println!();
    halt!();

    for i in 0..(v.len() - 1) {
        for j in (1 + i)..v.len() {
            e(&mut m1, &m2, i, j);
            e_l(&m1, &mut m2, i, j);

            println!("m1");
            print!("{:?}", PrettyVec::from_2d(m1.clone()));
            println!();
            println!("m2");
            print!("{:?}", PrettyVec::from_2d(m2.clone()));
            println!();
            halt!();
        }
    }
}

fn e(m1: &mut Vec<Vec<f32>>, m2: &Vec<Vec<f32>>, i: usize, j: usize) {
    let mut max_div = f32::MIN;
    for k in i..j {
        let div = m1[i][j - k - 1] / m2[k + 1][j];
        max_div = if max_div < div { div } else { max_div }
    }
    m1[i][j] = max_div;
}

fn e_l(m1: &Vec<Vec<f32>>, m2: &mut Vec<Vec<f32>>, i: usize, j: usize) {
    let mut min_div = f32::MAX;
    for k in i..j {
        let div = m2[i][j - k - 1] / m1[k + 1][j];
        min_div = if min_div > div { div } else { min_div }
    }
    m2[i][j] = min_div;
}
