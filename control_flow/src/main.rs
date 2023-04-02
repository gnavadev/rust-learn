fn main() {
    println!("Hello, world!");
    //No semicolon after value, can't use return, because return only applies to functions.
    //If on rust is an expression
    //all the blocks must return the same type
    //There is a semicolon after
    let num = 3;
    let msg = if num == 5 {
        "five"
    } else if num == 4 {
        "four"
    } else {
        "other"
    };
    // let test = if a { b } else { c };
    // let nested_test = if a {
    //     if x { y } else { y }
    // } else {
    //     c
    // };


    // 'bob: loop {
    //     loop {
    //         loop {
    //             break 'bob; Will break bob loop, if no name is given, it breakes the inner loop
    //             continue 'bob; will continue bob loop, if no name is given, it continues the inner loop
    //         }
    //     }
    // }

    // loop {
    //     //do stuff
    //     if !dizzy() { break } //Do while implementation on rust
    // }

    // for num in [7,8,9].iter() {
    //     //Do stuff with num
    // }

    // let array = [(1,2), (3,4)];
    // for (x, y) in array.iter() {
    //     //do stuff with x and y , for on tuples!
    // }

    // for num in 0..50 {
    //     //num is 0 to 49
    // }

    // for num in 0..=50 {
    //     //num is 0 to 50
    // }


}
