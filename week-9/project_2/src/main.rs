use std::fs::File;
use std::io::{self, Write};

// --- 1. Define the Data Structure ---
// A struct to hold the data for a single student.
struct Student {
    name: String,
    matric_number: String,
    department: String,
    level: u32,
}

// Implement a method to convert the student data into a formatted string
// suitable for displaying or saving to a file.
impl Student {
    fn to_string_display(&self) -> String {
        format!(
            "| {:<20} | {:<15} | {:<15} | {:<5} |",
            self.name, self.matric_number, self.department, self.level
        )
    }

    // A method to format the data for a CSV-like file (comma-separated values)
    fn to_string_csv(&self) -> String {
        format!(
            "{},{},{},{}\n",
            self.name, self.matric_number, self.department, self.level
        )
    }
}

fn main() -> io::Result<()> {
    println!("--- PAU-SMIS Data Processing ---");

    // --- 2. Define the Data (The array/vector) ---
    // Create a vector of Student structs using the data provided in the image.
    let students: Vec<Student> = vec![
        Student {
            name: "Oluchi Mordi".to_string(),
            matric_number: "ACC10231111".to_string(),
            department: "Accounting".to_string(),
            level: 300,
        },
        Student {
            name: "Adams Aliyu".to_string(),
            matric_number: "ECO10110101".to_string(),
            department: "Economics".to_string(),
            level: 100,
        },
        Student {
            name: "Shanla Bolade".to_string(),
            matric_number: "CSC10328828".to_string(),
            department: "Computer".to_string(),
            level: 200,
        },
        Student {
            name: "Adekunle Gold".to_string(),
            matric_number: "EEE1020202".to_string(),
            department: "Electrical".to_string(),
            level: 200,
        },
        Student {
            name: "Blanca Edemoh".to_string(),
            matric_number: "MEE10202001".to_string(),
            department: "Mechanical".to_string(),
            level: 100,
        },
    ];

    // --- 3. Display the Details to the Console ---
    println!("\n## Student Details (Console Display) ##");
    
    // Print the header
    let header = format!(
        "| {:<20} | {:<15} | {:<15} | {:<5} |",
        "Student Name", "Matric Number", "Department", "Level"
    );
    println!("{}", "-".repeat(header.len()));
    println!("{}", header);
    println!("{}", "-".repeat(header.len()));

    // Print the student data
    for student in &students {
        println!("{}", student.to_string_display());
    }
    println!("{}", "-".repeat(header.len()));


    // --- 4. Save the Details to a File (CSV Format) ---
    let filename = "pau_smis_data.csv";
    // Create or truncate the file
    let mut file = File::create(filename)?; 
    
    println!("\nSaving data to {}...", filename);

    // Write the CSV header to the file
    let file_header = "Student Name,Matric Number,Department,Level\n";
    file.write_all(file_header.as_bytes())?;

    // Write the student data to the file
    for student in &students {
        file.write_all(student.to_string_csv().as_bytes())?;
    }

    println!("Saved {} records to {}.", students.len(), filename);

    // The `main` function returns `io::Result<()>`, so we must return `Ok(())` 
    // to indicate success.
    Ok(())
}