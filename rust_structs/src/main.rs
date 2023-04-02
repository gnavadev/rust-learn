fn main() {

    struct RedFox {
        enemy: bool,
        life: u8,
    }

    // let fox = RedFox {
    //     enemy: true,
    //     life: 70,
    // };

    impl RedFox {
        fn new() -> Self {
            Self {
                enemy: true,
                life: 70,
            }
        }

    }

    let fox = RedFox::new();
    let life_left = fox.life;

}
