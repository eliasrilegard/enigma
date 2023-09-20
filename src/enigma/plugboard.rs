use std::collections::HashSet;

use super::common::encode_char;

pub struct Plugboard {
  mapping: Vec<u8>
}

impl Plugboard {
  pub fn transform(&self, letter: u8) -> u8 {
    self.mapping[letter as usize]
  }

  pub fn from_mapping(mapping: &str) -> Self {
    let mut plugboard = (0..26).collect::<Vec<u8>>();

    if mapping.is_empty() {
      return Plugboard::identity();
    }

    let pairs = mapping.split_whitespace().collect::<Vec<_>>();

    if pairs.iter().any(|pair| pair.len() != 2) {
      return Plugboard::identity();
    }

    let mut plugged_letters = HashSet::new();

    for pair in pairs {
      let mut chars = pair.chars();
      let c1 = chars.next().unwrap();
      let c2 = chars.next().unwrap();

      if plugged_letters.contains(&c1) || plugged_letters.contains(&c2) {
        return Plugboard::identity();
      }

      plugged_letters.insert(c1);
      plugged_letters.insert(c2);

      let c1_encoded = encode_char(c1);
      let c2_encoded = encode_char(c2);

      plugboard[c1_encoded as usize] = c2_encoded;
      plugboard[c2_encoded as usize] = c1_encoded;
    }

    Self { mapping: plugboard }
  }
}

impl Plugboard {
  fn identity() -> Self {
    Self {
      mapping: (0..26).collect::<Vec<u8>>()
    }
  }
}
