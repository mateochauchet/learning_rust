fn main() {
  let myTuple: (u8, u8, u8) = (1, 2, 3);

  let (x, y, z) = myTuple;
}

fn tuples () {
  let player: (&str, u8, bool) = ("Messi", 10, true);
  let (name, number, active) = player;
}