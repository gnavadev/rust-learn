use module_system::greet;
// use rand::thread_rng;

fn main() {
    println!("Hello, world!");
    module_system::greet(); // lib functions are private by default, adding pub before fn, makes it public
    greet(); //Works with use module_system::greet;
    // let x = thread_rng()
    // print!("{}", x)
}
