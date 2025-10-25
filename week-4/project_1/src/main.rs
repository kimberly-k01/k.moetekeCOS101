use std::io;

fn main() 
{
   println!("Quadratic equation format ax² + bx + c");
   let mut a = String::new();
   let mut b = String::new();
   let mut c = String::new();

   println!("Enter the value of a: ");
   io::stdin().read_line(&mut a).expect("Not a valid string");
   let a:f32 = a.trim().parse().expect("Not a valid number");

   println!("Enter the value of b: ");
   io::stdin().read_line(&mut b).expect("Not a valid string");
   let b:f32 = b.trim().parse().expect("Not a valid number");

   println!("Enter the value of c: ");
   io::stdin().read_line(&mut c).expect("Not a valid string");
   let c:f32 = c.trim().parse().expect("Not a valid number");

   // discriminant
   let discriminant = b.powf(2.0) - 4.0 * a * c;

   // nature of roots
   if discriminant > 0.0 {
    let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
    let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
    println!("Two distinct real roots:");
    println!("Root 1 = {}", root1);
    println!("Root 2 = {}", root2);
   
   } else if discriminant == 0.0 {
    let root = -b / (2.0 * a);
    println!("Exactly one real root:");
    println!("Root = {}", root);
   } else {
    println!("There aren't any real roots");
   }
}