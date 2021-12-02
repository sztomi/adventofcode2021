use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::{anyhow, Result};

const INPUT_FILE: &str = "inputs/day2.txt";

type Commands = Vec<(String, i32)>;

fn get_input() -> Result<Commands> {
  let file = File::open(&INPUT_FILE)?;
  let reader = BufReader::new(&file);
  let mut res = Vec::new();
  for line in reader.lines() {
    let line = line?;
    let mut parts = line.split(" ");
    let cmd = parts.next().ok_or(anyhow!("poop cmd"))?.to_string();
    let value = parts.next().ok_or(anyhow!("poop value"))?.parse::<i32>()?;
    res.push((cmd, value));
  }
  Ok(res)
}

fn get_pos_part1(commands: &Commands) -> Result<(i32, i32)> {
  let mut horiz = 0;
  let mut depth = 0;
  for (cmd, value) in commands {
    match cmd.as_str() {
      "forward" => horiz += value,
      "down" => depth += value,
      "up" => depth -= value,
      _ => panic!("at the disco"),
    }
  }

  Ok((horiz, depth))
}

fn get_pos_part2(commands: &Commands) -> Result<(i32, i32)> {
  let mut horiz = 0;
  let mut depth = 0;
  let mut aim = 0;
  for (cmd, value) in commands {
    match cmd.as_str() {
      "down" => aim += value,
      "up" => aim -= value,
      "forward" => {
        horiz += value;
        depth += aim * value;
      }
      _ => panic!("at the disco"),
    }
  }

  Ok((horiz, depth))
}

fn main() -> Result<()> {
  let commands = get_input()?;
  let pos = get_pos_part1(&commands)?;
  println!("Part 1: {}", pos.0 * pos.1);
  let pos = get_pos_part2(&commands)?;
  println!("Part 2: {}", pos.0 * pos.1);
  Ok(())
}
