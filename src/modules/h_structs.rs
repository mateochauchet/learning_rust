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

  let suarez = Player::create("Luis suarez".to_string(), 34, true);

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

struct SoccerPlayer {
  name: String,
  age: u8,
  position: String,
  team: String,
  goals_scored: u32,
}

impl SoccerPlayer {
  fn new(name: String, age: u8, position: String, team: String) -> SoccerPlayer {
    SoccerPlayer {
      name,
      age,
      position,
      team,
      goals_scored: 0
    }
  }

  fn score_goal(&mut self) {
    self.goals_scored += 1;
    println!("{} ha marcado! Ahora tiene {} goles.", self.name, self.goals_scored);
  }

  fn transfer(&mut self, new_team: String) {
    println!("{} was transferd to {}", self.name, new_team);
    self.team = new_team;
  }
}


fn main_2() {
  let mut player = SoccerPlayer::new(String::from("Lionel Messi"), 34, String::from("Delantero"), String::from("Paris Saint-Germain"));
  
  player.score_goal(); // Lionel Messi ha marcado! Ahora tiene 1 goles.
  player.score_goal(); // Lionel Messi ha marcado! Ahora tiene 2 goles.
  
  player.transfer(String::from("Barcelona")); // Lionel Messi ha sido transferido de Paris Saint-Germain a Barcelona.
}