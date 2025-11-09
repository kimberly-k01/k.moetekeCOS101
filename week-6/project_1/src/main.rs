use std::io;

fn main() {
    println!("WELCOME TO K's DELICACIES");
    println!("\nPick what you'd like:");

    println!("CODE    MENU                             PRICE");
    println!("P       Pounded Yam/Edinkaiko Soup    -   ₦3200");
    println!("F       Fried Rice & Chicken          -   ₦3000");
    println!("A       Amala & Ewedu Soup            -   ₦2500");
    println!("E       Eba & Egusi Soup              -   ₦2000");
    println!("W       White Rice & Stew             -   ₦2500");

println!("\nEnter the code for the food you'd like:");
let mut code_1 = String::new();
io::stdin().read_line(&mut code_1).expect("Failed to read input");
let code_1 = code_1.trim().to_uppercase();

println!("How much would you like?");
let mut quantity = String::new();
io::stdin().read_line(&mut quantity).expect("Failed to read quantity");
let mut  quantity: f32 =  quantity.trim().parse().expect("Invalid quantity entered");
println!("Quantity: {}", quantity);

while quantity < 1.0 {
        println!("Invalid quantity. Try again:");
        let mut new_quantity = String::new();
        io::stdin().read_line(&mut new_quantity).expect("Failed to read quantity");
        let quant:f32 = new_quantity.trim().parse().expect("Invalid number entered");
    }

    println!("Quantity: {}", quantity);



let item:&str = match code_1.as_str() {
        "P" => "Pounded Yam/Edinkaiko Soup",
        "F" => "Fried Rice & Chicken",
        "A" => "Amala & Ewedu Soup",
        "E" => "Eba & Egusi Soup",
        "W" => "White Rice & Stew",
        _ => {
            println!("Invalid item code. Please try again.");
            return;
        }
    };

    let price:f32 = match code_1.as_str() {
        "P" => 3200.0,
        "F" => 3000.0,
        "A" => 2500.0,
        "E" => 2000.0,
        "W" => 2500.0,
        _ => {
            println!("Invalid item code. Please try again.");
            return;
        }
    };


    let total_cost = price * quantity ;
    println!("\nYour order: {}    {} X ₦{}. \nTotal cost: ₦{}.", item, quantity, price, total_cost);


    if total_cost > 10000.0 {
        let dis = total_cost * 0.05;
        let dist = total_cost - dis;
        println!("\nYou get a 5% discount!");
        println!("\nDiscounted price: ₦{}",dist);
    }else {
        println!("\nTotal cost: ₦{}", total_cost);
    }
 



}