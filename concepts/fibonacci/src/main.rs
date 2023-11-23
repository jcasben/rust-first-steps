fn main() {
    println!("Fibonacci's Sequence!");

    let mut first: i64 = 1;
    let mut second: i64 = 1;

    loop {
        let aux: i64 = first + second;
        first = second;
        second = aux;
        println!("{first}");
    };
}
