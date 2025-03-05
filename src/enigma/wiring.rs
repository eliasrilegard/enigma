use std::str::FromStr;

pub struct Wiring {
  pub forward: [char; 26],
  pub backward: [char; 26],
}

impl FromStr for Wiring {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if s.len() != 26 {
      return Err("Input must be exactly 26 characters long".to_string());
    }

    let mut forward = [' '; 26];
    let mut backward = [' '; 26];

    for (i, c) in s.chars().enumerate() {
      if !c.is_ascii_uppercase() {
        return Err(format!("Invalid character '{}' in input", c));
      }

      let index = (c as u8 - b'A') as usize;
      forward[i] = c;
      backward[index] = (b'A' + i as u8) as char;
    }

    Ok(Wiring { forward, backward })
  }
}
