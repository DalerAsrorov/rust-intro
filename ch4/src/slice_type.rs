
pub fn first_word(s: &str) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[0..i];
    }
  }

  &s[..]
}

pub fn second_word(s: &str) -> &str {
  let bytes = s.trim().as_bytes();
  let mut indexes = (0, s.len());

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      if indexes.0 == 0 {
        indexes = (i, indexes.1);
      } else if indexes.0 != 0 {
        indexes = (indexes.0 + 1, i);
        break;
      }
    }
  }

  &s[indexes.0..indexes.1]
}