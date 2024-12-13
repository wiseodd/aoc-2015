use anyhow::Result;
use std::fs;

pub fn solve() -> Result<()> {
    let input: String = fs::read_to_string("data/day04_input.txt")?;

    println!("Day 4 Part 1: {}", part12(input.clone(), 5));
    println!("Day 4 Part 2: {}", part12(input.clone(), 6));

    Ok(())
}

fn part12(input: String, leading_zeros: usize) -> i32 {
    for i in 0..10000000 {
        let digest = md5::compute(format!("{}{}", input.trim(), i));
        let hash: String = format!("{:x}", digest);

        if hash.starts_with(&"0".repeat(leading_zeros)) {
            return i;
        }
    }

    -1
}
