use crate::input_vec;

pub fn solve() {
    let expr = input_vec!(f64);

    let mut e: Vec<Vec<f64>> = vec![vec![0f64; expr.len()]; expr.len()];

    for i in 0..expr.len() {
        e[i][i] = expr[i];
    }

    print!("{e:?}");
}
