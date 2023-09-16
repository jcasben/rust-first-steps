fn main() {
    //declaring a new empty vector.
    let _v: Vec<i32> = Vec::new();
    //creating a new Vec<i32> using the vec! macro.
    let _v = vec![1, 2, 3];
    
    //UPDATING A VECTOR
    let mut v = Vec::new();
    v.push(4);
    v.push(5);
    v.push(6);

    //READING ELEMENTS FROM A VECTOR
    let v = vec![1, 2, 3, 4, 5];
    let second: &i32 = &v[1];
    println!("Second: {second}");
    
    //this is a better way of doing it.
    let second: Option<&i32> = v.get(1);
    match second {
        Some(second) => println!("Second: {second}"),
        None => println!("There is no second element"),
    }

    //ITERATING OVER A VECTOR
    //immutable
    let v = vec![100, 37, 5];
    for i in &v {
        println!("{i}");
    }

    //mutable
    let mut v = vec![100, 37, 5];
    for i in &mut v {
        *i += 50; 
    }

    //We can use enums to store multiple types.
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        SpreadSheetCell::Int(2),
        SpreadSheetCell::Float(9.6),
        SpreadSheetCell::Text(String::from("blue")),
    ];
}

