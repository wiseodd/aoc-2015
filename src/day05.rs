use anyhow::Result;
use std::fs;

pub fn solve() -> Result<()> {
    let input: String = fs::read_to_string("data/day05_input.txt")?;
    // let input: String = fs::read_to_string("data/day05_input_toy.txt")?;

    println!("Day 5 Part 1: {}", part1(input.clone()));
    // println!("Day 5 Part 2: {}", part12(input.clone(), 6));

    Ok(())
}

fn part1(input: String) -> usize {
    input
        .trim()
        .split('\n')
        .map(|line| {
            // Requirement 1
            let vowel_count: usize = "aiueo"
                .chars()
                .map(|vowel| line.chars().filter(|c| *c == vowel).count())
                .sum();

            // Requirement 2
            let mut repeat_count: usize = 0;
            for i in 1..line.len() {
                if line.chars().nth(i - 1).unwrap() == line.chars().nth(i).unwrap() {
                    repeat_count += 1;
                }
            }

            // Requirement 3
            let not_containing_count: usize = ["ab", "cd", "pq", "xy"]
                .iter()
                .map(|bad_str| line.find(bad_str).map_or(0, |_| 1))
                .sum();

            let ok = (vowel_count >= 3) && (repeat_count > 0) && (not_containing_count == 0);

            ok as usize
        })
        .sum()
}
