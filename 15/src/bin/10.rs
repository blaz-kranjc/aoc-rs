// TODO is there any way to not brute force this?

// TODO this is a very implicit way of calculating the result, try to rewrite this more functionally?
fn next(seq: &[i8]) -> Vec<i8> {
    let mut result = vec![];
    let mut iter = seq.iter();
    let mut count = 1;
    let mut curr = iter.next();
    loop {
        if curr.is_none() {
            break;
        }
        let next = iter.next();
        if curr.eq(&next) {
            count += 1;
            continue;
        } else {
            result.push(count as i8);
            result.push(*curr.unwrap() as i8);
            curr = next;
            count = 1;
        }
    }
    result
}

fn advance(seq: &[i8], n: u32) -> Vec<i8> {
    (0..n).fold(seq.to_owned(), |acc, _| next(&acc))
}

fn main() {
    let sequence = aoc::get_input(15, 10)
        .trim()
        .chars()
        .map(|c| c.to_digit(10).map(|d| d as i8))
        .collect::<Option<Vec<_>>>()
        .expect("Invalid input");

    println!("Part 1: {}", advance(&sequence, 40).len());
    println!("Part 2: {}", advance(&sequence, 50).len());
}
