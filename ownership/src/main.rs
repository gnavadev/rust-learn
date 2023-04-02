fn main() {
    println!("Hello, world!");
    //Each value has a owner
    //Only on owner per value
    //Value gets dropped if its owner goes out of scope

    let s1 = String::from("abc");
    let s2 = s1;
    //s1 value is not copied to s2, it is transfered.
    //s1 now has no value, and s2 has s1 value
    //this happens because rust doesnt let 2 pointers have the same heap value
    let s3 = s2.clone();
    //s3 now has the value of s2, but s2 mantain its value
    //this happens because rust creates another heap for this variable

}
