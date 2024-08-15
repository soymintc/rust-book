use std::io;

fn main() {
    let mut system = String::new();
    println!("Please enter the temperature system (either F or C):");
    io::stdin()
        .read_line(&mut system)
        .expect("Failed to read line.");

    let mut temp = String::new();
    println!("Please enter the temperature to convert:");
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read temperature.");
    let temp: f64 = temp.trim().parse().expect("Wrong input for `temp`");
    
    let dst_temp: f64 = 
        if system.trim() == "C" {
            temp * 2.0 + 30.0
        } else {
            (temp - 30.0) / 2.0
        };

    println!("Converted temperature: {}", dst_temp);
}
