use crate::input_vec;

pub fn solve() {
    let v = input_vec!(isize); 

    print!("{}", sum(&v, 0, v.len() - 1));
}

fn sum(v: &Vec<isize>, l: usize, r: usize) -> isize {
    if l == r {
        return v[l]
    }

    let m = (l + r) / 2;
    
    let ls = sum(v, l, m);
    let rs = sum(v, m + 1, r);
    let ms = sum_between(v, l, m, r);

    std::cmp::max(ms, std::cmp::max(ls, rs))
}

fn sum_between(v: &Vec<isize>, l: usize, m: usize, r: usize) -> isize {
    let mut sum_l = isize::MIN;
    let mut sum_r = isize::MIN;
    let mut sum = 0isize;

    for i in (l..=m).rev() {
        sum += v[i];
        if sum > sum_l { sum_l = sum; }
    }

    sum = 0;

    for i in (m + 1)..=r {
        sum += v[i];
        if sum > sum_r { sum_r = sum; }
    }

    sum_l + sum_r
}
