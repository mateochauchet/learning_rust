fn main () {
  let myArray: [u8; 5] = [1, 2, 3, 4, 5];

  let first = myArray[0];

  println!("The first element is: {}", first);

  // Declare array, initialize all values to 3, length = 5
  let bytes = [3; 5]; // same as let bytes = [3, 3, 3, 3, 3];
}


// los arrays son una coleccion de elementos del mismo tipo
// los arrays tienen un tamaño fijo a diferencia de los vectores
// se pueden declarar de dos formas
// let array = [1, 2, 3, 4, 5];
// let array: [i32; 5] = [1, 2, 3, 4, 5];   