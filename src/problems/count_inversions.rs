use crate::input_vec;

pub fn solve() {
    let mut v = input_vec!(isize);
    let size = v.len();

    print!("{}", sort(&mut v, 0, size - 1));
}

fn sort(v: &mut Vec<isize>, l: usize, r: usize) -> usize {
    if l >= r {
        return 0
    }

    let mid = (l + r) / 2;
    sort(v, l, mid) + sort(v, mid + 1, r) + merge(v, l, mid, r)
}

fn merge(v: &mut Vec<isize>, l: usize, mid: usize, r: usize) -> usize {
    let size = r - l + 1;
    let mut i_l = l;
    let mut i_r = mid + 1;

    let mut aux_v = vec![0isize; size]; 
    let mut invs = 0usize;

    for i in 0..size {
        if i_l > mid {
            aux_v[i] = v[i_r];
            i_r += 1;
        } else if i_r > r || v[i_l] <= v[i_r] {
            aux_v[i] = v[i_l];
            i_l += 1;
        } else {
            aux_v[i] = v[i_r];
            i_r += 1;
            invs += mid - i_l + 1;
        }
    }

    for i in 0..size {
        v[l + i] = aux_v[i];
    }

    invs
}
