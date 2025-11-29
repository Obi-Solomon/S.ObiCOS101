use std::fs::File;
use std::io::{Write, Result};

fn main() -> Result<()> {
    let commissioners = [
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etiyey",
    ];

    let ministries = [
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    let zones = [
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    println!("Merged EFCC Dataset\n");
    println!(
        "{:<5} {:<28} {:<18} {:<15}",
        "S/N", "Commissioner", "Ministry", "Geo-Political Zone"
    );

    for i in 0..commissioners.len() {
        println!(
            "{:<5} {:<28} {:<18} {:<15}",
            i + 1,
            commissioners[i],
            ministries[i],
            zones[i]
        );
    }

    let mut file = File::create("efcc_merged.csv")?;
    writeln!(file, "S/N,Commissioner,Ministry,Geo-Political Zone")?;

    for i in 0..commissioners.len() {
        writeln!(
            file,
            "{},{},{},{}",
            i + 1,
            commissioners[i],
            ministries[i],
            zones[i]
        )?;
    }

    Ok(())
}
