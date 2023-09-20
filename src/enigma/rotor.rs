use super::common::{decode_wire_map, encode_char};
use super::enigma::RotorModel;

pub struct Rotor {
  wiring_forward: Vec<u8>,
  wiring_backward: Vec<u8>,
  notches: Vec<u8>,
  position: u8,
  ring_setting: u8
}

impl Rotor {
  pub fn new(model: RotorModel, position: char, ring_setting: u8) -> Self {
    let rotor_position = encode_char(position);

    let (wiring, turnovers) = match model {
      RotorModel::I => ("EKMFLGDQVZNTOWYHXUSPAIBRCJ", vec!['Q']),
      RotorModel::II => ("AJDKSIRUXBLHWTMCQGZNPYFVOE", vec!['E']),
      RotorModel::III => ("BDFHJLCPRTXVZNYEIWGAKMUSQO", vec!['V']),
      RotorModel::IV => ("ESOVPZJAYQUIRHXLNFTGKDCMWB", vec!['J']),
      RotorModel::V => ("VZBRGITYUPSDNHLXAWMJQOFECK", vec!['Z']),
      RotorModel::VI => ("JPGVOUMFYQBENHZRDKASXLICTW", vec!['M', 'Z']),
      RotorModel::VII => ("NZJHGRCXMYSWBOUFAIVLPEKQDT", vec!['M', 'Z']),
      RotorModel::VIII => ("FKQHTLXOCBJSPDZRAMEWNIUYGV", vec!['M', 'Z']),
      RotorModel::Identity => ("ABCDEFGHIJKLMNOPQRSTUVWXYZ", vec!['A'])
    };

    Self::from_settings(wiring, turnovers, rotor_position, ring_setting)
  }

  pub fn is_at_notch(&self) -> bool {
    self.notches.contains(&self.position)
  }

  pub fn turnover(&mut self) {
    self.position = (self.position + 1) % 26;
  }

  pub fn forward(&self, letter: u8) -> u8 {
    self.encipher(letter, &self.wiring_forward)
  }

  pub fn backward(&self, letter: u8) -> u8 {
    self.encipher(letter, &self.wiring_backward)
  }
}

impl Rotor {
  fn from_settings(wire_map: &str, notches: Vec<char>, position: u8, ring_setting: u8) -> Self {
    let mapping = decode_wire_map(wire_map);
    let backwards = invert_map(&mapping);

    Self {
      wiring_forward: mapping,
      wiring_backward: backwards,
      notches: notches.iter().map(|&c| encode_char(c)).collect::<Vec<_>>(),
      position,
      ring_setting
    }
  }

  fn encipher(&self, letter: u8, wiring: &[u8]) -> u8 {
    // Step forward to mimic rotation of rotor
    let shift = self.position as i8 - self.ring_setting as i8;

    // Step from letter index and wrap around if necessary
    let index = (letter as i8 + shift).rem_euclid(26);

    // The resulting letter
    let ciphered = wiring[index as usize];

    // Undo forward step, wrap around if necessary
    (ciphered as i8 - shift).rem_euclid(26) as u8
  }
}

fn invert_map(mapping: &Vec<u8>) -> Vec<u8> {
  // Possible to make declarative?

  let mut inverse = vec![0; mapping.len()];
  for (i, &forward_map) in mapping.iter().enumerate() {
    inverse[forward_map as usize] = i as u8;
  }

  inverse
}

impl Default for Rotor {
  fn default() -> Self {
    Self::new(RotorModel::Identity, 'A', 0)
  }
}
