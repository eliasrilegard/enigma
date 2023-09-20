mod enigma;

use enigma::enigma::{Enigma, ReflectorModel, RotorModel};

fn main() {
  let plain_text = "Amwwr, Bscwk!";
  println!("Plain text:\n{}", plain_text);

  let prepared = Enigma::prepare(plain_text);
  println!("\nPrepared text:\n{}", prepared);

  let mut enigma = Enigma::new(
    "AD CN ET FL GI JV KZ PU QY WX",
    vec![
      (RotorModel::I, 'Q', 15),
      (RotorModel::IV, 'W', 25),
      (RotorModel::III, 'E', 7),
    ],
    ReflectorModel::B
  );

  let encrypted = enigma.encrypt_string(prepared);
  println!("\nEncrypted text:\n{}", encrypted);
}
