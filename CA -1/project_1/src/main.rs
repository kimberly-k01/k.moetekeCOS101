use std::io;

fn main() {
    println!("\nStudent's Test scores");

    //input student name
    println!("\n Enter your name: ");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Not a valid string");

    //input score

    let mut score_1 = String::new();
    let mut score_2 = String::new();
    let mut score_3 = String::new();

    println!("\nWhat was your score?");
    io::stdin().read_line(&mut score_1).expect("Not a valid string");
    let score:f32 = score_1.trim().parse().expect("Not a valid float");

    io::stdin().read_line(&mut score_2).expect("Not a valid string");
    let score:f32 = score_2.trim().parse().expect("Not a valid float");

    io::stdin().read_line(&mut score_3).expect("Not a valid string");
    let score:f32 = score_3.trim().parse().expect("Not a valid float");

    let total_score:f32 = score + &score + &score;
    let average:f32 = total_score / 3.00;


    if average >= 70.00{
      println!("\nYour grade is: \nA");
    }
    else if average >= 60.00 && average <=69.00 {
        println!("\nYour grade is: \nB");
    }
    else if average >= 50.00 && average <= 59.00 {
        println!("\nYour grade is: \nC");
    }
    else if average >= 45.00 && average <= 49.00 {
        println!("\nYour grade is: \nD");
    }
    else if average >= 0.00 && average <= 44.00 {
        println!("\nYour grade is: \nF");
    }
    else if average <0.00 && average >100.00 {
        println!("There was an error in the submitted score!");
    }
    
    
}
