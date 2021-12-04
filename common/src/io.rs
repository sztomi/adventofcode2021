use std::{fs::File, path::Path};
use std::io::{BufReader, BufRead};
use crate::result::Result;

pub fn get_lines(file_name: &Path) -> Result<Vec<String>> {
  let file = File::open(&file_name)?;
  let reader = BufReader::new(&file);
  Ok(reader.lines().map(|line| line.unwrap()).collect())
}