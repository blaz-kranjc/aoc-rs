use anyhow::{bail, Context};
use array_init::array_init;
use std::str::FromStr;

// TODO should be a bitset, but no bitset in std
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Segments([bool; 7]);

impl FromStr for Segments {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = [false; 7];
        for c in s.chars() {
            let index = c as usize - 'a' as usize;
            if index < 7 {
                result[index] = true;
            } else {
                bail!("Unknown character in segment");
            }
        }
        Ok(Segments(result))
    }
}

impl Segments {
    fn n_active(&self) -> usize {
        self.0.iter().filter(|&&x| x).count()
    }

    fn contains_all(&self, other: &Segments) -> bool {
        self.0
            .iter()
            .zip(other.0.iter())
            .map(|(&a, &b)| !b || b && a)
            .all(|x| x)
    }
}

#[derive(Debug)]
struct Display([i8; 4]);

impl Display {
    fn value(&self) -> i32 {
        let [w, x, y, z] = self.0;
        w as i32 * 1000 + x as i32 * 100 + y as i32 * 10 + z as i32
    }
}

#[derive(Debug)]
struct ScrambledDisplay {
    private: [Segments; 10],
    public: [Segments; 4],
}

impl FromStr for ScrambledDisplay {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const SEPARATOR: &str = " | ";
        let separator_id = s.find(SEPARATOR).context("missing separator")?;
        let privates = s[..separator_id]
            .split(' ')
            .filter_map(|s| Segments::from_str(s).ok())
            .collect::<Vec<Segments>>();
        let publics = s[separator_id + SEPARATOR.len()..]
            .split(' ')
            .filter_map(|s| Segments::from_str(s).ok())
            .collect::<Vec<Segments>>();
        if privates.len() != 10 || publics.len() != 4 {
            bail!("Wrong number of digits");
        }
        // TODO find a good way to create these values
        Ok(ScrambledDisplay {
            private: array_init(|i| privates[i]),
            public: array_init(|i| publics[i]),
        })
    }
}

impl ScrambledDisplay {
    // TODO this is very ugly
    fn resolve(&self) -> anyhow::Result<[&Segments; 10]> {
        let privates_it = self.private.iter();
        let one = privates_it
            .clone()
            .find(|d| d.n_active() == 2)
            .context("no 1")?;
        let four = privates_it
            .clone()
            .find(|d| d.n_active() == 4)
            .context("no 4")?;
        let seven = privates_it
            .clone()
            .find(|d| d.n_active() == 3)
            .context("no 7")?;
        let eight = privates_it
            .clone()
            .find(|d| d.n_active() == 7)
            .context("no 8")?;
        let six = privates_it
            .clone()
            .find(|d| d.n_active() == 6 && !d.contains_all(seven))
            .context("no 6")?;
        let nine = privates_it
            .clone()
            .find(|d| d.n_active() == 6 && d.contains_all(four))
            .context("no 9")?;
        let zero = privates_it
            .clone()
            .find(|d| d.n_active() == 6 && **d != *nine && **d != *six)
            .context("no 0")?;
        let three = privates_it
            .clone()
            .find(|d| d.n_active() == 5 && d.contains_all(one))
            .context("no 3")?;
        let five = privates_it
            .clone()
            .find(|d| d.n_active() == 5 && six.contains_all(d))
            .context("no 5")?;
        let two = privates_it
            .clone()
            .find(|d| d.n_active() == 5 && **d != *three && **d != *five)
            .context("no 2")?;
        Ok([zero, one, two, three, four, five, six, seven, eight, nine])
    }

    fn read(&self) -> anyhow::Result<Display> {
        let resolved = self.resolve()?;
        let mut result = [0; 4];
        for (index, segments) in self.public.iter().enumerate() {
            result[index] = resolved
                .iter()
                .position(|s| **s == *segments)
                .context("unknown display")? as i8;
        }
        Ok(Display(result))
    }
}

fn is_simple(n: i8) -> bool {
    n == 1 || n == 4 || n == 7 || n == 8
}

fn main() {
    let displays = aoc::get_input(21, 8)
        .split('\n')
        .filter_map(|s| ScrambledDisplay::from_str(s).ok())
        .filter_map(|d| d.read().ok())
        .collect::<Vec<_>>();
    println!(
        "Part 1: {}",
        displays
            .iter()
            .map(|d| d.0.iter().filter(|&&i| is_simple(i)).count())
            .sum::<usize>()
    );
    println!(
        "Part 2: {}",
        displays.iter().map(Display::value).sum::<i32>()
    );
}
