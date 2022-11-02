use std::io::{self, Read};

fn main() {
    // floatting_point_types();
    // numerical_operations();
    // boolean_type();
    // char_type();
    // tuple_type();
    // array_type();
    example();
}

fn floatting_point_types() {
    let x = 2.0; //f64
    let y: f32 = 3.0; //f32
}

fn numerical_operations() {
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3;
    let remainder = 43 % 5;
}

fn boolean_type() {
    let t = true;
    let f: bool = false;
}

fn char_type() {
    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';
}

fn tuple_type() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
}

fn array_type() {
    let a = [1, 2, 3, 4, 5];

    let b: [i32; 5] = [1, 2, 3, 4, 5]; // explicits type and size

    let c = [3; 5]; //initialize an array of 5 elements allwith value 3

}

fn example() { // if you insert a index greater than 4 it will cause a panic.
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");
    
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
