fn main() {
    //Mutability
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    //Constants
    const _Y:i32 = 10;

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
    let _v: i8 = -8;
    let _v: u8 = 0b0000_1000;

    let _v: i16 = -16;
    let _v: u16 = 0x10;

    let _v: i32 = -32;
    let _v: u32 = 32;

    let _v: i64 = -64;
    let _v: u64 = 64;

    let _v: isize = -64;
    let _v: usize = 64;

    //Floating point
    let _v: f32 = 2.0;
    let _v: f64 = 12.0;

    //Numeric Operations
    let _sum = 5 + 1;
    let _subs = 95.5 - 4.4;
    let _mult = 10 * 10;
    let _div = 56.3 / 32.3;
    let _remainder = 44 % 5;

    //Boolean
    let _boolean: bool = true;

    //Character
    let _c = 'z';
    let _z: char = 'Z';

    /*
     * Compound types
     * - Tuples
     * - Array
     */

    //Tuples
    let tup: (i32, char, f32) = (10, 'c', 6.5);
    let _c = tup.1; //'c'
    let (_x, y, _z) = tup;

    println!("The value of y is: {y}");

    let () = (); //unit

    //Array
    let a = [1, 2, 3, 4, 5];
    let _b: [i32; 4];
    let _f = [3; 6]; //equals to let f = [3, 3, 3, 3, 3, 3];

    let _first = a[1];
    let _last = a[a.len() - 1];
}
