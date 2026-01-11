use std::io;

fn read_i32() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    //if there is any error then crash the program --unwrap
    input.trim().parse().unwrap()
    //unwrap if there is no error,  returns the value in Ok(..)
}
fn main() {
    println!("Enter num1 : ");
    let num1 = read_i32();
    println!("Enter num2 : ");
    let num2 = read_i32();

    println!("Enter the operator : ");

    let mut operator = String::new();
    io::stdin().read_line(&mut operator).unwrap();
    let op = operator.trim();

    // match that returns a value
    // let res: Option<i32> = match op{
    //     "+" => Some(num1+num2),
    //     "-" => Some(num1-num2),
    //     "*" => Some(num1*num2),
    //     "/" => {
    //         if num2==0 {
    //             None
    //         }
    //         else{
    //             Some(num1/num2)
    //         }
    //     },
    //     _ => None,
    // };

    // match res{
    //     Some(val) => println!("the result is {}", val),
    //     None => println!("Invalid operation(maybe divided by 0 OR wrong operator)")
    // }

    // to get proper error messages, instead of mixing two errors into None

    let res: Result<i32, String> = match op {
        "+" => Ok(num1 + num2),
        "-" => Ok(num1 - num2),
        "*" => Ok(num1 * num2),
        "/" => {
            if num2 == 0 {
                Err("Cannot divide by zero".to_string())
            } else {
                Ok(num1 / num2)
            }
        }
        _ => Err("Invalid operator".to_string()),
    };

    match res {
        Ok(val) => println!("the result is {}", val),
        Err(msg) => println!("{}", msg),
    }
}
