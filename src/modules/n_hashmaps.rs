fn main() {


}

fn hashmaps() {

  // option 1
  use std::collections::HashMap;

  let mut scores = HashMap::new();

  scores.insert(String::from("messi"), 100);
  scores.insert(String::from("ronaldo"), 90);

  // option 2
  let players = vec![String::from("messi"), String::from("ronaldo")];
  let initial_scores = vec![100, 90];

  let mut scores: HashMap<_, _> = players.into_iter().zip(initial_scores.into_iter()).collect();


  // access
  let messi_score = scores.get("messi");

  // update
  scores.insert("messi".to_string(), 110);

  // insert if not exists
  scores.entry("messi".to_string()).or_insert(120);

  // iterate
  for (key, value) in &scores {
    println!("{}: {}", key, value);
  }

}