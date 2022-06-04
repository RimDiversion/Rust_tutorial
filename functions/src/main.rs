fn main() {
    test();
    add_numbers(4, 7);

    let number = {
        let x = 3;
        x + 1   // this gets returned, no ; or it won't get returned (expression vs statement)
    };
    println!("{}", number);

    let result = add_numbers_2(43, 57);
    println!("{}", result)
}

fn test() { // functions use snake case
    println!("Test has been called...");
}

fn add_numbers(x: i32, y: i32) {
    println!("The sum is: {}", x + y);
}

fn add_numbers_2(x: i32, y: i32) -> i32 {   // must declare type of value that gets returned with ->
    let res = x + y;
    if res > 10 {
        return res - 10;
    }
    res   // return x + y; would also work
}