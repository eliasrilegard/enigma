pub fn encode_char(letter: char) -> u8 {
  letter as u8 - 65
}

pub fn decode_wire_map(wire_map: &str) -> Vec<u8> {
  wire_map.chars().map(encode_char).collect::<Vec<_>>()
}
