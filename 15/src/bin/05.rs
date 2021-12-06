const NAUGHTY_PARTS: [&str; 4] = ["ab", "cd", "pq", "xy"];
const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn has_naughty_parts(s: &str) -> bool {
    NAUGHTY_PARTS.iter().any(|&part| s.contains(part))
}

fn has_duplicated_letter(s: &str) -> bool {
    s.as_bytes().windows(2).any(|pair| pair[0] == pair[1])
}

fn has_3_vowels(s: &str) -> bool {
    s.chars().filter(|&c| VOWELS.contains(&c)).count() >= 3
}

fn is_nice_part1(s: &str) -> bool {
    !has_naughty_parts(s) && has_duplicated_letter(s) && has_3_vowels(s)
}

fn has_sandwiched_letter(s: &str) -> bool {
    s.as_bytes().windows(3).any(|pair| pair[0] == pair[2])
}

fn has_duplicated_pair(s: &str) -> bool {
    for i in 0..(s.len() - 3) {
        for j in (i + 2)..(s.len() - 1) {
            if s[i..=(i + 1)] == s[j..=(j + 1)] {
                return true;
            }
        }
    }
    false
}

fn is_nice_part2(s: &str) -> bool {
    has_sandwiched_letter(s) && has_duplicated_pair(s)
}

fn main() {
    let input = aoc::get_input(15, 5);
    let strings = input.lines().collect::<Vec<_>>();
    println!(
        "Part 1: {}",
        strings.iter().filter(|s| is_nice_part1(s)).count()
    );
    println!(
        "Part 2: {}",
        strings.iter().filter(|s| is_nice_part2(s)).count()
    );
}
