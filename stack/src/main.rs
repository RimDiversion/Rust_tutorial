fn main() {
    let x = 2;
    let y = x;
    let string = Sting::from("hello");  // stored on heap

    example();
    let z = add(x, y);
    println!("{}", z)
}

fn example() {
    let _a = "true";
    let _b = false;
}

fn add(x: i32,  y: i32) -> i32 {
    x + y
}
