use std::str::FromStr;

use anyhow::{bail, Context};

#[derive(Debug)]
struct Grid {
    data: Vec<i8>,
    n_columns: usize,
}

impl Grid {
    fn neighbors(&self, i: usize) -> Vec<usize> {
        let mut result = vec![];
        let to_index = |row: usize, col: usize| row * self.n_columns + col;
        let (r, c) = (i / self.n_columns, i % self.n_columns);
        if r > 0 {
            result.push(to_index(r - 1, c));
        }
        if r < (self.data.len() / self.n_columns) - 1 {
            result.push(to_index(r + 1, c));
        }
        if c > 0 {
            result.push(to_index(r, c - 1));
        }
        if c < self.n_columns - 1 {
            result.push(to_index(r, c + 1));
        }
        result
    }
}

impl FromStr for Grid {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows = s
            .split('\n')
            .map(|r| {
                r.chars()
                    .map(|c| c.to_digit(10).map(|n| n as i8))
                    .collect::<Option<Vec<_>>>()
            })
            .collect::<Option<Vec<_>>>()
            .context("Invalid input characters")?;
        if rows.is_empty() {
            bail!("No rows supplied");
        }
        let n_columns = rows[0].len();
        if rows.iter().skip(1).any(|v| v.len() != n_columns) {
            bail!("Not all rows of the same size");
        }
        Ok(Grid {
            data: rows.into_iter().flatten().collect(),
            n_columns,
        })
    }
}

fn fill_basin(grid: &Grid, origin: usize, visited: &mut Vec<bool>) -> usize {
    let mut count = 0;
    let mut to_visit = vec![origin];
    while !to_visit.is_empty() {
        let next = to_visit.pop().unwrap();
        if visited[next] {
            continue;
        }
        visited[next] = true;
        if grid.data[next] != 9 {
            count += 1;
            for n in grid.neighbors(next) {
                if !visited[n] && !to_visit.contains(&n) {
                    to_visit.push(n);
                }
            }
        }
    }
    count
}

fn basins(grid: &Grid) -> Vec<usize> {
    let mut basins = vec![];
    let mut visited = vec![false; grid.data.len()];
    for i in 0..visited.len() {
        if !visited[i] {
            let basin = fill_basin(grid, i, &mut visited);
            if basin > 0 {
                basins.push(basin);
            }
        }
    }
    basins
}

fn main() {
    let caves = Grid::from_str(aoc::get_input(21, 9).trim()).expect("Invalid grid provided");
    let local_minimums = (0..caves.data.len())
        .map(|i| (i, caves.data[i]))
        .filter(|&(i, v)| caves.neighbors(i).into_iter().all(|i| caves.data[i] > v))
        .collect::<Vec<_>>();

    println!(
        "Part 1: {}",
        local_minimums
            .into_iter()
            .map(|(_, v)| v as i32 + 1)
            .sum::<i32>()
    );

    let mut basins = basins(&caves);
    basins.sort();
    println!(
        "Part 2: {}",
        basins.iter().rev().take(3).fold(1, |acc, v| acc * v)
    );
}
