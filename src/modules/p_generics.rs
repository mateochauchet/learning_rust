struct Point<T> {
  x: T,
  y: T,
}



struct Point2<T, U> {
  x: T,
  y: U,
}

pub fn main() {
  let entero = Point { x: 5, y: 10 };
  let decimal = Point { x: 1.0, y: 4.0 };

  // will fail because the types are different
  //let fail = Point { x: 5, y: 4.0 };
  let entero_decimal = Point2 { x: 5, y: 4.0 };
}

