fn main() {
    println!("Hello World!");

    test_function(10);
    test_function_2('J', 19);
}

fn test_function(x: i32) {
    println!("The value of the parameter is: {x}");
}

fn test_function_2(initial: char, age: i32) {
    println!("Hi, my initial is {initial} and I'm {age} years old.");
}

fn five() -> i32 {
    5 //Expression. In Rust, a function returns the last expression or the value using "return"
}