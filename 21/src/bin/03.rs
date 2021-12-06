use partition::partition;

fn add_ones(acc: Vec<i32>, v: &String) -> Vec<i32> {
    acc.into_iter()
        .zip(v.chars())
        .map(|(count, ch)| if ch == '1' { count + 1 } else { count })
        .collect()
}

fn power_consumption(ns: &Vec<String>) -> i32 {
    let bit_size = ns[0].len();
    let n_ones = ns.iter().fold(vec![0; bit_size], add_ones);
    let n_numbers = ns.len();
    let (gamma_rate, epsilon_rate) = n_ones.into_iter().fold((0, 0), |(gamma, epsilon), count| {
        if count as usize > n_numbers / 2 {
            ((gamma << 1) + 1, epsilon << 1)
        } else {
            (gamma << 1, (epsilon << 1) + 1)
        }
    });
    gamma_rate * epsilon_rate
}

#[derive(PartialEq, Eq)]
enum Criteria {
    LeastCommon,
    MostCommon,
}

fn extract_extreme(mut ns: &mut [String], criteria: Criteria) -> i32 {
    let mut bit = 1;
    while ns.len() > 1 {
        let (longer, shorter) = partition_numbers_by_bit(ns, bit);
        ns = if criteria == Criteria::MostCommon {
            longer
        } else {
            shorter
        };
        bit = bit + 1;
    }
    binary_to_i32(&ns[0])
}

fn binary_to_i32(s: &str) -> i32 {
    s.chars().fold(0, |acc, c| (acc << 1) + (c == '1') as i32)
}

fn partition_numbers_by_bit(ns: &mut [String], bit: usize) -> (&mut [String], &mut [String]) {
    let (zeroes, ones) = partition(ns, |s| s.chars().nth(bit).unwrap() == '0');
    if zeroes.len() > ones.len() {
        (zeroes, ones)
    } else {
        (ones, zeroes)
    }
}

fn life_support_rating(ns: &mut Vec<String>) -> i32 {
    let (oxygen_range, co2_range) = partition_numbers_by_bit(ns, 0);
    let oxygen = extract_extreme(oxygen_range, Criteria::MostCommon);
    let co2 = extract_extreme(co2_range, Criteria::LeastCommon);
    oxygen * co2
}

fn main() {
    let mut input = aoc::get_input(21, 3)
        .split('\n')
        .filter(|v| !v.is_empty())
        .map(str::to_owned)
        .collect::<Vec<_>>();

    println!("Part 1: {}", power_consumption(&input));
    println!("Part 2: {}", life_support_rating(&mut input));
}
