fn display_store_diff(s: &str) -> usize {
    let mut result = 0;
    let mut it = s.chars();
    loop {
        let current = it.next();
        if current.is_none() {
            break;
        }
        let current = current.unwrap();
        if current == '\"' {
            result += 1;
        } else if current == '\\' {
            let next = it.next().unwrap();
            if next == 'x' {
                result += 3;
                it.next();
                it.next();
            } else {
                result += 1;
            }
        }
    }
    result
}

fn display_escaped_diff(s: &str) -> usize {
    s.chars()
        .fold(2, |acc, c| acc + if c == '\\' || c == '\"' { 1 } else { 0 })
}

fn main() {
    let strings = aoc::get_input(15, 8)
        .trim()
        .split('\n')
        .map(str::to_owned)
        .collect::<Vec<_>>();

    println!(
        "Part 1: {}",
        strings.iter().fold(0, |acc, s| acc + display_store_diff(s))
    );
    println!(
        "Part 2: {}",
        strings
            .iter()
            .fold(0, |acc, s| acc + display_escaped_diff(s))
    );
}
