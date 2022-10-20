use std::io;

mod clase02;

fn main() {

    /*
    println!("Welcome to your number adding machine!");

    println!("Please enter your first number:");
    let mut first_number = String::new();
    io::stdin().read_line(&mut first_number).expect("Failed to read line"); // Some basic error handling.

    println!("Please enter your second number:");
    let mut second_number = String::new();
    io::stdin().read_line(&mut second_number).expect("Failed to read line");

    // let mut a_number = "1";

    // let first_number: i32 = |x: String| x.trim().parse::<i32>().unwrap().expect("Please type a number!");
    // let first_number: i32 = |x: String| x.trim().parse::<i32>().unwrap().expect("Please type a number!");

    let first_number: i32 = first_number.trim().parse().expect("Please type a number!");
    let second_number: i32 = second_number.trim().parse().expect("Please type a number!");

    println!("we are here");

    println!("The sum of your numbers is: {}", first_number + second_number);
    */
    clase02::clase02();
}
