use core::panic;
use std::error;
use std::fs::File;
use std::io::ErrorKind;

pub fn main() {
  error_handling();

}

fn error_handling() {
  // panic

  panic!("crash and burn");

}

fn recoverable_error() {
  // Result enum
  // enum Result<T, E> {
  //   Ok(T),
  //   Err(E),
  // }

  // unwrap
  // unwrap is a shortcut method that is implemented on Result types. If the Result is an Ok value, unwrap will return the value inside the Ok. If the Result is an Err value, unwrap will call the panic! macro for us.
  let f = File::open("hello.txt").unwrap();

  // expect
  // expect is similar to unwrap, but it allows us to choose the panic! error message. This can be helpful when the error message is specific to the situation.
  let f = File::open("hello.txt").expect("Failed to open hello.txt");

  // match
  let f = File::open("hello.txt");
  let f = match f {
    Ok(file) => file,
    Err(error) => panic!("Problem opening the file: {:?}", error),
  };
}

fn error_kind() {
  let f = File::open("hello.txt");

  // find if not create

  let f = match f {
    Ok(file) => file,
    Err(error) => match error.kind() {
        ErrorKind::NotFound => todo!(),
        other_error => panic!("Problem opening the file: {:?}", other_error),
    }
      
  };
}

// ? operator
// The ? operator can only be used in functions that have a return type of Result. If the value of the Result is an Ok, the value inside the Ok will get returned from the function. If the value is an Err, the Err will be returned from the whole function.

fn error_handling_with_question_mark() -> Result<String, std::io::Error> {
  let f = File::open("hello.txt")?;
  Ok("hello".to_string())
}

