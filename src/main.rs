mod enigma;

use std::str::FromStr as _;

use enigma::{Enigma, Plugboard, Reflector, ReflectorModel, Rotor, RotorModel};

fn main() {
  let plugboard = Plugboard::from_str("DN AC QB TU SE PX WO ZJ IV YK").unwrap();
  let rotors = vec![
    Rotor::new(RotorModel::VII, 'R', 3),
    Rotor::new(RotorModel::I, 'B', 9),
    Rotor::new(RotorModel::VI, 'T', 5),
  ];
  let reflector = Reflector::from_model(ReflectorModel::B);

  let mut enigma = Enigma::new(plugboard, rotors, reflector);

  let input = "DUWUDUHRIKQERRQQQYACTZACIOVXJPCLYKLJTANIRIWKQOIXWKUCAHIKMIQWHEZWGVRXWNBGLAWCECYXYNXUZFOPYIBGHVCQYHPFFDCYIYWPCPPNZWJPVTUCNKQEOWXLWMZOJDUDILLWJOAQQIVHPASBKFGWPUQMWMNKOVMLFLMYKSHHPZXWXPZFYMOJGVWHRTFEKZOXNFQTQNSHVMFKRHQFTXORZUOATWFTKUIPUFEUAQBPQLXNWZNPLURGSALMWTKKHBAXCNNAXJCDFQDXTXIRIOLBUYTNFKZNGSYTHQNZNEJHITLXRCNVFZTUOPSXSQHIGYTZQXKXDDJJFDPTTJWSXFHJBYNLUWRLXUUCTPLHVZAFEYBRURKKEZLFJEWXLEARYZTZOSEJZIBMSVHQBYYZDPFGNSWFCIILBBWYDIIVMXDRWDFFGQUOGJHNJJXNQZRPWKSHPDWPGMMUHJSUUBDMUZVSRMCJWQUESLVTCUKVWNCPIVVWGBRULZVFOHJULKRUVALLBMSERSNYVTGTDAIFZCCYPWHAEGIKBLHTIFYHACOWXMSEDAOPDPSVLEDSYLPORENR";
  let ciphertext = enigma.encrypt(input);

  println!("{}", ciphertext);
}
