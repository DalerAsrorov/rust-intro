use std::collections::HashMap;

#[derive(Debug)]
pub struct CalcMetaData {
  avg: f64,
  median: i32,
  mode: i32
}

pub fn calculate_avg_median_mode(numbers: &mut Vec<i32>) -> CalcMetaData {
  CalcMetaData {
    avg: calculate_avg(numbers),
    mode: calculate_mode(numbers),
    median: calculate_median(numbers),
  }
}

fn calculate_avg(numbers: &mut Vec<i32>) -> f64 {
  let mut sum: f64 = 0.0;
  let len = numbers.len() as f64;

  for num in numbers.iter() {
    sum = sum + f64::from(*num);
  }

  sum / len
}

fn calculate_median(numbers: &mut Vec<i32>) -> i32 {
  numbers.sort();

  numbers[numbers.len() / 2]
}

fn calculate_mode(numbers: &mut Vec<i32>) -> i32 {
  let mut numbers_count_map: HashMap<i32, i32> = HashMap::new();

  for num in numbers.iter() {
    let count = numbers_count_map.entry(*num).or_insert(0);
    *count += 1;
  }

  let mut highest_counter = numbers_count_map.get(&numbers[0]);
  let mut res = numbers[0];

  for (k, val) in numbers_count_map.iter() {
    let count = Some(val);

    if highest_counter < count {
      highest_counter = count;
      res = *k;
    }
  }

  res
}