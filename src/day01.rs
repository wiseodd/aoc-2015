use anyhow::Result;
use std::fs;

pub fn day01() -> Result<()> {
    let input: String = fs::read_to_string("data/day01_input.txt")?;

    println!("Day 1 Part 1: {}", part1(input.clone()));
    println!("Day 1 Part 2: {}", part2(input.clone()));

    Ok(())
}

fn part1(input: String) -> i32 {
    let mut res: i32 = 0;

    for char in input.chars() {
        res += match char {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };
    }

    res
}

fn part2(input: String) -> usize {
    let mut res: i32 = 0;

    for (idx, char) in input.chars().enumerate() {
        res += match char {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };

        if res == -1 {
            return idx + 1;
        }
    }

    input.chars().count()
}
