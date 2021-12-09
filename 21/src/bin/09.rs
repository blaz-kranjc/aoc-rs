use std::str::FromStr;

use anyhow::{bail, Context};

#[derive(Debug)]
struct Grid {
    data: Vec<i8>,
    n_columns: usize,
}

impl Grid {
    fn index(&self, row: usize, column: usize) -> usize {
        row * self.n_columns + column
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn from_index(&self, n: usize) -> (usize, usize) {
        (n / self.n_columns, n % self.n_columns)
    }

    fn at(&self, row: usize, column: usize) -> i8 {
        self.data[self.index(row, column)]
    }

    fn rows(&self) -> usize {
        self.data.len() / self.n_columns
    }

    fn neighbors(&self, row: usize, column: usize) -> Vec<(usize, usize)> {
        let mut result = vec![];
        if row > 0 {
            result.push((row - 1, column));
        }
        if column > 0 {
            result.push((row, column - 1));
        }
        if column < self.n_columns - 1 {
            result.push((row, column + 1))
        }
        if row < self.rows() - 1 {
            result.push((row + 1, column))
        }
        result
    }
}

impl FromStr for Grid {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (values, columns) = s.split('\n').fold(Ok((vec![], None)), |v, s| {
            if v.is_err() {
                return v;
            }
            let (mut values, n_cols) = v.unwrap();
            let mut vs = s
                .chars()
                .map(|c| c.to_digit(10).map(|d| d as i8))
                .collect::<Option<Vec<_>>>()
                .context("unknown character")?;
            let v_len = vs.len();
            values.append(&mut vs);
            if let Some(l) = n_cols {
                if l != v_len {
                    bail!("Column size is not the same across rows ")
                }
            }
            Ok((values, Some(v_len)))
        })?;
        let columns = columns.context("could not read matrix")?;
        Ok(Grid {
            data: values,
            n_columns: columns,
        })
    }
}

fn find_basin(grid: &Grid, i: usize) -> Vec<bool> {
    let mut result = vec![false; grid.len()];
    let mut to_visit = vec![i];
    while !to_visit.is_empty() {
        let next = to_visit.pop().unwrap();
        if grid.data[next] != 9 {
            result[next] = true;
            let (row, col) = grid.from_index(next);
            let mut added = grid
                .neighbors(row, col)
                .into_iter()
                .map(|(r, c)| grid.index(r, c))
                .filter(|&i| !result[i])
                .collect();
            to_visit.append(&mut added);
        }
    }
    result
}

fn basins(grid: &Grid) -> Vec<usize> {
    let mut basins = vec![];
    let mut visited = vec![false; grid.len()];
    for i in 0..visited.len() {
        if !visited[i] {
            let basin = find_basin(grid, i);
            let mut count = 0;
            for (i, &v) in basin.iter().enumerate() {
                if v {
                    count += 1;
                    visited[i] = true;
                }
            }
            if count > 0 {
                basins.push(count);
            }
        }
    }
    basins
}

fn main() {
    let caves = Grid::from_str(aoc::get_input(21, 9).trim()).expect("Invalid grid provided");
    let minimas = (0..caves.len())
        .map(|i| (i, caves.data[i]))
        .filter(|&(i, v)| {
            let (r, c) = caves.from_index(i);
            caves
                .neighbors(r, c)
                .into_iter()
                .map(|(r, c)| caves.at(r, c))
                .all(|n_v| n_v > v)
        })
        .map(|(_, v)| v as i32 + 1)
        .sum::<i32>();
    let mut basins = basins(&caves);
    basins.sort();
    println!("Part 1: {}", minimas);
    println!(
        "Part 2: {}",
        basins.iter().rev().take(3).fold(1, |acc, v| acc * v)
    );
}
