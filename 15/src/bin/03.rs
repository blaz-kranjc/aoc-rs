use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Point(i32, i32);

fn visit(instructions: impl Iterator<Item = char>) -> HashSet<Point> {
    let mut visited = HashSet::new();
    let mut current = Point(0, 0);
    visited.insert(current);
    for dir in instructions {
        match dir {
            '^' => current = Point(current.0, current.1 + 1),
            '>' => current = Point(current.0 + 1, current.1),
            '<' => current = Point(current.0 - 1, current.1),
            'v' => current = Point(current.0, current.1 - 1),
            _ => (),
        };
        visited.insert(current);
    }
    visited
}

fn main() {
    let instructions = aoc::get_input(15, 3);
    println!("Part 1: {}", visit(instructions.chars()).len());
    let santa = visit(instructions.chars().step_by(2));
    let robot = visit(instructions.chars().skip(1).step_by(2));
    println!("Part 2: {}", santa.union(&robot).count());
}
