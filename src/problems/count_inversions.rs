use crate::input_vec;

pub fn solve() {
    let mut v = input_vec!(isize);
    let len = v.len();

    print!("{}", sort(&mut v, 0, len - 1, 0)); 
}

pub fn sort(v: &mut Vec<isize>, l: usize, r: usize, mut c: usize) -> usize {
    if l < r {
        let mid = (l + r + 1) / 2;
        c = sort(v, l, mid, c) + sort(v, mid + 1, r, c);
        c += merge(v, l, mid, r, c);
    }

    c
}

pub fn merge(v: &mut Vec<isize>, l: usize, mid: usize, r: usize, mut c: usize) -> usize {
    let size = r - l + 1;
    let mut sub_vec = vec![0; size]; 
    let mut ptr_l = l;
    let mut ptr_r = mid + 1;

    for i in 0..size {
        sub_vec[i] = if ptr_r <= r && v[ptr_r] < v[ptr_l] {
            ptr_r += 1;
            c += 1; 
            v[ptr_r]
        } else {
            ptr_l += 1;
            v[ptr_l]
        };
    }

    for i in 0..size {
        v[i + ptr_l] = sub_vec[i];
    }

    c
}
