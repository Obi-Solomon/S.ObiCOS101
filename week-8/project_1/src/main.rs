use std::io;

// Simple helper for user input
fn ask(question: &str) -> String {
    println!("{}", question);
    let mut text = String::new();
    std::io::stdin().read_line(&mut text).expect("Input error");
    text.trim().to_string()
}

fn main() {

    // Departments in order of columns
    let departments = vec![
        "Office Administrator",
        "Academic",
        "Lawyer",
        "Teacher",
    ];

    // APS table stored as (APS Level, row of job titles)
    let aps_table = vec![
        ("APS 1-2",   vec!["Intern", "-", "Paralegal", "Placement"]),
        ("APS 3-5",   vec!["Administrator", "Research Assistant", "Junior Associate", "Classroom Teacher"]),
        ("APS 5-8",   vec!["Senior Administrator", "PhD Candidate", "Associate", "Snr Teacher"]),
        ("EL 1 8-10", vec!["Office Manager", "Post-Doc Researcher", "Senior Associate 1-2", "Leading Teacher"]),
        ("EL 2 10-13",vec!["Director", "Senior Lecturer", "Senior Associate 3-4", "Deputy Principal"]),
        ("SES",       vec!["CEO", "Dean", "Partner", "Principal"]),
    ];

    println!("--- Public Service APS Level Checker ---");

    let role = ask("Enter staff role (e.g. Associate, Administrator, CEO):");
    let dept = ask("Enter department (Office Administrator / Academic / Lawyer / Teacher):");

    // Find which department column we are checking
    let dept_index = departments
        .iter()
        .position(|d| d.eq_ignore_ascii_case(&dept));

    if dept_index.is_none() {
        println!("Invalid department. Please check spelling.");
        return;
    }

    let column = dept_index.unwrap();
    let mut matched = false;

    // Now search through the APS table
    for (aps_level, row) in &aps_table {
        if row[column].eq_ignore_ascii_case(&role) {
            println!("\nMatch found!");
            println!("Role: {}", role);
            println!("Department: {}", dept);
            println!("APS Level: {}", aps_level);
            matched = true;
            break;
        }
    }

    if !matched {
        println!("\nNo APS level found for that role and department.");
    }
}
