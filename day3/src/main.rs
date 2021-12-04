use std::path::PathBuf;

use common::get_lines;
use common::Result;

const INPUT_FILE: &str = "inputs/day3.txt";

fn bit_count(lines: &Vec<String>, bit_pos: usize) -> (usize, usize) {
  let ones = lines
    .iter()
    .filter(|line| line.as_bytes()[bit_pos] == b'1')
    .count();
  let zeros = lines.len() - ones;
  (ones, zeros)
}

fn frequent(bitcnt: (usize, usize), invert: bool, preferred: u8) -> u8 {
  let (ones, zeros) = bitcnt;
  if ones == zeros {
    preferred
  } else if (ones > zeros) ^ invert {
    b'1'
  } else {
    b'0'
  }
}

fn part1(lines: &Vec<String>) -> (u64, u64) {
  let max_bit = lines[0].len();
  let (mut gamma, mut epsilon) = (0u64, 0u64);

  for pos in 0..max_bit {
    let (ones, zeros) = bit_count(&lines, pos);
    if ones > zeros {
      gamma += 1 << (max_bit - pos - 1);
    } else {
      epsilon += 1 << (max_bit - pos - 1);
    }
  }
  (gamma, epsilon)
}

fn calc_filtered(lines: &Vec<String>, invert: bool, preferred: u8) -> u64 {
  let max_bit = lines[0].len();
  let mut filtered: Vec<String> = lines.iter().cloned().collect();

  for pos in 0..max_bit {
    filtered = filtered
      .iter()
      .filter(|&line| {
        let bitcnt = bit_count(&filtered, pos);
        line.as_bytes()[pos] == frequent(bitcnt, invert, preferred)
      })
      .cloned()
      .collect();
    if filtered.len() == 1 {
      break;
    }
  }

  let mut res = 0u64;
  let value = filtered[0].to_ascii_lowercase();
  for pos in 0..max_bit {
    let b = (value.as_bytes()[pos] - b'0') as u64;
    res += (b << (max_bit - pos - 1)) as u64;
  }
  res
}

fn part2(lines: &Vec<String>) -> u64 {
  let o2 = calc_filtered(&lines, false, b'1');
  let co2 = calc_filtered(&lines, true, b'0');
  o2 * co2
}

fn main() -> Result<()> {
  let lines = get_lines(&PathBuf::from(&INPUT_FILE))?;
  let (gamma, epsilon) = part1(&lines);
  println!("Part 1: {}", gamma * epsilon);
  println!("Part 2: {}", part2(&lines));
  Ok(())
}
