use std::str::FromStr as _;

use super::RotorModel;
use super::wiring::Wiring;

const WIRINGS: [&str; 9] = [
  "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
  "EKMFLGDQVZNTOWYHXUSPAIBRCJ",
  "AJDKSIRUXBLHWTMCQGZNPYFVOE",
  "BDFHJLCPRTXVZNYEIWGAKMUSQO",
  "ESOVPZJAYQUIRHXLNFTGKDCMWB",
  "VZBRGITYUPSDNHLXAWMJQOFECK",
  "JPGVOUMFYQBENHZRDKASXLICTW",
  "NZJHGRCXMYSWBOUFAIVLPEKQDT",
  "FKQHTLXOCBJSPDZRAMEWNIUYGV",
];

pub struct Rotor {
  wiring: Wiring,
  /// The letter(s) containing notches, propagating the rotations forward
  notches: Vec<isize>,
  position: isize,
  /// Move the wiring that maps 'A' this far forward
  ring_setting: isize,
}

impl Rotor {
  pub fn new(model: RotorModel, position: char, ring_setting: usize) -> Self {
    let position_index = (position as u8 - b'A') as isize;
    let ring_index = (ring_setting - 1) as isize; // 1-indexed to follow convention

    match model {
      RotorModel::Identity => Self {
        wiring: Wiring::from_str(WIRINGS[0]).unwrap(),
        notches: vec![(b'A' - b'A') as isize],
        position: position_index,
        ring_setting: ring_index,
      },
      RotorModel::I => Self {
        wiring: Wiring::from_str(WIRINGS[1]).unwrap(),
        notches: vec![(b'Q' - b'A') as isize],
        position: position_index,
        ring_setting: ring_index,
      },
      RotorModel::II => Self {
        wiring: Wiring::from_str(WIRINGS[2]).unwrap(),
        notches: vec![(b'E' - b'A') as isize],
        position: position_index,
        ring_setting: ring_index,
      },
      RotorModel::III => Self {
        wiring: Wiring::from_str(WIRINGS[3]).unwrap(),
        notches: vec![(b'V' - b'A') as isize],
        position: position_index,
        ring_setting: ring_index,
      },
      RotorModel::IV => Self {
        wiring: Wiring::from_str(WIRINGS[4]).unwrap(),
        notches: vec![(b'J' - b'A') as isize],
        position: position_index,
        ring_setting: ring_index,
      },
      RotorModel::V => Self {
        wiring: Wiring::from_str(WIRINGS[5]).unwrap(),
        notches: vec![(b'Z' - b'A') as isize],
        position: position_index,
        ring_setting: ring_index,
      },
      RotorModel::VI => Self {
        wiring: Wiring::from_str(WIRINGS[6]).unwrap(),
        notches: vec![(b'M' - b'A') as isize, (b'Z' - b'A') as isize],
        position: position_index,
        ring_setting: ring_index,
      },
      RotorModel::VII => Self {
        wiring: Wiring::from_str(WIRINGS[7]).unwrap(),
        notches: vec![(b'M' - b'A') as isize, (b'Z' - b'A') as isize],
        position: position_index,
        ring_setting: ring_index,
      },
      RotorModel::VIII => Self {
        wiring: Wiring::from_str(WIRINGS[8]).unwrap(),
        notches: vec![(b'M' - b'A') as isize, (b'Z' - b'A') as isize],
        position: position_index,
        ring_setting: ring_index,
      },
    }
  }

  fn encipher(&self, c: char, mapping: &[char]) -> char {
    let index = (c as u8 - b'A') as usize;
    let shift = (self.position - self.ring_setting).rem_euclid(26);

    let mapped_index = mapping[(index + shift as usize) % 26] as u8 - b'A';
    let final_index = (mapped_index as isize - shift).rem_euclid(26) as u8;

    (final_index + b'A') as char
  }

  pub fn forward(&self, input: char) -> char {
    self.encipher(input, &self.wiring.forward)
  }

  pub fn backward(&self, input: char) -> char {
    self.encipher(input, &self.wiring.backward)
  }

  pub fn at_notch(&self) -> bool {
    self.notches.contains(&self.position)
  }

  pub fn turnover(&mut self) {
    self.position = (self.position + 1) % 26;
  }
}

impl Default for Rotor {
  fn default() -> Self {
    Rotor::new(RotorModel::Identity, 'A', 1)
  }
}
