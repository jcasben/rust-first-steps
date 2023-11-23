fn main() {
    loop {
        //press Ctrl + C to stop
        println!("again!");
    }

    //using loops inside let
    let mut counter = 0;
    let result = loop {
        counter +=1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    //putting labels to loops
    let mut count = 0;
    'counting_up: loop {
        println!("count: {count}");
        let mut remaining = 10;

        loop {
            println!("remaining: {remaining}");
            if remaining == 10 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("final count: {count}");

    //while loop
    let mut m = 3;
    while m != 0 {
        println!("{m}");
        m -= 1;
    }
    println!("end!");

    //for loop
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("end!");
}
