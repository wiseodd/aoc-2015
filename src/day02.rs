use anyhow::Result;
use std::fs;

pub fn solve() -> Result<()> {
    let input: String = fs::read_to_string("data/day02_input.txt")?;

    println!("Day 2 Part 1: {}", part1(input.clone()));
    println!("Day 2 Part 2: {}", part2(input.clone()));

    Ok(())
}

fn part1(input: String) -> usize {
    let mut res: usize = 0;

    for line in input.trim().split('\n') {
        let mut lwh: Vec<usize> = line
            .split('x')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        if let [l, w, h] = lwh.as_slice() {
            res += 2 * l * w + 2 * w * h + 2 * h * l;
            lwh.sort();
            res += lwh[0] * lwh[1];
        }
    }

    res
}

fn part2(input: String) -> usize {
    let mut res: usize = 0;

    for line in input.trim().split('\n') {
        let mut lwh: Vec<usize> = line
            .split('x')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        lwh.sort();

        res += 2 * lwh[0] + 2 * lwh[1] + lwh.iter().product::<usize>();
    }

    res
}
