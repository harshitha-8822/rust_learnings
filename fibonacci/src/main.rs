use std::io;
fn main(){

    println!("Enter the number of terms ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Cant read line");

    let mut n : u32 = match input.trim().parse(){
        Ok(val)=>val,
        Err(_) => {
            println!("Invalid input, enter valid number");
            return;
        } 
    };

    if n==0{
        return;
    }

    let mut f0: u64 = 0;
    let mut f1: u64 = 1;

    //print first term
     print!("{}",f0);
    if n==1{
        return;
    }

    //print second term
     print!(" {f1}");

    
    for _ in 3..=n{
        //overflow check here while adding
        let f2 = match f0.checked_add(f1){
            Some(val) => val,
            None => {
                println!("Overflow occured!");
                return;
            }
        };
        print!(" {f2}");
        f0 = f1;
        f1 = f2;
    }
    println!();

}