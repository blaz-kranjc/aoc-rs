use std::collections::HashMap;

fn propagate(
    values: &HashMap<[char; 2], i64>,
    rules: &HashMap<[char; 2], char>,
) -> HashMap<[char; 2], i64> {
    let mut result = HashMap::new();
    for (k, v) in values {
        if let Some(c) = rules.get(k) {
            let (first, second) = ([k[0], *c], [*c, k[1]]);
            result.insert(first, v + result.get(&first).unwrap_or(&0));
            result.insert(second, v + result.get(&second).unwrap_or(&0));
        } else {
            result.insert(*k, *v);
        }
    }
    result
}

fn expand(template: &str) -> HashMap<[char; 2], i64> {
    template
        .as_bytes()
        .windows(2)
        .map(|v| [v[0] as char, v[1] as char])
        .fold(HashMap::new(), |mut acc, n| {
            acc.insert(n, acc.get(&n).unwrap_or(&0) + 1);
            acc
        })
}

fn count_chars(template: &str, values: &HashMap<[char; 2], i64>) -> Vec<(char, i64)> {
    let mut result = HashMap::new();
    for (k, v) in values {
        result.insert(k[0], result.get(&k[0]).unwrap_or(&0) + v);
    }
    let last = template.chars().nth_back(0).unwrap();
    result.insert(last, result.get(&last).unwrap_or(&0) + 1);
    result.into_iter().collect()
}

fn min_max_diff(template: &str, values: &HashMap<[char; 2], i64>) -> i64 {
    let mut chars = count_chars(template, values);
    chars.sort_by(|(_, l), (_, r)| l.cmp(r));
    chars.last().unwrap().1 - chars.first().unwrap().1
}

fn main() {
    let input = aoc::get_input(21, 14)
        .trim()
        .split('\n')
        .map(str::to_owned)
        .collect::<Vec<_>>();
    let template = input[0].clone();
    let rules = input[2..]
        .iter()
        .map(|s| {
            let mut parts = s.split(" -> ");
            let pattern = parts.next().unwrap();
            let result = parts.next().unwrap().chars().nth(0).unwrap();
            (
                [
                    pattern.chars().nth(0).unwrap(),
                    pattern.chars().nth(1).unwrap(),
                ],
                result,
            )
        })
        .collect::<HashMap<_, _>>();

    let part1 = (0..10).fold(expand(&template), |acc, _| propagate(&acc, &rules));
    println!("Part 1: {:?}", min_max_diff(&template, &part1));

    let part2 = (10..40).fold(part1, |acc, _| propagate(&acc, &rules));
    println!("Part 2: {:?}", min_max_diff(&template, &part2));
}
