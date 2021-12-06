fn n_jellyfish(v: i32, t: i32, cache: &mut [Option<usize>]) -> usize {
    if v >= t {
        1
    } else if v != 0 {
        n_jellyfish(0, t - v, cache)
    } else if let Some(result) = cache[t as usize] {
        result
    } else {
        let result = n_jellyfish(6, t - 1, cache) + n_jellyfish(8, t - 1, cache);
        cache[t as usize] = Some(result);
        result
    }
}

fn main() {
    let jellyfish = aoc::get_input(21, 6)
        .trim()
        .split(',')
        .filter_map(|c| c.parse::<i32>().ok())
        .collect::<Vec<_>>();

    let mut cache = [None; 265];
    println!(
        "Part 1: {}",
        jellyfish
            .iter()
            .map(|&j| n_jellyfish(j, 80, &mut cache))
            .sum::<usize>()
    );
    println!(
        "Part 2: {}",
        jellyfish
            .iter()
            .map(|&j| n_jellyfish(j, 256, &mut cache))
            .sum::<usize>()
    );
}
