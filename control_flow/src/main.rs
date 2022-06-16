fn main() {
    // if/else
    let x = 5;

    if x < 10 {
        println!("fist condition is true");
    } else if x > 10 {
        println!("second condition is true");
    } else {
        println!("no condition was true");
    }


    // interate
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }


    // range
    for number in 1..4 {    // last number is exclusing ie: 1-3
        println!("{}", number);
    }


    // loop
    loop {
        println!("repeat forever until break");
        break;
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 5 {
            break counter;  // adding counter after break returns counter
        }
    };
    println!("the counter is {}", result);


    // while
    let mut y = 3;

    while y != 0 {
        println!("{}!", y);
        y -= 1;
    }
    println!("Liftoff!");
}
