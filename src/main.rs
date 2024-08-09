use std::io;
use rand::Rng;
use std::cmp::Ordering;

static mut MAX_VALUE: u16 = 0;

fn set_difficulty(diff: u8){
    println!("DIFFICULTY: {diff}");
    unsafe{
        match diff{
            1 => {MAX_VALUE = 5;},
            2 => {MAX_VALUE = 7;},
            3 => {MAX_VALUE = 10;},
            4 => {MAX_VALUE = 15;},
            5 => {MAX_VALUE = 20;},
            6 => {MAX_VALUE = 30;},
            7 => {MAX_VALUE = 45;},
            8 => {MAX_VALUE = 60;},
            9 => {MAX_VALUE = 80;},
            10 => {MAX_VALUE = 100;},
            _ => {}
        }
    }
}

fn main(){
    let mut diff: String = String::new(); // difficulty
    
    println!("Please enter a difficulty name or level between 1 and 10");
    println!("Difficulty names: [effortless: 1, easy: 2, mild: 3, moderate: 4, ");
    println!("medium: 5, average: 6, above average: 7, talented: 8");
    println!("spicy: 9, hard: 10]");

    io::stdin()
        .read_line(&mut diff)
        .expect("Failed to read line");

    match &*(diff.trim()) {
        "easy" => set_difficulty(1),
        _ => {
            if !diff.trim().chars().all(char::is_numeric){
                panic!("Nuh uh");
            }
            set_difficulty(
                diff.trim().parse().expect(
                    "unexpected behavior alert"
                )
            )
        }
    }    
    let max_value: u16;
    unsafe{
        max_value = MAX_VALUE
    }

    println!("Guess the number");
    println!("Please input a number between 1 and {max_value}");

    let number: u16;

    

    number = rand::thread_rng().gen_range(1..=max_value);


    loop{
        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u16 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {}", guess);

        match guess.cmp(&number) {
            Ordering::Less => {println!("Too small!"); continue;},
            Ordering::Greater => {println!("Too big!"); continue;},
            Ordering::Equal => {println!("You win!"); break;},
        }
    }

}