use anyhow::{bail, Result};

mod bingo {
    #[derive(Debug)]
    pub struct BingoBoard([[i8; 5]; 5], [[bool; 5]; 5]);

    impl BingoBoard {
        pub fn new(numbers: [[i8; 5]; 5]) -> BingoBoard {
            BingoBoard(numbers, [[false; 5]; 5])
        }

        fn all_checked_row(&self, row: usize) -> bool {
            for column in 0usize..5 {
                if !self.1[row][column] {
                    return false;
                }
            }
            true
        }

        fn all_checked_column(&self, column: usize) -> bool {
            for row in 0usize..5 {
                if !self.1[row][column] {
                    return false;
                }
            }
            true
        }

        pub fn mark_and_check(&mut self, number: i8) -> bool {
            for row in 0usize..5 {
                for column in 0usize..5 {
                    if self.0[row][column] == number {
                        self.1[row][column] = true;
                        return self.all_checked_row(row) || self.all_checked_column(column);
                    }
                }
            }
            false
        }

        pub fn unchecked(&self) -> Vec<i8> {
            let mut result = vec![];
            for row in 0usize..5 {
                for column in 0usize..5 {
                    if !self.1[row][column] {
                        result.push(self.0[row][column]);
                    }
                }
            }
            result
        }
    }
}

use bingo::BingoBoard;

impl TryFrom<&[String]> for BingoBoard {
    type Error = anyhow::Error;
    fn try_from(value: &[String]) -> Result<Self, Self::Error> {
        if value.len() != 5 {
            bail!("wrong size of input array");
        }
        let mut bingo = [[0i8; 5]; 5];
        for (row, a) in value.iter().enumerate() {
            let nums = a
                .split(' ')
                .filter_map(|a| a.parse::<i8>().ok())
                .collect::<Vec<_>>();
            if nums.len() != 5 {
                bail!("Wrong size of line");
            }
            for (col, &v) in nums.iter().enumerate() {
                bingo[row][col] = v;
            }
        }
        Ok(BingoBoard::new(bingo))
    }
}

fn bingo_on(board: &mut BingoBoard, draws: &[i8]) -> Result<(usize, i32)> {
    for (n, &v) in draws.iter().enumerate() {
        if board.mark_and_check(v) {
            return Ok((n, score(board, v)));
        }
    }
    bail!("the board never wins")
}

fn score(board: &BingoBoard, last_drawn: i8) -> i32 {
    board.unchecked().into_iter().map(i32::from).sum::<i32>() * (last_drawn as i32)
}

fn main() {
    let input = aoc::get_input(21, 4)
        .split('\n')
        .map(str::to_owned)
        .collect::<Vec<_>>();
    let draws = input[0]
        .split(',')
        .filter_map(|s| s.parse::<i8>().ok())
        .collect::<Vec<_>>();
    let mut bingos = vec![];
    let mut begin_id = 2usize;
    while begin_id < input.len() {
        if let Ok(bingo) = BingoBoard::try_from(&input[begin_id..begin_id + 5]) {
            bingos.push(bingo);
        }
        begin_id += 6;
    }

    let mut bingos = bingos
        .iter_mut()
        .filter_map(|b| bingo_on(b, &draws).ok())
        .collect::<Vec<_>>();
    bingos.sort_by(|(on_left, _), (on_right, _)| on_left.cmp(on_right));

    println!("Part 1: {}", bingos[0].1);
    println!("Part 2: {}", bingos[bingos.len() - 1].1);
}
