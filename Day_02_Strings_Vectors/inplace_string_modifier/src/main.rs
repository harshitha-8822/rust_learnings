use std::io;

// 3 operations 
// convert to uppercase, 
// remove spaces, 
// add prefix RUST_                                                                                                                          

fn modify(s: &mut String) {

    *s = s.to_uppercase();
    s.retain(|c| c!=' ');
    s.insert_str(0,"RUST_");

}
fn main() {
    let mut sen = String::new();
    io::stdin().read_line(&mut sen).expect("cant read line");

    modify(&mut sen);

    println!("{sen}");

}
