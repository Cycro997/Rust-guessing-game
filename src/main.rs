use rand::Rng;
use std::cmp::Ordering;
use std::io;

static mut MAX_VALUE: u16 = 10;
static mut HINT_RANGE: u8 = 100; 

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
            11 => {MAX_VALUE = 125;},
            12 => {MAX_VALUE = 160;},
            13 => {MAX_VALUE = 200;},
            14 => {MAX_VALUE = 250;},
            15 => {MAX_VALUE = 325;},
            _ => {panic!("Invalid value")}
        }
        if diff <= 5{
            HINT_RANGE = 100;
        } else if diff <= 7{
            HINT_RANGE = 75;
        } else if diff <= 10{
            HINT_RANGE = 60;
        } else if diff <= 13{
            HINT_RANGE = 45;
        } else if diff <= 15{
            HINT_RANGE = 35;
        }
    }
}

fn main(){
    let mut diff: String; // difficulty
    
    println!("Please enter a difficulty name or level between 1 and 15");
    println!("Difficulty names: [effortless: 1, easy: 2, mild: 3, moderate: 4, ");
    println!("medium: 5, average: 6, above average: 7, talented: 8");
    println!("spicy: 9, hard: 10, difficult: 11, painful: 12, extreme: 13");
    println!("insane: 14, destructive: 15]");

    loop{
        diff = String::new();

        io::stdin()
            .read_line(&mut diff)
            .expect("Failed to read line");

        match &*(diff.trim()) {
            "effortless" => set_difficulty(1),
            "easy" => set_difficulty(2),
            "mild" => set_difficulty(3),
            "moderate" => set_difficulty(4),
            "medium" => set_difficulty(5),
            "average" => set_difficulty(6),
            "above average" => set_difficulty(7),
            "talented" => set_difficulty(8),
            "spicy" => set_difficulty(9),
            "hard" => set_difficulty(10),
            "difficult" => set_difficulty(11),
            "painful" => set_difficulty(12),
            "extreme" => set_difficulty(13),
            "insane" => set_difficulty(14),
            "destructive" => set_difficulty(15),
            _ => {
                if !diff.trim().chars().all(char::is_numeric){
                    println!("Please enter a valid difficulty");
                    continue;
                }
                set_difficulty(
                    diff.trim().parse().expect(
                        "unexpected behavior alert"
                    )
                )
            }
        }
        break;
    }
    let max_value: u16;
    let hint_range: u8;
    unsafe{
        max_value = MAX_VALUE;
        hint_range = HINT_RANGE;
    }
    let hint_range_num: u16 = 
        (max_value as f32 * ((hint_range as f32 / 100.))) as u16;

    println!("Guess the number");
    println!("Please input a number between 1 and {max_value}");

    let number: u16;

    number = rand::thread_rng().gen_range(1..=max_value);


    loop{
        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");


        let guess: u16 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number");
                continue;
            },
        };

        println!("You guessed: {}", guess);
        
        let difference: u16 = (
            guess as i16 - number as i16
        ).abs() as u16;

        match guess.cmp(&number) {
            Ordering::Less => {
                if difference <= hint_range_num{
                    println!("Too small!");
                } else{
                    println!("Wrong number");
                }
                continue;
            },
            Ordering::Greater => {
                if difference <= hint_range_num{
                    println!("Too big!");
                } else{
                    println!("Wrong number");
                }
                continue;
            },
            Ordering::Equal => {println!("You win!"); break;},
        }
    }

}