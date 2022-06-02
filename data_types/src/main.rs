fn main() {
    println!("Hello, world!");

    let x: i32 = -2;    // signed
    let y: i8 = 1;

    let z: u32 = 768;   // unsigned

    let fl: f32 = 95.486;
    let fl2: f64 = 452.2;

    let bl: bool = false;
    let bl2: bool = true;

    let letter: char = 'p';

    let tup: (i32, bool, char) = (1, true, 's');
    let tup2: (&str, u32, i32) = ("Hello", 45, -76);
    println!("{} agent {}", tup2.0, tup2.1);

    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}", arr[arr.len() -1]);
}
