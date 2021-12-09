// TODO is there any way to not brute force this?

// TODO this is allocating like crazy, maybe vectors would be a better use as string in rust are quite naughty :)
// TODO this is a very implicit way of calculating the result, try to rewrite this more functionally?
fn next(seq: &str) -> String {
    let mut result = "".to_owned();
    let mut iter = seq.chars();
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
            result.push_str(char::from_digit(count, 10).unwrap().to_string().as_ref());
            result.push_str(curr.unwrap().to_string().as_ref());
            curr = next;
            count = 1;
        }
    }
    result
}

fn advance(seq: &str, n: u32) -> String {
    (0..n).fold(seq.to_owned(), |acc, _| next(&acc))
}

fn main() {
    let sequence = aoc::get_input(15, 10).trim().to_owned();

    println!("Part 1: {}", advance(&sequence, 40).len());
    println!("Part 2: {}", advance(&sequence, 50).len());
}
