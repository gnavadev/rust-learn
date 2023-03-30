fn main() {
    const STARTING_MISSILES: i32 = 8;
    const READY: i32 = 2;
    let (missiles, ready) = (STARTING_MISSILES, READY);
    println!("Firing {} of my {} missiles...", ready, missiles);
    println!("{} missiles left", missiles - ready);
}
