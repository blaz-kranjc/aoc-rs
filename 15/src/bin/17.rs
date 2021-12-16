fn count_arrangements(parts: &[i32], value: i32, n_containers: i32) -> i32 {
    if value == 0 && n_containers == 0 {
        1
    } else if parts.is_empty() || value < 0 || n_containers <= 0 {
        0
    } else {
        count_arrangements(&parts[1..], value - parts[0], n_containers - 1)
            + count_arrangements(&parts[1..], value, n_containers)
    }
}

fn main() {
    let containers = aoc::get_input(15, 17)
        .trim()
        .split('\n')
        .map(|s| s.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()
        .expect("Invalid input");

    let mut min_arrangements = None;
    let mut all_arrangements = 0;
    for i in 1..=containers.len() {
        let count = count_arrangements(&containers, 150, i as i32);
        all_arrangements += count;
        if min_arrangements.is_none() && count > 0 {
            min_arrangements = Some(count);
        }
    }

    println!("Part 1: {}", all_arrangements);
    println!("Part 2: {}", min_arrangements.unwrap_or(0));
}
