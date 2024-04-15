

fn main() {

  let maximo_configurado = Some(7u8);

  // With if let we can do the same as with match but in a more concise way
  // with match
  match maximo_configurado {
    Some(value) => println!("El valor máximo es: {}", value),
    _ => (), 
  }

  // with if let
  if let Some(value) = maximo_configurado {
    println!("El valor máximo es: {}", value)
  }
}