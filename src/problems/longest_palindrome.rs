use crate::input;

pub fn solve() {
    let bytes = input!("string: ");
    let bbytes = bytes.as_bytes();

    for i in 1..(bbytes.len() - 1) {
        let mut k = 1;

        while i + k < bbytes.len() && i - k >= 0 && bbytes[i + k] == bbytes[i - k] {
            k = k + 1
        }

        while bbytes[i + k - 1] == bbytes[i + k] {
        }
    }
}
