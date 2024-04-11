pub fn main () {
  // let number = 6;
  // if_expression(number);

  for_expression();
}

fn if_expression (number: u8) {
  if number < 5 {
    println!("The number is less than 5");
  } else {
    println!("The number is greater than or equal to 5");
  }

  let valid = number < 5;
  if valid {
    println!("yes");
  } else {
    println!("No");
  }
}

fn conditional_expression (number: u8) {
  let result = if number < 5 {
    "The number is less than 5"
  } else {
    "The number is greater than or equal to 5"
  };
  println!("Result: {}", result);
}

fn loop_expression () {
  let mut counter = 0;
  let result = loop {
    counter += 1;
    if counter == 10 {
      break counter;
    }
  };
  println!("Result: {}", result);
}

fn while_expression () {
  let mut counter = 0;
  while counter < 10 {
    counter += 1;
    println!("Counter: {}", counter);
  }
  println!("Result: {}", counter);
}

fn for_expression () {
  let array = [10, 20, 30, 40, 50];
  for element in array.iter() {
    println!("Element: {}", element);
  }

  // a..b valores de a inclusive hasta b exclusive osea de 1, 2, 3
  //para que sea inclusive a..=b
  // .rev() para que sea inverso
  for number in (1..4) {
    println!("Number: {}", number);
  }

  array.iter().for_each(|element| {
    println!("Element: {}", element);
  });
}