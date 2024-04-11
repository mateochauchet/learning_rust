pub fn main() {
  let mut s = String::from("hello world");

  let (size, word) = first_word_bytes(&s);
  println!("First word size: {}, {}", size, word);
}


fn first_word_bytes(string: &String) -> (usize, &String) {
  let bytes = string.trim().as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    println!("{}: {}", i, item);
    if item == b' ' {
      return (i, string);
    }
  }

  (string.len(), string)
}

fn first_word (text: &String) -> &str {
  let bytes = text.trim().as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    println!("{}: {}", i, item);
    if item == b' ' {
      let word = &text[..i];
      return word;
    }
  }
  text
}

fn slices() {
  let a = [1, 2, 3, 4, 5];
  let slice = &a[1..3];

}