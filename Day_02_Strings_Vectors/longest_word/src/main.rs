use std::io::{self, Write};

fn longest_word(s: &str) -> &str{
    let mut longest = "";
    for word in s.split_whitespace(){
        if word.len()>longest.len(){
            longest = word;
        }
    }
    longest
}

fn main(){

    print!("Type the sentence: ");
    io::stdout().flush().unwrap();

    let mut sentence = String::new();
    io::stdin().read_line(&mut sentence).expect("Cannot read the line");

    let word = longest_word(&sentence);

    if word.len()>0{
        println!("the longest word is {word}");
    }
    else{
        println!("No longest word found(Empty string)");
    }
}