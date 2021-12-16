use std::{
    cmp,
    ops::{Add, Mul},
    str::FromStr,
};

use anyhow::Context;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Default, Clone)]
struct Recipe {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

impl FromStr for Recipe {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RECIPE_REGEX: Regex =
                Regex::new(r"(?P<name>[^:]*): capacity (?P<capacity>-?[0-9]+), durability (?P<durability>-?[0-9]+), flavor (?P<flavor>-?[0-9]+), texture (?P<texture>-?[0-9]+), calories (?P<calories>[0-9]+)").unwrap();
        }
        let parts = RECIPE_REGEX.captures(s).context("Invalid layout")?;
        Ok(Recipe {
            capacity: parts["capacity"].parse()?,
            durability: parts["durability"].parse()?,
            flavor: parts["flavor"].parse()?,
            texture: parts["texture"].parse()?,
            calories: parts["calories"].parse()?,
        })
    }
}

impl Add<Recipe> for Recipe {
    type Output = Recipe;
    fn add(self, rhs: Recipe) -> Self::Output {
        Recipe {
            capacity: self.capacity + rhs.capacity,
            durability: self.durability + rhs.durability,
            flavor: self.flavor + rhs.flavor,
            texture: self.texture + rhs.texture,
            calories: self.calories + rhs.calories,
        }
    }
}

impl Mul<i64> for Recipe {
    type Output = Recipe;
    fn mul(self, rhs: i64) -> Self::Output {
        Recipe {
            capacity: self.capacity * rhs,
            durability: self.durability * rhs,
            flavor: self.flavor * rhs,
            texture: self.texture * rhs,
            calories: self.calories * rhs,
        }
    }
}

fn score(recipe: &Recipe, calories: Option<i64>) -> i64 {
    if recipe.capacity <= 0
        || recipe.durability <= 0
        || recipe.flavor <= 0
        || recipe.texture <= 0
        || (calories.is_some() && recipe.calories != calories.unwrap())
    {
        0
    } else {
        recipe.capacity * recipe.durability * recipe.flavor * recipe.texture
    }
}

fn best_score(recipes: &[Recipe], amount: i64, calories: Option<i64>) -> i64 {
    fn maximize(
        recipes: &[Recipe],
        amount: i64,
        calories: Option<i64>,
        acc: Recipe,
        current_max: i64,
    ) -> i64 {
        let new_value = if recipes.len() == 1 {
            let recipe = acc + recipes[0].clone() * amount;
            score(&recipe, calories)
        } else if amount == 0 {
            score(&acc, calories)
        } else {
            let mut max_local = current_max;
            for i in 0..=amount {
                max_local = maximize(
                    &recipes[1..],
                    amount - i,
                    calories,
                    acc.clone() + recipes[0].clone() * i,
                    max_local,
                );
            }
            max_local
        };
        cmp::max(current_max, new_value)
    }
    maximize(recipes, amount, calories, Recipe::default(), 0)
}

fn main() {
    let recipes = aoc::get_input(15, 15)
        .trim()
        .split('\n')
        .map(|l| Recipe::from_str(l))
        .collect::<anyhow::Result<Vec<_>>>()
        .expect("invalid input");

    println!("Part 1: {}", best_score(&recipes, 100, None));
    println!("Part 2: {}", best_score(&recipes, 100, Some(500)));
}
