#![allow(non_snake_case)]
use std::io;        /*adding a scope to the program [basic input/output]*/
use rand::Rng;
use std::cmp::Ordering;



fn main() {         //the main function is entry point of the program
    
    println!("Guess Game 2020");        //macro that prints a string to the screen
    
    let secret_number = rand::thread_rng().gen_range(1,101);
    
    //println!("The secret number is: {}", secret_number);
    
    loop {

    println!("Please input your guess"); //macro that prints a string to the screen
    


    let mut guess = String::new();      /*create a place to store user input, 
                                        let is used to create a variable, 
                                        mut is used for mutable variable, 
                                        String::new returns a new string instance, 
                                        String is growable, 
                                        new creates a new value of some kind.*/

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
                                        /*Call stdin from io module
                                        acts as a handle to deal user input
                                        read_line gets input from the user
                                        (&mut guess) & acts as a reference
                                        .expect mocks the error to whatever
                                        has been added in the text. */

        let guess: u32 = match guess.trim().parse() {
               Ok(num) => num,
               Err(_) => continue,
                                        };
        //.expect("Please type a number!");
                                        
    println!("You guessed: {}", guess);     /*macro to print something
                                            curly brackets take the immediate 
                                            next variable's value */
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too Small", ),
        Ordering::Greater => println!("Too Big", ),
        Ordering::Equal => {
            println!("You Win", );
            break;
        }

    }

    }
}
