use std::fs::File;
use std::io::{Write, Result};

struct Student {
    name: String,
    matric: String,
    department: String,
    level: u32,
}

fn main() -> Result<()> {
    let students = vec![
        Student {
            name: "Oluchi Mordi".into(),
            matric: "ACC10211111".into(),
            department: "Accounting".into(),
            level: 300,
        },
        Student {
            name: "Adams Aliyu".into(),
            matric: "ECO10110101".into(),
            department: "Economics".into(),
            level: 100,
        },
        Student {
            name: "Shania Bolade".into(),
            matric: "CSC10328828".into(),
            department: "Computer".into(),
            level: 200,
        },
        Student {
            name: "Adekunle Gold".into(),
            matric: "EEE11020202".into(),
            department: "Electrical".into(),
            level: 200,
        },
        Student {
            name: "Blanca Edemoh".into(),
            matric: "MEE10202001".into(),
            department: "Mechanical".into(),
            level: 100,
        },
    ];

    println!("PAU SMIS");
    println!("{:<20} {:<15} {:<15} {:<5}",
        "Student Name", "Matric Number", "Department", "Level");

    for student in &students {
        println!("{:<20} {:<15} {:<15} {:<5}",
            student.name, student.matric, student.department, student.level);
    }

    let mut file = File::create("students.csv")?;
    writeln!(file, "Student Name,Matric Number,Department,Level")?;

    for student in &students {
        writeln!(
            file,
            "{},{},{},{}",
            student.name, student.matric, student.department, student.level
        )?;
    }

    println!("\nFile 'students.csv' created successfully!");

    Ok(())
}
