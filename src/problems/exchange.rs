use crate::input;

pub fn solve() {
    let s = input!("target value: ").trim().parse::<usize>().expect("expected number");

    let coin_count: usize = input!("number of coins: ")
        .trim().parse::<usize>().expect("expected number");

    let mut coins: Vec<usize> = vec![0; coin_count];

    for i in 0..coin_count {
        coins[i] = input!(format!("coin {}: ", i + 1))
            .trim().parse::<usize>().expect("expected number");
    }

    let mut exchange: Vec<u32> = vec![u32::MAX; s + 1];
    exchange[0] = 0;

    for i in 1..=s {
        for coin in coins.iter() {
            if *coin <= i {
                exchange[i] = std::cmp::min(exchange[i], exchange[i - coin] + 1); 
            }
        }
    }

    print!("{:?}", exchange);
}
