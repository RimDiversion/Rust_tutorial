fn main() {
    /* 
        garbage collection (high level languages)
        manual memory management (low loevel languages)    
        ownership model (rust model) consider stack vs heap
    */

    fn a() {
        let _x: &str = "hello";  // stack and immutable
        let _y = 22;
        b();
    }

    fn b() {
        let _x: String = String::from("world");  // heap with variable size, pointer gets stored on stack to point to heap
    }

    /*
        1. Each value in rust has a variable that's called its owner
        2. There can only be one owner at a time
        3. When the owner goes out of scope, the value will be dropped
     */

    // copy for simply types
    let a: i32 = 3;
    let _b: bool = true;
    let _c: char = 'c';
    let _a2 = a; // copy

    // move for complex types
    let s1: String = String::from("string");
    let s2: String = s1;    // this is a move not a shallow copy
    let _s3: String = s2.clone();


    let s: String = String::from("hello");
    takes_ownership(s);
    // println!("{}", s);  this line would throw an error as value s has been moved to takes_ownership

    let x: i32 = 5;
    makes_copy(x);
    println!("{}", x);  // this works because integers are copied

    let give_string: String = gives_ownership();
    println!("given_string = {}", give_string);

    let h = String::from("howdy");
    let h = take_give_back(h);
    println!("{} again!", h);

    let len = calculate_length(&h);
    println!("The length of '{}' is {}.", h, len);

    //string slices
    let mut k: String = String::from("Hello world");
    let hello: &str = &k[..5];
    let world: &str = &k[6..];

    let word: &str = first_word(&k);
    println!("{}", word);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let another_string: String = String::from("string");
    another_string
}

fn take_give_back(a_string: String) -> String {
    println!("{}", a_string);
    a_string
}

// this takes a reference to a string instead of the string itself so owndership remains in main
// called borrowing and cannot modify the value 
// can also use mutable reference but only once per scope - this avoids race conditions
// can't have mutable reference if immutable reference exists
// can have multiple immutable refferences
// the data a reference points to must be valid and still exist in the stack
fn calculate_length(h: &String) -> usize {
    let length = h.len();
    length
}

fn first_word(k: &str) -> &str {
    let bytes = k.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &k[..i];
        }
    }

    &k[..]
}