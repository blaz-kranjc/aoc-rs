use std::str::FromStr;

struct Box(i64, i64, i64);

impl Box {
    fn wrapping_area(&self) -> i64 {
        let mut faces = [self.0 * self.1, self.1 * self.2, self.2 * self.0];
        faces.sort();
        faces[0] * 3 + faces[1] * 2 + faces[2] * 2
    }

    fn volume(&self) -> i64 {
        self.0 * self.1 * self.2
    }

    fn ribbon_length(&self) -> i64 {
        let mut sides = [self.0, self.1, self.2];
        sides.sort();
        sides[0] * 2 + sides[1] * 2 + self.volume()
    }
}

impl FromStr for Box {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split("x")
            .map(|s| s.parse::<i64>())
            .collect::<Result<Vec<i64>, _>>()
            .map(|v| Box(v[0], v[1], v[2]))
            .map_err(|_| "Unable to parse Box".to_owned())
    }
}

fn main() {
    let boxes = aoc::get_input(15, 2)
        .lines()
        .map(|s| s.parse::<Box>().unwrap())
        .collect::<Vec<Box>>();
    println!(
        "Part 1: {}",
        boxes.iter().map(|b| b.wrapping_area()).sum::<i64>()
    );
    println!(
        "Part 2: {}",
        boxes.iter().map(|b| b.ribbon_length()).sum::<i64>()
    );
}
