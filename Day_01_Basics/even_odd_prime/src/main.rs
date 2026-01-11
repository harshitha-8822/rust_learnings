use std::io;
//input --->integer, output: even/odd, output:prime or not
fn main(){
    println!("Enter a number : ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Cannot read");

    let num: i32 = match input.trim().parse(){
        Ok(val) => val,
        Err(_) => {
            println!("Invalid input, enter a number");
            return;
        }
    };

    //check even or odd
    if num%2==0{
        println!("The number {num} is even");
    }
    else{
        println!("The number {num} is odd");
    }

    //check prime or not
    if num<=1{
        println!("Not prime");
        return;
    }
    //0,1 and negative numbers are not prime

    let root_n = (num as f64).sqrt() as i32;
    for i in 2..=root_n{
        if num%i==0{
            println!("Not Prime, divisibe by {}",i);
            return;
        }
    }
    println!("Prime");
    
}