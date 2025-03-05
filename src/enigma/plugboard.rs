use std::collections::HashSet;
use std::str::FromStr;

use super::wiring::Wiring;

const IDENTITY: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub struct Plugboard {
  wiring: Wiring,
}

impl Plugboard {
  pub fn swap(&self, c: char) -> char {
    let index = (c as u8 - b'A') as usize;
    self.wiring.forward[index]
  }
}

impl FromStr for Plugboard {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut mapping: Vec<char> = IDENTITY.chars().collect();
    let mut used_chars = HashSet::new();

    // Process space-separated pairs
    for pair in s.split_whitespace() {
      let chars: Vec<char> = pair.chars().collect();
      if chars.len() != 2 {
        return Err(format!("Invalid plugboard pair: {}", pair));
      }

      let (a, b) = (chars[0], chars[1]);

      if !a.is_ascii_uppercase() || !b.is_ascii_uppercase() {
        return Err(format!("Invalid characters in plugboard pair: {}", pair));
      }

      if used_chars.contains(&a) || used_chars.contains(&b) {
        return Err(format!("Character {} or {} already used in another pair", a, b));
      }

      used_chars.insert(a);
      used_chars.insert(b);

      // Swap them in the mapping
      let idx_a = (a as u8 - b'A') as usize;
      let idx_b = (b as u8 - b'A') as usize;
      mapping.swap(idx_a, idx_b);
    }

    let encoded_string: String = mapping.iter().collect();

    Ok(Self {
      wiring: Wiring::from_str(&encoded_string).unwrap(),
    })
  }
}

impl Default for Plugboard {
  fn default() -> Self {
    Self {
      wiring: Wiring::from_str(IDENTITY).unwrap(),
    }
  }
}
