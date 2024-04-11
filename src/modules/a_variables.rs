
pub fn main () {
  let name = "John";

  let x: u32;
  x = 42;

  let mut my_variable = 10;

  println!("Hello, {}!", name);
  println!("The value of x is: {}", x);


  my_variable = 20;
  println!("The value of my_variable is: {}", my_variable);
}

fn constants () {
  // Constants are always immutable
  // They must be annotated with a type
  const PI : f32 = 3.14159;
  println!("The value of PI is: {}", PI);
}

fn shadowing () {
  let x = 5;
  let x = x + 1;
  let x = x * 2;
  println!("The value of x is: {}", x);
  // result is 12


  let a = 5;
  let a = "hola";
}

fn tuples () {
  let player: (&str, u8, bool) = ("Messi", 10, true);
  let (name, number, active) = player;
}