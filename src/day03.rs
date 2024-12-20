use anyhow::Result;
use rustc_hash::FxHashSet;
use std::fs;

pub fn solve() -> Result<()> {
    let input: String = fs::read_to_string("data/day03_input.txt")?;

    println!("Day 3 Part 1: {}", part1(input.clone()));
    println!("Day 3 Part 2: {}", part2(input.clone()));

    Ok(())
}

fn part1(input: String) -> usize {
    let mut loc: (i32, i32) = (0, 0);
    let mut visited = FxHashSet::default();
    visited.insert(loc);

    for c in input.trim().chars() {
        loc = go(loc, c);
        update(loc, &mut visited);
    }

    visited.len()
}

fn part2(input: String) -> usize {
    let mut loc1: (i32, i32) = (0, 0);
    let mut loc2: (i32, i32) = (0, 0);
    let mut visited = FxHashSet::default();
    visited.insert(loc1);

    for (i, c) in input.trim().chars().enumerate() {
        if i % 2 == 0 {
            loc1 = go(loc1, c);
            update(loc1, &mut visited);
        } else {
            loc2 = go(loc2, c);
            update(loc2, &mut visited);
        }
    }

    visited.len()
}

fn go(loc: (i32, i32), dir: char) -> (i32, i32) {
    match dir {
        '>' => (loc.0 + 1, loc.1),
        'v' => (loc.0, loc.1 + 1),
        '<' => (loc.0 - 1, loc.1),
        _ => (loc.0, loc.1 - 1),
    }
}

fn update(loc: (i32, i32), visited: &mut FxHashSet<(i32, i32)>) -> () {
    if let None = visited.get(&loc) {
        visited.insert(loc);
    }
}
