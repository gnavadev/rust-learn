fn main() {
    println!("Hello, world!");
    let msg = "ab".to_string();
    let msg2 = "ab".to_string();
    let msg3 = String::from("ab");
    let msg4: &str = "lol"; //Data in borrowed string slice cannot be modified
    //methods
    //word.bytes() can be used for index, since bytes are fixed size
    //word.chars() retrieve a iterator that you can use to iterate trhough the Unicode
    //unicode-segmentation package provides various functions for it
    //Like most indexing operations, the count starts from zero, so nth(0) returns the first value, nth(1) the second, and so on.
    //nth() will return None if n is greater than or equal to the length of the iterator.
}
