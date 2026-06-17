use crate::input;

pub fn solve() {
    let f1 = input!("factor 1: ", usize);
    let f2 = input!("factor 2: ", usize);

    let (result, mults) = mult(f1, f2);
    print!("{} with {} multiplications", result, mults);
}

// TODO: the multiplication counting is probably wrong
fn mult(f1: usize, f2: usize) -> (usize, usize) {
    let max = std::cmp::max(f1, f2);
    let size = if max == 0 { 1 } else { max.ilog2() + 1 };

    if size == 1 {
        return (f1 & f2, 1)
    }

    let f1_1 = f1 >> (size / 2);
    let f1_2 = f1 & ((1 << (size / 2)) - 1);
    let f2_1 = f2 >> (size / 2);
    let f2_2 = f2 & ((1 << (size / 2)) - 1);

    let mut mults = 0usize;
    let (t1, m1) = mult(f1_1, f2_1);
    let (t2, m2) = mult(f1_2, f2_2);
    let (t3, m3) = mult(f1_1 + f1_2, f2_1 + f2_2);
    mults += m1 + m2 + m3;

    ((t1 << (size / 2) * 2) + ((t3 - t1 - t2) << (size / 2)) + t2, mults)
}

