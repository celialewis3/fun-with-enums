mod bar;
mod talk;
mod generics;

use rand::Rng;

fn main() {
  //bar::simulate_bar(2);

  let room = talk::Room::new(10);

  //room.talk();

  println!("{:?}", generics::perry());
  println!("{:?}", generics::animal_computer());
  generics::computer_data();


}

/*

  0 - Generate random customers w/ random names
  1 - Simulate bar based on number of patrons
  2 - Make simulation more interesting
  3 - Add GUI

*/