fn main() {
    let mut s1 = String::from("abc");
    do_stuff(&s1);
    //references default to imutable

    fn do_stuff(s: &String) { //&String is a reference to a string
        //after the end of the function scope,
        //the reference gets dropped
        //s1 value never moved, so we can use it.
    }

    do_mutable_stuff(&mut s1);
    fn do_mutable_stuff(s: &mut String) { //&String is a reference to a string
        s.insert_str(0, "Hi, ");
        *s = String::from("Replacement");
    }
}
