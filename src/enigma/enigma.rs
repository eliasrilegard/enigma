use super::{Plugboard, Reflector, Rotor};

pub struct Enigma {
  pub plugboard: Plugboard,
  pub rotors: Vec<Rotor>,
  pub reflector: Reflector,
}

impl Enigma {
  pub fn new(plugboard: Plugboard, rotors: Vec<Rotor>, reflector: Reflector) -> Self {
    Self {
      plugboard,
      rotors,
      reflector,
    }
  }

  pub fn encrypt(&mut self, input: &str) -> String {
    input.chars().map(|c| self.encrypt_char(c)).collect()
  }

  fn encrypt_char(&mut self, input: char) -> char {
    self.rotate();

    // Plugboard in
    let mut signal = self.plugboard.swap(input);

    // Right to left
    for rotor in self.rotors.iter().rev() {
      signal = rotor.forward(signal);
    }

    // Reflector
    signal = self.reflector.reflect(signal);

    // Left to right
    for rotor in &self.rotors {
      signal = rotor.backward(signal);
    }

    // Plugboard out
    self.plugboard.swap(signal)
  }

  fn rotate(&mut self) {
    // 3-rotor implementation, double-stepping included
    // assert_eq!(self.rotors.len(), 3);

    // Double-stepping occurs when middle rotor hits notch
    if self.rotors[1].at_notch() {
      self.rotors[1].turnover();
      self.rotors[0].turnover();
    }
    // Normal behavior when right rotor hits notch
    else if self.rotors[2].at_notch() {
      self.rotors[1].turnover();
    }

    // Increment right-most rotor
    self.rotors[2].turnover();
  }
}
