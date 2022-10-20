use std::io;

fn type_of<T>(_: &T) -> &str {
    std::any::type_name::<T>()
}

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

    println!("bla bla bla1");
    println!("bla bla bla1");

    println!("The {} {} {}", "quick", "brown", "fox");
    println!("The {2} {1} {0} {1} {2}", "quick", "brown", "fox");

    println!("{noun} {verb} {adjective}", noun="dog", verb="barks", adjective="loudly");
    println!("{noun} {verb} {adjective}", verb="barks", adjective="loudly", noun="dog");

    println!("The magic number is: {}", 49);

    println!("Base 10 repr      {}", 69420);
    println!("Base 2  repr      {:b}", 69420);
    println!("Base 8  repr      {:o}", 69420);
    println!("Base 16 repr      {:x}", 69420);
    println!("Base 16 repr      {:X}", 69420);

    println!("{number:0>10}", number=1);
    println!("{number:>10}", number=1);
    println!("{number:>9}", number=1);
    println!("{number:>9}", number=10);
    println!("{number:>8}", number=1);
    println!("{number:>7}", number=1);
    println!("{number:>6}", number=1);
    println!("{number:>5}", number=1);
    println!("{number:>4}", number=1);
    println!("{number:>3}", number=1);
    println!("{number:>2}", number=1);
    println!("{number:>1}", number=1);

    println!("My name is {1}, {0} {1}", "James", "Bond");

    let small:i32 = 1;

    // println!("this number is: {}", typeof(small));
    println!("this number is: {:?}", small);
    println!("");
    println!("");
    println!("");

    let extra_small_n: i8 = 1;
    let sorta_small_n: i16 = 1;
    let small_n: i32 = 1;
    let big_n: i64 = 1;
    let extra_big_n: i128 = 1;

    println!("The size of this number is {}", type_of(&extra_small_n));
    println!("The size of this number is {}", type_of(&sorta_small_n));
    println!("The size of this number is {}", type_of(&small_n));
    println!("The size of this number is {}", type_of(&big_n));
    println!("The size of this number is {}", type_of(&extra_big_n));

    let extra_small_un: u8 = 1;
    let sorta_small_un: u16 = 1;
    let small_un: u32 = 1;
    let big_un: u64 = 1;
    let extra_big_un: u128 = 1;

    println!("The size of this number is {}", type_of(&extra_small_un));
    println!("The size of this number is {}", type_of(&sorta_small_un));
    println!("The size of this number is {}", type_of(&small_un));
    println!("The size of this number is {}", type_of(&big_un));
    println!("The size of this number is {}", type_of(&extra_big_un));

    let my_string = "Hello, world";
    let my_char = 'a';

    let extra_small_number: u8 = 255;

    // TODO: HOMEWORK: Get a user input and test what happens with overflow
    // println!("{}", extra_small_number + 1);

    let my_array: [i32; 5] = [1, 2, 3, 4, 5];
    let my_vector: Vec<i32> = vec![1, 2, 3, 4, 5];
    let my_tuple: (i32, i32, i32, i32, i32) = (1, 2, 3, 4, 5);

    // struct data type in rust
    struct MyStruct {
        my_field: i32,
    }

    let immutable_number: i32 = 1;
    let mut mutable_number: i32 = 1;

    println!("The immutable number is {}", immutable_number);
    println!("The mutable number is {}", mutable_number);
    mutable_number = mutable_number + 1;
    println!("The mutable number is {}", mutable_number);

}
