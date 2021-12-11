use std::str::FromStr;

use anyhow::{bail, Context};

#[derive(Debug, Clone)]
struct NormalGrid {
    data: Vec<bool>,
    n_columns: usize,
}

trait Grid {
    fn get(&self, i: usize) -> bool;
    fn set(&mut self, i: usize, v: bool);
    fn size(&self) -> usize;
    fn n_columns(&self) -> usize;
}

fn neighbors(i: usize, size: usize, n_cols: usize) -> Vec<usize> {
    let mut result = vec![];
    let to_index = |row: usize, col: usize| row * n_cols + col;
    let (r, c) = (i / n_cols, i % n_cols);
    let n_rows = size / n_cols;
    if r > 0 {
        result.push(to_index(r - 1, c));
    }
    if r < n_rows - 1 {
        result.push(to_index(r + 1, c));
    }
    if r > 0 && c > 0 {
        result.push(to_index(r - 1, c - 1));
    }
    if r < n_rows - 1 && c > 0 {
        result.push(to_index(r + 1, c - 1));
    }
    if c > 0 {
        result.push(to_index(r, c - 1));
    }
    if c < n_cols - 1 {
        result.push(to_index(r, c + 1));
    }
    if r > 0 && c < n_cols - 1 {
        result.push(to_index(r - 1, c + 1));
    }
    if r < n_rows - 1 && c < n_cols - 1 {
        result.push(to_index(r + 1, c + 1));
    }
    result
}

impl Grid for NormalGrid {
    fn get(&self, i: usize) -> bool {
        self.data[i]
    }

    fn set(&mut self, i: usize, v: bool) {
        self.data[i] = v;
    }

    fn size(&self) -> usize {
        self.data.len()
    }

    fn n_columns(&self) -> usize {
        self.n_columns
    }
}

fn parse_cell(c: char) -> Option<bool> {
    match c {
        '#' => Some(true),
        '.' => Some(false),
        _ => None,
    }
}

impl FromStr for NormalGrid {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows = s
            .split('\n')
            .map(|r| r.chars().map(|c| parse_cell(c)).collect::<Option<Vec<_>>>())
            .collect::<Option<Vec<_>>>()
            .context("Invalid input characters")?;
        if rows.is_empty() {
            bail!("No rows supplied");
        }
        let n_columns = rows[0].len();
        if rows.iter().skip(1).any(|v| v.len() != n_columns) {
            bail!("Not all rows of the same size");
        }
        Ok(NormalGrid {
            data: rows.into_iter().flatten().collect(),
            n_columns,
        })
    }
}

fn step(grid: &mut impl Grid) {
    let neighbors = (0..grid.size())
        .map(|i| {
            neighbors(i, grid.size(), grid.n_columns())
                .into_iter()
                .filter(|&n| grid.get(n))
                .count()
        })
        .collect::<Vec<_>>();
    for i in 0..grid.size() {
        if grid.get(i) && (neighbors[i] < 2 || neighbors[i] > 3) {
            grid.set(i, false);
        } else if !grid.get(i) && neighbors[i] == 3 {
            grid.set(i, true);
        }
    }
}

struct BrokenGrid(NormalGrid);

impl Grid for BrokenGrid {
    fn get(&self, i: usize) -> bool {
        let (r, c) = (i / self.n_columns(), i % self.n_columns());
        let n_rows = self.size() / self.n_columns();
        if (r, c) == (0, 0)
            || (r, c) == (n_rows - 1, 0)
            || (r, c) == (0, self.n_columns() - 1)
            || (r, c) == (n_rows - 1, self.n_columns() - 1)
        {
            true
        } else {
            self.0.get(i)
        }
    }

    fn set(&mut self, i: usize, v: bool) {
        self.0.data[i] = v;
    }

    fn size(&self) -> usize {
        self.0.size()
    }

    fn n_columns(&self) -> usize {
        self.0.n_columns()
    }
}

fn main() {
    let state = NormalGrid::from_str(aoc::get_input(15, 18).trim()).expect("invalid input");

    let mut normal = state.clone();
    for _ in 0..100 {
        step(&mut normal);
    }
    println!(
        "Part 1: {}",
        (0..normal.size())
            .map(|i| normal.get(i))
            .filter(|&x| x)
            .count()
    );

    let mut broken = BrokenGrid(state);
    for _ in 0..100 {
        step(&mut broken);
    }
    println!(
        "Part 2: {}",
        (0..broken.size())
            .map(|i| broken.get(i))
            .filter(|&x| x)
            .count()
    );
}
