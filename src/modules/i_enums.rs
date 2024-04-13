
#[derive(Debug, PartialEq)]
enum Door_state {
  Open,
  Closed
}

struct Door {
  height: u32,
  state: Door_state
}

fn main() {
  let door = Door_state::Open;
  println!("Estado inicial de la puerta: {:?}", door);

  let door = change_door(door);
  println!("Estado después de cambiar la puerta: {:?}", door);

  let my_door = Door{
    height: 18,
    state: Door_state::Open
  };
}

fn change_door(state: Door_state) -> Door_state {
  if state == Door_state::Closed {
    Door_state::Open
  } else {
    Door_state::Closed
  }
}

enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32)
}

impl Message {
  fn llamar(&self) {

  }
}

fn enum_with_values() {

}


// Rust no permite trabajar con valores null
// existe este enum para lograr añgp similar
// esto esta en la libreria estandar

// None, Ninguni
// Some, Algo
enum Option<T> {
  None,
  Some(T),
}
