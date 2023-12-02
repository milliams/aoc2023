use anyhow::{Context, Result};
use aoc_2023::read_lines;
use format as f;
use itertools::Itertools;
use std::io::BufRead;

fn first_last_digit_to_int(line: &str) -> Result<u32> {
    let mut digits = line.chars().filter(|c| c.is_ascii_digit());
    let first = digits.next().context("No digits found")?;
    let last = match digits.last() {
        Some(d) => d,
        None => first,
    };
    let out = f!("{}{}", first, last).parse()?;
    Ok(out)
}

fn first_last_digits_words_to_int(line: &str) -> Result<u32> {
    let digits: Vec<u32> = (0..line.len())
        .map(|i| {
            let s = &line.to_string()[i..];
            if s.starts_with("one") || s.starts_with("1") {
                Some(1)
            } else if s.starts_with("two") || s.starts_with("2") {
                Some(2)
            } else if s.starts_with("three") || s.starts_with("3") {
                Some(3)
            } else if s.starts_with("four") || s.starts_with("4") {
                Some(4)
            } else if s.starts_with("five") || s.starts_with("5") {
                Some(5)
            } else if s.starts_with("six") || s.starts_with("6") {
                Some(6)
            } else if s.starts_with("seven") || s.starts_with("7") {
                Some(7)
            } else if s.starts_with("eight") || s.starts_with("8") {
                Some(8)
            } else if s.starts_with("nine") || s.starts_with("9") {
                Some(9)
            } else {
                None
            }
        })
        .filter_map(|v| v)
        .collect();
    let first = digits.first().context("No digits found")?;
    let last = match digits.last() {
        Some(d) => d,
        None => first,
    };
    let out = f!("{}{}", first, last).parse()?;
    Ok(out)
}

fn main() -> Result<()> {
    let first: u32 = read_lines!("data/day01.txt")
        .map(|l| first_last_digit_to_int(&l))
        .process_results(|iter| iter.sum())?;
    println!("First: {}", first);
    let second: u32 = read_lines!("data/day01.txt")
        .map(|l| first_last_digits_words_to_int(&l))
        .process_results(|iter| iter.sum())?;
    println!("Second: {}", second);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_first() -> Result<()> {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let expected = [12, 38, 15, 77];
        assert_eq!(
            input
                .split('\n')
                .map(first_last_digit_to_int)
                .collect::<Result<Vec<_>>>()?,
            expected
        );
        Ok(())
    }
    #[rstest::rstest]
    fn test_second() -> Result<()> {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let expected = [29, 83, 13, 24, 42, 14, 76];
        assert_eq!(
            input
                .split('\n')
                .map(first_last_digits_words_to_int)
                .collect::<Result<Vec<_>>>()?,
            expected
        );
        Ok(())
    }
}
