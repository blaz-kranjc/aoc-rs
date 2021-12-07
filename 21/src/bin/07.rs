fn sum_ints_until(n: i32) -> i32 {
    (n * (n + 1)) / 2
}

fn main() {
    let crab_positions = aoc::get_input(21, 7)
        .trim()
        .split(',')
        .filter_map(|s| s.parse::<i32>().ok())
        .collect::<Vec<_>>();
    let linear = crab_positions
        .iter()
        .map(|p| crab_positions.iter().map(|x| (x - p).abs()).sum::<i32>())
        .min()
        .unwrap();
    println!("Part 1: {}", linear);

    let quadratic = crab_positions
        .iter()
        .map(|p| {
            crab_positions
                .iter()
                .map(|x| sum_ints_until((x - p).abs()))
                .sum::<i32>()
        })
        .min()
        .unwrap();
    println!("Part 2: {}", quadratic);
}
