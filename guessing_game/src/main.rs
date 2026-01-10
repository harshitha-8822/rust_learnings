use std::io;
use rand::Rng; //rng random number generator
use std::cmp::Ordering;

//Rng is a trait that provides gen_range() method

const MIN: u32 =1;
const MAX: u32 = 100;
fn main(){
    let secret_number = rand::thread_rng().gen_range(MIN..=MAX);
    //generate a random number for this thread

    println!("Guess the number ({MIN} to {MAX})!");
    
    loop{
        println!("enter your guess");
       

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("cant read line");

        //parse to input
        let guess: u32 = match guess.trim().parse(){
            Ok(val) => val,
            Err(_) =>{
                println!("Invalid input, enter a valid number");
                continue;
            }
        };

        if guess<MIN || guess>MAX{
            println!("guess must be between {MIN} and {MAX}");
            continue;
        }

        //compare with secret number
        match guess.cmp(&secret_number){
            Ordering::Equal => {
                println!("guessed right!!!!!!!");
                break;
            },
            Ordering::Less => println!("Too low"),
            Ordering::Greater => println!("Too high"),
        }
    }

}