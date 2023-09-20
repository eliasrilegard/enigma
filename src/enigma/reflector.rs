use super::common::decode_wire_map;
use super::enigma::ReflectorModel;

pub struct Reflector {
  mapping: Vec<u8>
}

impl Reflector {
  pub fn reflect(&self, letter: u8) -> u8 {
    self.mapping[letter as usize]
  }

  pub fn new(model: ReflectorModel) -> Self {
    let mapping = match model {
      ReflectorModel::A => "ZYXWVUTSRQPONMLKJIHGFEDCBA",
      ReflectorModel::B => "YRUHQSLDPXNGOKMIEBFZCWVJAT",
      ReflectorModel::C => "FVPJIAOYEDRZXWGCTKUQSBNMHL"
    };

    Self {
      mapping: decode_wire_map(mapping)
    }
  }
}
