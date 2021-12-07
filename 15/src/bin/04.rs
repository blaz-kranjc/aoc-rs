use hex::encode;
use md5::{Digest, Md5};

fn append_number(key: &str, n: i64) -> String {
    key.to_owned() + &n.to_string()
}

fn hash_of(s: &str) -> String {
    let mut hasher = Md5::new();
    hasher.update(s);
    encode(hasher.finalize())
}

fn starts_with_n_zeroes(s: &str, n: usize) -> bool {
    s.chars().take(n).all(|c| c == '0')
}

fn main() {
    let base = aoc::get_input(15, 4).trim().to_owned();
    println!(
        "Part 1: {}",
        (1i64..)
            .find(|&n| starts_with_n_zeroes(&hash_of(&append_number(&base, n)), 5))
            .unwrap()
    );
    println!(
        "Part 1: {}",
        (1i64..)
            .find(|&n| starts_with_n_zeroes(&hash_of(&append_number(&base, n)), 6))
            .unwrap()
    );
}
