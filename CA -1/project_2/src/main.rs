use std::io;

fn main() {
    println!("\nEnter Principal: ");
    let mut principal = String::new();
    io::stdin().read_line(&mut principal).expect("Not a valid string");

    println!("\nEnter Rate: ");
    let mut rate = String::new();
    io::stdin().read_line(&mut rate).expect("Not a valid string");

    println!("\nEnter Time: ");
    let mut time = String::new();
    io::stdin().read_line(&mut time).expect("Not a valid string");


    //Solve for compound interest
    let a = principal * (1 + rate / 100);
    println!("Amount is: ", a);
}
