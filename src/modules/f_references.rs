pub fn main () {
  let string1 = String::from("Primer String");

  let (string, length) = calculate_length(&string1);
  println!("The length of '{}' is {}.", string, length);
  println!("{}", string1);
}

fn take_properties (string: String) {
    println!("string: {}", string);
}

fn calculate_length (string: &String) -> (&String, usize) {
  let length = string.len();
  (string, length)
}

fn mutable_references () {
  let mut s = String::from("hello");

  change(&mut s);
  println!("{}", s);
}

fn change (some_string: &mut String) {
  some_string.push_str(", world");
}