use std::io::Write;

fn main() {

    let category = "  Lager         Stout          Non-Alcoholic";
    let drinks =   "\n33 Export     Legend         Maltina
                    \nDesperados    Turbo King     Amstel Malta
                    \nGoldberg      Williams       Malta Gold
                    \nGulder                       Fayrouz
                    \nHeineken                     
                    \nStar";

                    
    let mut file = std::fs::File::create("Drinks.txt").expect("create failed");
    file.write_all(category.to_string().as_bytes()).expect("Write failed");
    file.write_all(drinks.to_string().as_bytes()).expect("write failed");
    println!("Data written to file");
}
