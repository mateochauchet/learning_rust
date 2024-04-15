
pub fn main() {

}

fn string() {
  let mut s = String::new();
  let s2 = "Hola ".to_string();
  let s3 = String::from("Mundo");

  let h = "messi";

  let s4 = h.to_string();
}

fn string_concatenation() {
  let s1 = String::from("Hola ");
  let s2 = String::from("Mundo");

  // option 1
  let s3 = s1 + &s2;
  println!("{}", s3);

  // option 2
  //let s4 = format!("{} {}", s1, s2);

  // option 3
  let mut s5 = String::from("Hola ");
  s5.push_str("Mundo");

}

fn string_indexing() {
  let s1 = String::from("Hola Mundo");
  let h = &s1[0..4];
  let m = &s1[5..10];
}


