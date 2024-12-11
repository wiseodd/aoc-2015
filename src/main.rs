use anyhow::{anyhow, Result};
use std::env;

pub mod day01;
pub mod day02;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        return Err(anyhow!("Enter the day!"));
    }

    let module: &str = &args[1];

    match module {
        "day01" => day01::solve()?,
        "day02" => day02::solve()?,
        _ => println!("Not found!"),
    }

    Ok(())
}
