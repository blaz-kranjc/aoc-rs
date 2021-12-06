fn count_increasing(heights: &[i32], step: usize) -> usize {
    heights.windows(step + 1).filter(|x| x[0] < x[step]).count()
}

fn main() {
    let heights: Vec<i32> = aoc::get_input(21, 1)
        .split('\n')
        .map(str::parse)
        .filter_map(Result::ok)
        .collect();

    println!("Part 1: {}", count_increasing(&heights, 1));
    println!("Part 2: {}", count_increasing(&heights, 3));
}
