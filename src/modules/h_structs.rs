struct Player {
  name: String,
  age: u32,
  is_active: bool,
  stats: u8,
}

impl Player {
  fn create(name: String, age: u32, is_active: bool) -> Player {
    Player {
      name,
      age,
      is_active,
      stats: 0,
    }
  }

  fn display_name(&self) {
    println!("Name: {}", self.name);
  }
}


fn main() {
  let messi = Player {
    name: "Messi".to_string(),
    age: 33,
    is_active: true,
    stats: 0,
  };

  let suarez = Player::create("Luis suarez".to_string(), 34, true)

  let messi_2012 = Player {
    age: 25,
    stats: 91,
    ..messi
  };

}

fn mutability() {
  let mut messi = Player {
    name: "Messi".to_string(),
    age: 33,
    is_active: true,
    stats: 0,
  };

  messi.age = 34;
}

fn tuple_structs() {
  struct Color(u8, u8, u8);
  let black = Color(0, 0, 0);
}

