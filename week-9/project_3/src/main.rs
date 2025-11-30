struct ConvictedMinister {
    s_n: u32, // Serial number from the original datasets
    name: String,
    ministry: String,
    geopolitical_zone: String,
}

// Function to merge the separate datasets
fn merge_datasets(
    names: &Vec<(u32, &str)>,
    ministries: &Vec<(u32, &str)>,
    zones: &Vec<(u32, &str)>,
) -> Vec<ConvictedMinister> {
    
    let mut merged_data: Vec<ConvictedMinister> = Vec::new();

    
    for (s_n, name) in names {

       
        let index = (*s_n - 1) as usize;

        // Check if the index is valid for all vectors (which it should be based on the input)
        if index < ministries.len() && index < zones.len() {
            // Extract the ministry and zone value (the second element of the tuple)
            let ministry = ministries[index].1;
            let zone = zones[index].1;

            // Create a new ConvictedMinister struct and push it to the merged vector
            merged_data.push(ConvictedMinister {
                s_n: *s_n,
                name: name.to_string(), // Convert &str to String for the struct
                ministry: ministry.to_string(),
                geopolitical_zone: zone.to_string(),
            });
        } else {
            // Handle error or missing data (optional, but good practice)
            eprintln!("Warning: Data missing for S/N {}", s_n);
        }
    }

    merged_data
}

fn main() {
    
    // Dataset 1: Name of Commissioner
    let names_dataset: Vec<(u32, &str)> = vec![
        (1, "Aigbogun Alamba Daudu"),
        (2, "Murtala Afeez Bendu"),
        (3, "Okorocha Calistus Ogbona"),
        (4, "Adewale Jimoh Akanbi"),
        (5, "Osazuwa Faith Etiye"),
    ];

    // Dataset 2: Ministry
    let ministry_dataset: Vec<(u32, &str)> = vec![
        (1, "Internal Affairs"),
        (2, "Justice"),
        (3, "Defense"),
        (4, "Power & Steel"),
        (5, "Petroleum"),
    ];

    // Dataset 3: Geopolitical Zone
    let zone_dataset: Vec<(u32, &str)> = vec![
        (1, "South West"),
        (2, "North East"),
        (3, "South South"),
        (4, "South West"),
        (5, "South East"),
    ];

    // 2. Merge the datasets
    let merged_results = merge_datasets(
        &names_dataset,
        &ministry_dataset,
        &zone_dataset,
    );

    // 3. Output the single merged result
    println!("\n--- Merged Convicted Ministers Data ---");
    println!("{:<4} | {:<25} | {:<18} | {}", "S/N", "NAME OF COMMISSIONER", "MINISTRY", "GEOPOLITICAL ZONE");
    println!("-------------------------------------------------------------------------------------------------");

    for minister in merged_results {
        println!(
            "{:<4} | {:<25} | {:<18} | {}",
            minister.s_n, minister.name, minister.ministry, minister.geopolitical_zone
        );
    }
    println!("-------------------------------------------------------------------------------------------------");
}