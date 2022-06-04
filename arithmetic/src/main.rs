fn main() {
    let x: i32 = 12;
    let y = 10i32;

    let z = x + y;
    println!("{}", z);

    let q = 127_000 as i64;
    let r = 10_i32;
    
    let s = q / r as i64;
    println!("{}", s);

    let a = (i32::MAX as i64) + 1;
    let b = 10i32;

    let c = a as i32 / b;   // will cause overflow error but will not be caught
    println!("{}", c);

}
