use std::io;

fn get_input(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let value: f64 = input.trim().parse().expect("Invalid input. Please enter a number.");
    return value;
}

fn area_trapezium() {
    println!("\n--- Area of Trapezium ---");
    let h = get_input("Enter height:");
    let b1 = get_input("Enter base 1:");
    let b2 = get_input("Enter base 2:");
    
    let area = (h / 2.0) * (b1 + b2);
    println!("Result: Area of the trapezium is: {:.2}", area);
}

fn area_rhombus() {
    println!("\n--- Area of Rhombus ---");
    let d1 = get_input("Enter diagonal 1:");
    let d2 = get_input("Enter diagonal 2:");
    
    let area = 0.5 * d1 * d2;
    println!("Result: Area of the rhombus is: {:.2}", area);
}

fn area_parallelogram() {
    println!("\n--- Area of Parallelogram ---");
    let b = get_input("Enter base:");
    let al = get_input("Enter altitude:");
    
    let area = b * al;
    println!("Result: Area of the parallelogram is: {:.2}", area);
}

fn area_cube() {
    println!("\n--- Area of Cube ---");
    let l = get_input("Enter the length of one side:");
    let area = 6.0 * l * l;
    println!("Result: Surface area of the cube is: {:.2}", area);
}

fn volume_cylinder() {
    println!("\n--- Volume of Cylinder ---");
    let r = get_input("Enter radius:");
    let h = get_input("Enter height:");

    let volume = 3.142 * r.powf(2.0) * h;
    println!("Result: Volume of the cylinder is: {:.2}", volume);
}


fn main() {
    loop {
        println!("\n<========= MTH 101 Calculator =========>");
        println!("Select a problem to solve:");
        println!("  1. Area of Trapezium");
        println!("  2. Area of Rhombus");
        println!("  3. Area of Parallelogram");
        println!("  4. Area of Cube");
        println!("  5. Volume of Cylinder");
        println!("  0. Exit");
        println!("<========================================>");
        println!("Enter your choice (0-5): ");
        
        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("Failed to read line");
        
        let option: u32 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid choice! Please enter a number between 0 and 5.");
                continue;
            }
        };

        if option == 1 {
            area_trapezium();
        } else if option == 2 {
            area_rhombus();
        } else if option == 3 {
            area_parallelogram();
        } else if option == 4 {
            area_cube();
        } else if option == 5 {
            volume_cylinder();
        } else if option == 0 {
            println!("MTH 101 Calculator \nThanks for working with us");
            break;
        } 
    }
}