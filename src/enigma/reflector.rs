use std::str::FromStr;

use super::ReflectorModel;
use super::wiring::Wiring;

const WIRINGS: [&str; 4] = [
  "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
  "EJMZALYXVBWFCRQUONTSPIKHGD",
  "YRUHQSLDPXNGOKMIEBFZCWVJAT",
  "FVPJIAOYEDRZXWGCTKUQSBNMHL",
];

pub struct Reflector {
  wiring: Wiring,
}

impl Reflector {
  pub fn from_model(model: ReflectorModel) -> Self {
    match model {
      ReflectorModel::Identity => Self {
        wiring: Wiring::from_str(WIRINGS[0]).unwrap(),
      },
      ReflectorModel::A => Self {
        wiring: Wiring::from_str(WIRINGS[1]).unwrap(),
      },
      ReflectorModel::B => Self {
        wiring: Wiring::from_str(WIRINGS[2]).unwrap(),
      },
      ReflectorModel::C => Self {
        wiring: Wiring::from_str(WIRINGS[3]).unwrap(),
      },
    }
  }

  pub fn reflect(&self, c: char) -> char {
    let index = (c as u8 - b'A') as usize;
    self.wiring.forward[index]
  }
}

impl Default for Reflector {
  fn default() -> Self {
    Reflector::from_model(ReflectorModel::Identity)
  }
}
