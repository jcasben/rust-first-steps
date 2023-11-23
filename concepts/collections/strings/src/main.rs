fn main() {
    let _s = String::new();
    let _s = String::from("Hello World!");

    //We can grow strings
    let mut s = String::from("Hello, ");
    let s2 = "World!";
    //It doesn't take ownership of the string slice!
    s.push_str(s2);
    println!("s2 is {s2}");

    //We can append chars with push()
    let mut s = String::from("Hell");
    s.push('o');

    //We can also append string like this:
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let _s3 = s1 + &s2; //s1 has been moved and can't be used anymore

    //To concatenate Strings in a easier way, we can use the format! macro:
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let _s = format!("{s1}-{s2}-{s3}");

    //INDEXING INTO STRINGS
    let s = String::from("Hello");
    for c in s.chars() {
        println!("{c}");
    }
}