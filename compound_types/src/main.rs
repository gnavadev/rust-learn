// Silence some warnings so they don't distract from the exercise.
#![allow(unused_variables)]

fn main() {
    println!("Hello, world!");
    let info: (u8, f64, i32) = (1, 3.3, 999); // Tuple, maximun arity is 12./
    let jets = info.0;
    let fuel = info.1;
    let ammo = info.2;
    let (jets, fuel, ammo) = info;
    //Arrays are limited to a size of 32, and can only store values of the same type
    //They live on the stack by default, and have fixed size, is better to use Vectors
    let lbuf = [1,2,3]; //Literally specified
    let nbuf = [0; 3]; // [value; how many]
    let buf: [u8; 3] = [1, 2 ,3]; //3 values of u8 types
}
