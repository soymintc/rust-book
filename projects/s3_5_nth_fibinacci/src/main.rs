use std::io;

fn main() {
    let mut n = String::new();
    println!("Please enter an integer for the n-th Fibonacci number:");
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line.");

    let n: i32 = n.trim().parse().expect("Wrong input for `n`");
    // println!("n = {n}");

    let (mut a1, mut a2) = (1, 1);
    let mut nth = 1;
    let mut count = 0;
    loop {
        count += 1;
        if count > n { 
            break; 
        }
        else if count <= 2 {
            continue;
        } else {
            nth = a1 + a2;
            a1 = a2;
            a2 = nth;
        }
        println!("count: {count}, nth: {nth}");
    }
    println!("The {n}-th Fibonacci number is: {nth}");

}
