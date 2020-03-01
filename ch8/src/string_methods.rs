use std::collections::HashMap;

pub fn count_words(s: &str) -> HashMap<&str, usize> {
  let mut map = HashMap::new();

  for word in s.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
  }

  map
}