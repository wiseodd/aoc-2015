use anyhow::Result;
use std::fs;

pub fn solve() -> Result<()> {
    let input: String = fs::read_to_string("data/day05_input.txt")?;

    println!("Day 5 Part 1: {}", part1(input.clone()));
    println!("Day 5 Part 2: {}", part2(input.clone()));

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

fn part2(input: String) -> usize {
    input
        .trim()
        .split('\n')
        .map(|line| {
            // Requirement 1
            let mut repeat_ok = false;

            for i in 1..line.len() {
                let c1 = line.chars().nth(i - 1).unwrap();
                let c2 = line.chars().nth(i).unwrap();

                if let Some(_) = line.split_at(i + 1).1.find(&String::from_iter([c1, c2])) {
                    repeat_ok = true;
                    break;
                }
            }

            // Requirement 2
            let mut between_ok = false;

            for chars in line.chars().collect::<Vec<char>>().windows(3) {
                if let [a, _, c] = chars {
                    if a == c {
                        between_ok = true;
                        break;
                    }
                }
            }

            let ok = repeat_ok && between_ok;
            ok as usize
        })
        .sum()
}
