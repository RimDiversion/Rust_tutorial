fn main() {
    // < > <= >= != ==;
    let cond = 2 < 3;
    println!("1:{}", cond);

    let cond1 = (2 as f32) > 2.2;
    println!("2:{}", cond1);

    //  ! - not, && - and, || - or  <- order of operations 
    let cond2 = cond && cond1;
    let cond3 = cond || cond1;
    println!("3:{} {}", cond2, cond3);
    
    let cond4 = !true;
    println!("4:{}", cond4);

    let food = "cookie";
    if food == "cookie" {
        println!("food is a cookie");
    } else if food == "salad" {
        println!("food is not a salad"); 
    } else {
        println!("food is not a cookie or salad");
    }
}
