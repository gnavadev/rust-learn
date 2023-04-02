fn main() {
    println!("Hello, world!");

    struct RedFox {
        enemy: bool,
        life: u32,
    }

    trait Noisy {
        fn get_noise(&self) -> &str;
    }

    impl Noisy for RedFox {
        fn get_noise(&self) -> &str {
            "Meow? "
        }
    }

    fn print_noise<T: Noisy>(item: T) {
        println!("{}", item.get_noise())
    }
}
