use crate::input;

pub fn solve() {
    let text = input!("string: ");
    let bytes = text.as_bytes();

    for i in 1..(bytes.len() - 1) {
        let mut k = 1;

        while i + k < bytes.len() && i - k >= 0 && bytes[i + k] == bytes[i - k] {
            k = k + 1
        }

        while bytes[i + k - 1] == bytes[i + k] {
        }
    }
}
