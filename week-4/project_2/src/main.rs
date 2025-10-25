use std::io;

fn main() {
    println!("Say yes if the employee is experienced and no if not experienced");
    let mut experience_input = String::new();
    io::stdin().read_line(&mut experience_input).expect("Failed to read input");
    let experience_input = experience_input.trim();
    let experienced = experience_input == "yes";

    println!("What is your age?");
    let mut age = String::new();
    io::stdin()
    .read_line(&mut age)
    .expect("Failed to read input");
    let age:u32 = age.trim().parse().expect("Not a valid age");

    let incentive:u32;

    if experienced {
        if age >= 40 {
            incentive = 1_560_000;
        
        } else if age >=30 && age < 40 {
            incentive = 1_480_000;
        
         
         } else if age >=28 && age < 30 {
            incentive = 1_400_00;
         
         } else {
            incentive = 1_300_00;
         }
    } else {
        incentive = 100_000;
    }
    println!("The annual incentive of the employee is {}", incentive);
}

