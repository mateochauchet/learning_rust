pub fn main () {
  // Functions are named in snake_case
  // The return type must be specified


  say_my_name("Heisenberg");

}


fn say_my_name (name: &str) {
  println!("Your name is: {}", name);
}

fn divide_by_5(num: u32) -> u32 {
  if num == 0 {
    // Return early
    return 0;
  }
  num / 5
}

fn expression() {
  let x = 5;
  let y = {
    let x = 3;
    x + 1
  };
  println!("The value of y is: {}", y);
  // result is 4
}