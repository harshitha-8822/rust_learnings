// two input helpers
use std::io::{self, Write};


//  show a prompt
//  take input from user
//  remove extra spaces/newline
//  if empty input → keep asking
//  return valid string

pub fn read_line(prompt: &str) -> String{
    loop{
        print!("{}",prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();

        if io::stdin().read_line(&mut input ).is_err(){
            println!("failed to read input, try again");
            continue;
        }

        let input = input.trim().to_string();

        // trim give &str

        if input.is_empty(){
            println!("input cannot be empty");
            continue;
        }

        return input;
    }

}
// This function keeps asking input until user enters a non-empty valid string, then returns it.

pub fn read_u32(prompt: &str) -> u32{
    loop{
        let ip = read_line(prompt);

        match ip.parse::<u32>(){
            Ok(num) => return num,
            Err(_) => {
                println!("enter valid number")
            }
        }
    }
}