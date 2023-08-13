fn main() {
    let config_max = Some(3);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    } else {
        println!("It's None!"); 
    }
}
