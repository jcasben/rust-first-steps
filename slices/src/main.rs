fn main() {
    let s = String::from("Hello World");
    let hello = &s[..5];    //"hello"
    let llo_wo = &s[2..9];   //"llo_wo"
    let world = &s[6..];    //"world"

    let n = [1, 2, 3, 4, 5];
    let n_slice = &n[1..4];     //[2, 3, 4]
}

//first word program
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    return s;
}
