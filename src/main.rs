use std::process::Command;

fn main() {
    let examples = vec!["triangle", "clear_color"];

    println!("Available examples:");
    for (i, example) in examples.iter().enumerate() {
        println!("{}. {}", i + 1, example);
    }

    println!(
        "\nEnter the number of the example you want to run (1-{}):",
        examples.len()
    );

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let choice: usize = input.trim().parse().unwrap_or(0);

    if choice > 0 && choice <= examples.len() {
        let example = examples[choice - 1];
        println!("Running example: {}", example);

        let status = Command::new("cargo")
            .args(["run", "--example", example])
            .status()
            .expect("Failed to execute example");

        std::process::exit(status.code().unwrap_or(1));
    } else {
        println!("Invalid selection");
    }
}
