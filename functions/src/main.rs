fn main() {
    println!("Hello, world!");
    let n1: f64 = 10.0;
    let n2: f64 = 20.0;
    println!("value is {}", do_stuff(n1, n2))
}

fn do_stuff(qty: f64, oz: f64) -> f64 {
        return qty * oz;
    }