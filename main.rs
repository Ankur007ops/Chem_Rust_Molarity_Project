use std::io;
// Formula for molarity of solution when % of solution givien.
// Molarity = (% of solution * density of solution * 10) / (Molar mass of solution)
fn main() {
    println!("Enter the % of solution:");
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let percent: f64 = input.trim().parse().expect("Invalid input");

    println!("Enter the density of solution:");
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let density: f64 = input.trim().parse().expect("Invalid input");

    println!("Enter the molar mass of solution:");
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let molar_mass: f64 = input.trim().parse().expect("Invalid input");

    let molarity_1: f64 = (percent) * density * 10.0 / molar_mass;
    println!("The molarity of the solution is: {:.2}", molarity_1);

// Molarity_1 * Requried_Vol = Target_Volume * molarity_2
// Required_Volume = (Molarity_2(input) * target_volume)/ moolarity_1
    println!("Enter the Molarity_2 of solution:");
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let molarity_2: f64 = input.trim().parse().expect("Invalid input");

    println!("Enter the Target_Volume of solution:");
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let target_volume: f64 = input.trim().parse().expect("Invalid input");

    let required_volume: f64 = (molarity_2 * target_volume)/ molarity_1;
    println!("The Requried_Volume of the solution is: {:.2}", required_volume);

}
