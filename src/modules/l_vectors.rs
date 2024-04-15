fn main() {

}


// los vectores son una coleccion de elementos del mismo tipo
// los vectores tienen un tamaño dinamico a diferencia de los arrays
// se pueden declarar de dos formas
// let vector = vec![1, 2, 3, 4, 5];
// let vector: Vec<i32> = Vec::new();
// se pueden agregar elementos con el metodo push
// se pueden eliimnar elementos con el metodo pop
// se pueden acceder a los elementos con el operador []
// se pueden acceder a los elementos con el metodo get

fn vector() {
    let v = vec![1, 2, 3, 4, 5];
    let mut v2: Vec<i32> = Vec::new();

    v2.push(1);
    v2.push(2);
    v2.push(3);
    println!("la longitud del vector es {}", v.len());

    v2.pop();

}

fn vector2() {
    let v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    let second = v.get(1);

    match second {
        Some(value) => println!("The second element is: {}", value),
        None => println!("There is no second element"),
    }

    if let Some(value) = second {
        println!("The second element is: {}", value);
    } else {
        println!("There is no second element");
    }

    println!("The first element is: {}", first);
    println!("The second element is: {:?}", second);
}


fn iterar_vector() {
    let v = vec![1, 2, 3, 4, 5];

    for i in &v {
        println!("{}", i);
    }

    for i in v.iter() {
        println!("{}", i);
    }

}

// Vector con elementos de diferentes tipos
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn vector_de_enum() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}