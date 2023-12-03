use anyhow::{Context, Result};
use aoc_2023::read_lines;
use itertools::Itertools;
use std::io::BufRead;

#[derive(Debug)]
struct Game {
    number: u32,
    sets: Vec<Set>,
}

impl Game {
    fn try_new(line: &str) -> Result<Self> {
        let (game, sets) = line
            .split(':')
            .collect_tuple()
            .context("Could not get game and sets")?;
        Ok(Game {
            number: game
                .split_whitespace()
                .last()
                .context("No game number")?
                .parse()?,
            sets: sets
                .split(';')
                .map(|s| {
                    let mut set = Set::default();
                    s.split(',').for_each(|c| {
                        let (count, colour) = c
                            .split_whitespace()
                            .collect_tuple()
                            .context("Not count and colour")
                            .unwrap();

                        let count = count.parse::<u32>().expect("Could not parse number");
                        match colour {
                            "red" => {
                                set.red = if let Some(ref c) = set.red {
                                    Some(c + count)
                                } else {
                                    Some(count)
                                }
                            }
                            "green" => {
                                set.green = if let Some(ref c) = set.green {
                                    Some(c + count)
                                } else {
                                    Some(count)
                                }
                            }
                            "blue" => {
                                set.blue = if let Some(ref c) = set.blue {
                                    Some(c + count)
                                } else {
                                    Some(count)
                                }
                            }
                            &_ => println!("No match for colour"),
                        };
                    });
                    set
                })
                .collect(),
        })
    }
}

#[derive(Default, Debug)]
struct Set {
    red: Option<u32>,
    green: Option<u32>,
    blue: Option<u32>,
}

impl Set {
    fn can_allow(&self, other: &Self) -> bool {
        self.red >= other.red && self.green >= other.green && self.blue >= other.blue
    }

    fn power(&self) -> u32 {
        self.red.unwrap() * self.green.unwrap() * self.blue.unwrap()
    }
}

fn game_possible(line: &str) -> Result<Option<u32>> {
    let game = Game::try_new(line)?;
    let max_set = Set {
        red: game.sets.iter().map(|s| s.red).max().context("No sets")?,
        green: game.sets.iter().map(|s| s.green).max().context("No sets")?,
        blue: game.sets.iter().map(|s| s.blue).max().context("No sets")?,
    };

    let compare = Set {
        red: Some(12),
        blue: Some(14),
        green: Some(13),
    };

    if compare.can_allow(&max_set) {
        Ok(Some(game.number))
    } else {
        Ok(None)
    }
}

fn min_possible(line: &str) -> Result<u32> {
    let game = Game::try_new(line)?;
    let min = Set {
        red: Some(
            game.sets
                .iter()
                .map(|s| s.red.unwrap_or_default())
                .max()
                .context("No sets")?,
        ),
        green: Some(
            game.sets
                .iter()
                .map(|s| s.green.unwrap_or_default())
                .max()
                .context("No sets")?,
        ),
        blue: Some(
            game.sets
                .iter()
                .map(|s| s.blue.unwrap_or_default())
                .max()
                .context("No sets")?,
        ),
    };
    Ok(min.power())
}

fn main() -> Result<()> {
    let first: u32 = read_lines!("data/day02.txt")
        .map(|l| game_possible(&l))
        .collect::<Result<Vec<_>>>()?
        .iter()
        .filter_map(|&n| n)
        .sum();
    println!("First: {}", first);
    let second: u32 = read_lines!("data/day02.txt")
        .map(|l| min_possible(&l))
        .collect::<Result<Vec<_>>>()?
        .iter()
        .sum::<u32>();
    println!("Second: {}", second);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_first() -> Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let expected = [1, 2, 5];
        assert_eq!(
            input
                .split('\n')
                .map(game_possible)
                .collect::<Result<Vec<_>>>()?
                .iter()
                .filter_map(|&n| n)
                .collect::<Vec<u32>>(),
            expected
        );
        Ok(())
    }
    #[test]
    fn test_second() -> Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let expected = 2286;
        assert_eq!(
            input
                .split('\n')
                .map(min_possible)
                .collect::<Result<Vec<_>>>()?
                .iter()
                .sum::<u32>(),
            expected
        );
        Ok(())
    }
}
