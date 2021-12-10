use anyhow::Context;

const ILLEGAL_CHAR_SCORE: [(char, i32); 4] = [(')', 3), (']', 57), ('}', 1197), ('>', 25137)];
fn corrupted_score(c: char) -> anyhow::Result<i32> {
    ILLEGAL_CHAR_SCORE
        .iter()
        .find(|(ch, _)| *ch == c)
        .map(|(_, score)| *score)
        .context("Unknown character to score")
}

const MISSING_CHAR_SCORE: [(char, i64); 4] = [(')', 1), (']', 2), ('}', 3), ('>', 4)];
fn incomplete_score(v: &[char]) -> anyhow::Result<i64> {
    let values = v
        .iter()
        .map(|c| {
            MISSING_CHAR_SCORE
                .iter()
                .find(|(ch, _)| ch == c)
                .map(|(_, v)| *v)
        })
        .collect::<Option<Vec<_>>>()
        .context("Unknown character")?;
    Ok(values.into_iter().fold(0, |acc, v| acc * 5 + v))
}

#[derive(Debug)]
enum LineType {
    Corrupted(char),
    Incomplete(Vec<char>),
}

fn classify_line(s: &str) -> LineType {
    let mut expected = vec![];
    for c in s.chars() {
        match c {
            '(' => expected.push(')'),
            '[' => expected.push(']'),
            '{' => expected.push('}'),
            '<' => expected.push('>'),
            _ => {
                if let Some(v) = expected.pop() {
                    if v != c {
                        return LineType::Corrupted(c);
                    }
                }
            }
        }
    }
    expected.reverse();
    LineType::Incomplete(expected)
}

fn main() {
    let lines = aoc::get_input(21, 10)
        .trim()
        .split('\n')
        .map(str::to_owned)
        .collect::<Vec<_>>();

    let lines = lines.iter().map(|s| classify_line(&s)).collect::<Vec<_>>();
    let corrupted_scores = lines
        .iter()
        .filter_map(|s| {
            if let LineType::Corrupted(c) = s {
                Some(corrupted_score(*c))
            } else {
                None
            }
        })
        .collect::<anyhow::Result<Vec<_>>>()
        .expect("Illegal character in input");
    println!("Part 1: {}", corrupted_scores.into_iter().sum::<i32>());

    let mut incomplete_scores = lines
        .into_iter()
        .filter_map(|s| {
            if let LineType::Incomplete(inc) = s {
                Some(incomplete_score(&inc))
            } else {
                None
            }
        })
        .collect::<anyhow::Result<Vec<_>>>()
        .expect("Illegal character in input");
    incomplete_scores.sort();
    println!(
        "Part 2: {}",
        incomplete_scores[(incomplete_scores.len() / 2)]
    );
}
