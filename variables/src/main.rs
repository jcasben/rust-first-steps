fn main() {
    //Mutability
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    //Constants
    const Y:i32 = 10;

    //Shadowing
    let z = 1;
    let z = z + 1;

    {
        let z = z * 2;
        println!("The value of z in the inner scope is: {z}");
    }

    println!("The value of z is: {z}");

    // ------------- DATA TYPES -------------

    /*
     * Scalar types:
     * - Integers
     * - Floating-point numbers
     * - Booleans
     * - Characters
     */

    //Integers
    let v: i8 = -8;
    let v: u8 = 0b0000_1000;

    let v: i16 = -16;
    let v: u16 = 0x10;

    let v: i32 = -32;
    let v: u32 = 32;

    let v: i64 = -64;
    let v: u64 = 64;

    let v: isize = -64;
    let v: usize = 64;

    //Floating point
    let v: f32 = 2.0;
    let v: f64 = 12.0;

    //Numeric Operations
    let sum = 5 + 1;
    let subs = 95.5 - 4.4;
    let mult = 10 * 10;
    let div = 56.3 / 32.3;
    let remainder = 44 % 5;

    //Boolean
    let boolean: bool = true;

    //Character
    let c = 'z';
    let z: char = 'Z';

    /*
     * Compound types
     * - Tuples
     * - Array
     */

    //Tuples
    let tup: (i32, char, f32) = (10, 'c', 6.5);
    let c = tup.1; //'c'
    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let () = (); //unit

    //Array
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 4];
    let f = [3; 6]; //equals to let f = [3, 3, 3, 3, 3, 3];

    let first = a[1];
    let last = a[a.leng() - 1];
}
