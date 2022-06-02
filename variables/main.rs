fn main() {
    let x = 4;
    println!("x is: {}", x);

    {
        let x = x - 2;
        println!("x is: {}", x)
    }

    let x = x + 1;
    let mut y = "Dan is awesome";
    println!("x is: {}. Also: {}", x, y);
    y = "Another string";   // warning for unused assignment

    const SECONDS_IN_MINUTE: u32 = 60;
    println!("{}", SECONDS_IN_MINUTE);
    // const SECONDS_IN_MINUTES: u32 = 100; error - can't reasign a constant
}