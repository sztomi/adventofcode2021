use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::Result;

const INPUT_FILE: &str = "inputs/day1.txt";

fn get_input() -> Result<Vec<i32>> {
  let file = File::open(&INPUT_FILE)?;
  let reader = BufReader::new(&file);
  let mut res = Vec::new();
  for line in reader.lines() {
    res.push(line?.parse::<i32>()?);
  }

  Ok(res)
}

fn count_increases(numbers: &Vec<i32>) -> usize {
  let mut previous: Option<i32> = None;
  let mut increases: usize = 0;
  for current in numbers {
    let prev = previous.unwrap_or(i32::MAX);
    if *current > prev {
      increases += 1;
    }
    previous = Some(*current);
  }
  increases
}

fn main() -> Result<()> {
  let numbers = get_input()?;
  println!("Part 1: {}", count_increases(&numbers));

  let mut sliding_sums = Vec::new();
  for i in 0..numbers.len() - 2 {
    sliding_sums.push(numbers[i] + numbers[i + 1] + numbers[i + 2])
  }
  println!("Part 2: {}", count_increases(&sliding_sums));

  Ok(())
}
