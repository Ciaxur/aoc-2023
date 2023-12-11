use std::env;
use std::fs::File;

/// Finds the left-most number within the given string,
/// returning the value.
fn find_number(s: String) -> char {
  for c in s.chars() {
    if c.is_numeric() {
      // Cast character to number.
      println!("numeric found -> {c}");
      return c;
    }
  }
  panic!("Edge-case!! Could not find number");
}

fn main() {
  // Grab input from cmd-line.
  let filepath_arg = env::args().nth(1);
  if filepath_arg.is_none() {
    panic!("Please supply a file to read from");
  }
  let filepath = filepath_arg.unwrap();
  println!("Reading input from -> {filepath}");

  // Open the input file.
  let input_fs = File::open(filepath).unwrap();

  // Iterate over each line within the content.
  let calibration_vec: Vec<String> = std::io::read_to_string(input_fs)
    .unwrap()
    .split("\n")
    .map(|s| String::from(s))
    .collect();

  // DEBUG: logs
  println!("Data -> {:?}", calibration_vec.to_vec());

  // Iterate over each line and grab the calibration.
  let mut accumulated_calibration: u32 = 0;
  for line in calibration_vec {
    // Search for right-most and left-most digits.
    println!("{line:?}");

    // Find left & right most numbers;
    let l_num = find_number(line.clone());
    let r_num = find_number(line.clone().chars().rev().collect());

    // Combine them and accumulate.
    let calib_value_str = format!("{l_num}{r_num}");
    let calib_value_u32: u32 = calib_value_str.parse().unwrap();
    accumulated_calibration += calib_value_u32;
    println!("Calibration found - {calib_value_u32}");
  }

  println!("Calibration -> {accumulated_calibration}");
}
