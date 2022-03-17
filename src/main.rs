use std::io;
//this library will allow us to take input from user;

use rand::Rng;

use std::cmp::Ordering;



fn main() {
    
    loop {
        println!("Guess the number!");

    println!("Please enter your guess!");

    //create varialbe to store user input
    let mut guess = String::new();
    //this creates new string that can be used
    //keyword "mut" will make this variable mutable
    let secret_number = rand::thread_rng().gen_range(1, 101);
        
    

    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {}", guess );

    match guess.cmp(&secret_number) {
        Ordering::Greater =>  println!("Greater than"),
        Ordering::Equal => {
            println!("You win!");
            break;
        
        },
        Ordering::Less => println!("Less than"),

    }


    println!("Random number: {}", secret_number);
}



}
