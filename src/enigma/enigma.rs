use super::common::encode_char;
use super::plugboard::Plugboard;
use super::reflector::Reflector;
use super::rotor::Rotor;

pub struct Enigma {
  plugboard: Plugboard,
  rotors: Vec<Rotor>, // [Left, Middle, Right], etc
  reflector: Reflector
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum RotorModel {
  Identity = 0,
  I = 1,
  II = 2,
  III = 3,
  IV = 4,
  V = 5,
  VI = 6,
  VII = 7,
  VIII = 8
}

#[allow(dead_code)]
pub enum ReflectorModel {
  A = 0,
  B = 1,
  C = 2
}

impl Enigma {
  pub fn new(
    plugboard_connections: &str,
    rotor_data: Vec<(RotorModel, char, u8)>,
    reflector_model: ReflectorModel
  ) -> Self {
    Self {
      plugboard: Plugboard::from_mapping(plugboard_connections),
      rotors: rotor_data
        .iter()
        .map(|settings| Rotor::new(settings.0, settings.1, settings.2))
        .collect::<Vec<_>>(),
      reflector: Reflector::new(reflector_model)
    }
  }

  /// Todo: Tweak to accept `String` as well
  pub fn prepare(text: &str) -> String {
    text
      .chars()
      .filter(|c| c.is_ascii_alphabetic())
      .map(|c| c.to_ascii_uppercase())
      .collect::<String>()
  }

  pub fn encrypt_string(&mut self, string: String) -> String {
    string
      .chars()
      .map(|letter| self.encrypt_letter(letter))
      .collect::<String>()
  }

  pub fn encrypt_letter(&mut self, letter: char) -> char {
    let encoded = encode_char(letter);
    (self.encrypt(encoded) + 65) as char
  }
}

impl Enigma {
  fn encrypt(&mut self, letter: u8) -> u8 {
    self.rotate_rotors();

    // Plugboard in
    let mut encrypted = self.plugboard.transform(letter);

    // Right to left
    for rotor in self.rotors.iter().rev() {
      encrypted = rotor.forward(encrypted);
    }

    // Reflect
    encrypted = self.reflector.reflect(encrypted);

    // Left to right
    for rotor in self.rotors.iter() {
      encrypted = rotor.backward(encrypted);
    }

    // Plugboard out
    self.plugboard.transform(encrypted)
  }

  fn rotate_rotors(&mut self) {
    // If middle rotor notch - double-stepping
    if self.rotors[1].is_at_notch() {
      self.rotors[0].turnover();
      self.rotors[1].turnover();
    }
    // If left-rotor notch
    else if self.rotors[2].is_at_notch() {
      self.rotors[1].turnover();
    }

    // Increment right-most rotor
    self.rotors[2].turnover();
  }
}
