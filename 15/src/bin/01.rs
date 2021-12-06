fn main() {
    let levels = aoc::get_input(15, 1).chars().fold(vec![0], |mut acc, c| {
        match c {
            '(' => acc.push(acc.last().unwrap() + 1),
            ')' => acc.push(acc.last().unwrap() - 1),
            _ => panic!("Invalid input"),
        }
        acc
    });
    println!("Part 1: {}", levels.last().unwrap());
    println!(
        "Part 2: {}",
        levels.iter().enumerate().find(|(_, &e)| e < 0).unwrap().0
    );
}
