use anyhow::anyhow;
use anyhow::{Context, Result};
use aoc_2023::read_lines;
use itertools::Itertools;
use ndarray::{stack, Array1, Array2, Axis};
use std::io::BufRead;

#[derive(Debug)]
struct Number {
    row: u32,
    cols: std::ops::RangeInclusive<u32>,
    value: u32,
}

#[derive(Clone, Debug)]
enum Symbol {
    Digit(u32),
    Dot,
    Hash,
    Plus,
    Dollar,
    Star,
    Minus,
    And,
    Slash,
    At,
    Percent,
    Equals,
}

fn read_grid<I>(lines: I) -> Result<Array2<Symbol>>
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let grid: Result<Vec<Array1<_>>> = lines
        .into_iter()
        .enumerate()
        .map(|(_row, line)| {
            line.as_ref()
                .chars()
                .enumerate()
                .map(|(_col, d)| match d {
                    '0'..='9' => Ok(Symbol::Digit(d.to_digit(10).context("Cannot parse digit")?)),
                    '.' => Ok(Symbol::Dot),
                    '#' => Ok(Symbol::Hash),
                    '+' => Ok(Symbol::Plus),
                    '$' => Ok(Symbol::Dollar),
                    '*' => Ok(Symbol::Star),
                    '-' => Ok(Symbol::Minus),
                    '&' => Ok(Symbol::And),
                    '/' => Ok(Symbol::Slash),
                    '@' => Ok(Symbol::At),
                    '%' => Ok(Symbol::Percent),
                    '=' => Ok(Symbol::Equals),
                    s => Err(anyhow!("Mismatched symbol {}", s)),
                })
                .collect::<Result<Array1<_>>>()
        })
        .collect();
    let grid = grid?;
    let grid: Vec<_> = grid.iter().map(|x| x.view()).collect();
    let grid = stack(Axis(0), &grid)?;
    //println!("{:?}", grid);
    Ok(grid)
}

fn extract_numbers(g: &Array2<Symbol>) -> Result<Vec<Number>> {
    let nums = g
        .indexed_iter()
        .group_by(|((_row, _col), c)| matches!(c, Symbol::Digit(_)))
        .into_iter()
        .filter(|(k, _)| *k)
        .map(|(_, g)| {
            let g: Vec<((usize, usize), &Symbol)> = g.collect();
            let d: u32 = g
                .iter()
                .map(|c| match c.1 {
                    Symbol::Digit(d) => d,
                    _ => panic!(),
                })
                .fold(0, |acc, elem| acc * 10 + elem);
            let cols: Vec<usize> = g.iter().map(|c| c.0 .1).collect();
            Number {
                value: d,
                row: g[0].0 .0 as u32,
                cols: *cols.first().unwrap() as u32..=*cols.last().unwrap() as u32,
            }
        })
        .collect();
    //println!("{:?}", nums);
    Ok(nums)
}

fn is_part_num(n: &&Number, g: &Array2<Symbol>) -> bool {
    let r = n.row;
    let surround = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    n.cols
        .clone()
        .map(|c| {
            let here = (r, c);
            surround
                .iter()
                .map(|s| {
                    let i = (here.0 as i64 + s.0, here.1 as i64 + s.1);
                    if i.0 < 0
                        || i.1 < 0
                        || i.0 as usize >= g.len_of(Axis(0))
                        || i.1 as usize >= g.len_of(Axis(1))
                    {
                        false
                    } else {
                        let i = (i.0 as usize, i.1 as usize);
                        !matches!(g[i], Symbol::Digit(_) | Symbol::Dot)
                    }
                })
                .any(|e| e)
        })
        .any(|e| e)
}

fn main() -> Result<()> {
    let g = read_grid(read_lines!("data/day03.txt"))?;
    let first: u32 = extract_numbers(&g)?
        .iter()
        .filter(|n| is_part_num(n, &g))
        .map(|n| n.value)
        .sum();
    println!("First: {}", first);
    // let second: u32 = read_lines!("data/day02.txt")
    //     .map(|l| min_possible(&l))
    //     .collect::<Result<Vec<_>>>()?
    //     .iter()
    //     .sum::<u32>();
    // println!("Second: {}", second);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_first() -> Result<()> {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let g = read_grid(input.lines())?;
        let s: u32 = extract_numbers(&g)?
            .iter()
            .filter(|n| is_part_num(n, &g))
            .map(|n| n.value)
            .sum();
        assert_eq!(s, 4361);
        Ok(())
    }
}
