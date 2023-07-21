fn main() {
    let num = 4;

    if num < 5 {
        println!("The condition is true!");
    } else {
        println!("The condition is false!");
    }

    let number = 6;
    if number % 4 == 0 {
        println!("divisible by 4");
    } else if number % 3{
        println!("divisible by 3");
    } else if number % 2 {
        println!("divisible by 2");
    } else {
        println!("not divisible by 4, 3 or 2");
    }

    let condition = true;
    let n = if condition { 3 } else { 5 };
}
