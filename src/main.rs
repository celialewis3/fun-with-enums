mod bar;
mod talk;

use rand::Rng;

fn main() {
  //bar::simulate_bar(2);

  let room = talk::Room::new(10);

  room.talk();


}

/*

  0 - Generate random customers w/ random names
  1 - Simulate bar based on number of patrons
  2 - Make simulation more interesting
  3 - Add GUI

*/