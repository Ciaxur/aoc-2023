use std::collections::VecDeque;
use std::env;
use std::fs::File;

static NAMED_DIGIT: [(&str, char); 9] = [
  ("one", '1'),
  ("two", '2'),
  ("three", '3'),
  ("four", '4'),
  ("five", '5'),
  ("six", '6'),
  ("seven", '7'),
  ("eight", '8'),
  ("nine", '9'),
];

fn find_digit_name_in_str(s: &str) -> (char, bool) {
  // Iterate over all known digits, finding a substring match.
  for (named_digit, value) in NAMED_DIGIT {
    if s.contains(named_digit) {
      println!("{} contains {}!", s, named_digit);
      return (value, true);
    }
  }
  ('\0', false)
}

/// Finds the left/right-most digit within the given string,
/// returning the value.
fn find_digit(s: String, is_left_most: bool) -> char {
  // Search based on search side.
  let mut s_char_vec: Vec<char> = s.chars().collect();
  if !is_left_most {
    s_char_vec = s.chars().rev().collect();
  }

  // Buffer each character.
  let mut digit_char_buffer: VecDeque<char> = VecDeque::new();

  for c in s_char_vec {
    // Buffer each character based on search order into deque.
    if is_left_most {
      digit_char_buffer.push_back(c);
    } else {
      digit_char_buffer.push_front(c);
    }

    // Numeric-priority order.
    if c.is_numeric() {
      // Cast character to number.
      println!("numeric found -> {c}");
      return c;
    }

    // Check digit in english.
    let digit_word: String = digit_char_buffer.iter().collect();
    println!("Buffer -> {digit_word}");
    let (value, found) = find_digit_name_in_str(&digit_word);
    if found {
      println!("named numeric found -> {value}");
      return value;
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
    let l_num = find_digit(line.clone(), true);
    let r_num = find_digit(line.clone(), false);

    // Combine them and accumulate.
    let calib_value_str = format!("{l_num}{r_num}");
    let calib_value_u32: u32 = calib_value_str.parse().unwrap();
    accumulated_calibration += calib_value_u32;
    println!("Calibration found - {calib_value_u32}");
  }

  println!("Calibration -> {accumulated_calibration}");
}
