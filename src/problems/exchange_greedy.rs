use crate::input;

pub fn solve() {
    let mut s = input!("target value: ").trim().parse::<isize>().expect("expected number");

    let coin_count: usize = input!("number of coins: ").trim().parse::<usize>().expect("expected number");

    let mut coins: Vec<isize> = vec![0; coin_count];

    for i in 0..coin_count {
        coins[i] = input!(format!("coin {}: ", i + 1)).trim().parse::<isize>().expect("expected numebr");
    }

    coins.sort();
    
    let mut i: isize = (coins.len() - 1) as isize;
    let mut cont: usize = 0;

    while s > 0 && i >= 0 {
        if s < coins[i as usize] {
            i -= 1;
        } else {
            s -= coins[i as usize];
            cont += 1;
        }
    }

    if s == 0 {
        print!("{cont}");
    } else {
        print!("0");
    }
}
