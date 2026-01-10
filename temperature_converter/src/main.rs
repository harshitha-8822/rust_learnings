use std::io;
fn main() {
    // convert celcuis to farenheit

    println!("Enter temperature in Celsius: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let celsius: f64 = match input.trim().parse() {
        Ok(val) => val,
        Err(_) => {
            println!("Invalid input, please enter a number");
            return;
        }
    };
    //in order to handle alphabet inputs, or anything apart from number

    let fahrenheit: f64 = (celsius * 9.0 / 5.0) + 32.0;

    println!("{:.2}C = {:.2}F", celsius, fahrenheit);
    // print two digits after decimal
}
