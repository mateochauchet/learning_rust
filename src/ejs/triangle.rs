// Area rectangulo solucion 1

fn main() {
  //let area = calculate_area(30, 50);
  //println!("The area is: {}", area);

  let my_rectangle = Rectangle{
    height: 10,
    width: 10
  };

  let area = my_rectangle.calculate_area();
  println!("{}", area)
}

fn calculate_area(width: u32, height: u32) -> u32 {
  width * height
}


// Area rectangulo solucion 2

struct Dimensions(u32, u32);

fn calculate_area_2(dimensions: Dimensions) -> u32 {
  let Dimensions(width, height) = dimensions;
  width * height
  //dimensions.0 * dimensions.1
}


struct Rectangle {
  width: u32,
  height: u32,
}

fn calculate_area_3(rectangle: Rectangle) -> u32 {
  rectangle.height * rectangle.width
}


impl Rectangle {
  fn calculate_area(&self) -> u32 {
    self.height * self.width
  }
}

fn calculate_area_4(rectangle: Rectangle) -> u32 {
  rectangle.calculate_area()
}