use rand::Rng;
use std::cmp::Ordering;
use std::io;

static mut MAX_VALUE: u16 = 10;
static mut HINT_RANGE: u8 = 100; 
static mut MAX_GUESSES: u16 = u16::MAX;

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
            16 => {MAX_VALUE = 400;},
            17 => {MAX_VALUE = 500;},
            18 => {MAX_VALUE = 625;},
            19 => {MAX_VALUE = 800;},
            20 => {MAX_VALUE = 1000;},
            21 => {MAX_VALUE = 1250;},
            22 => {MAX_VALUE = 1750;},
            23 => {MAX_VALUE = 2500;},
            24 => {MAX_VALUE = 3500;},
            25 => {MAX_VALUE = 5000;},
            26 => {MAX_VALUE = 6500;},
            27 => {MAX_VALUE = 8500;},
            28 => {MAX_VALUE = 10_000;},
            29 => {MAX_VALUE = 15_000;},
            30 => {MAX_VALUE = 30_000;},
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
        } else if diff <= 20{
            HINT_RANGE = 25;
        } else if diff <= 25{
            HINT_RANGE = 20;
        } else if diff <= 30{
            HINT_RANGE = 15;
        }
        if diff <= 5{
            MAX_GUESSES = u16::MAX;
        } else if diff <= 7{
            MAX_GUESSES = 100;
        } else if diff <= 10{
            MAX_GUESSES = 75;
        } else if diff <= 13{
            MAX_GUESSES = 50;
        } else if diff <= 15{
            MAX_GUESSES = 35;
        } else if diff <= 20{
            MAX_GUESSES = 25;
        } else if diff <= 25{
            MAX_GUESSES = 20;
        } else if diff <= 30{
            MAX_GUESSES = 17;
        }
    }
}

fn main(){
    let mut diff: String; // difficulty
    
    println!("Please enter a difficulty name or level between 1 and 30");
    println!("Difficulty names: [effortless: 1, easy: 2, mild: 3, moderate: 4, ");
    println!("medium: 5, average: 6, above average: 7, talented: 8");
    println!("spicy: 9, hard: 10, difficult: 11, painful: 12, extreme: 13");
    println!("insane: 14, destructive: 15, tough: 16, unimaginable: 17, ");
    println!("grueling: 18, exhausting: 19, near impossible: 20, 21: uncompltetable");
    println!("22: unfriendly, 23: unknown, 24: deadly, 25: brutal: 26: fatal");
    println!("27: detrimental, 28: undisclosed, 29: unidentified, 30: impossible]");

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
            "tough" => set_difficulty(16),
            "unimaginable" => set_difficulty(17),
            "grueling" => set_difficulty(18),
            "exhausting" => set_difficulty(19),
            "near impossible" => set_difficulty(20),
            "uncompletable" => set_difficulty(21),
            "unfriendly" => set_difficulty(22),
            "unknown" => set_difficulty(23),
            "deadly" => set_difficulty(24),
            "brutal" => set_difficulty(25),
            "fatal" => set_difficulty(26),
            "detrimental" => set_difficulty(27),
            "undisclosed" => set_difficulty(28),
            "unidentified" => set_difficulty(29),
            "impossible" => set_difficulty(30),
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
    let mut guesses: u16;
    unsafe{
        max_value = MAX_VALUE;
        hint_range = HINT_RANGE;
        guesses = MAX_GUESSES;
    }
    let hint_range_num: u16 = 
        (max_value as f32 * ((hint_range as f32 / 100.))) as u16;

    println!("Guess the number");
    println!("Please input a number between 1 and {max_value}");
    println!("You have {guesses} guesses.");

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
                }            },
            Ordering::Greater => {
                if difference <= hint_range_num{
                    println!("Too big!");
                } else{
                    println!("Wrong number");
                }
            },
            Ordering::Equal => {println!("You win!"); break;},
        }
        guesses-=1;
        if guesses == 0{
            println!("Out of guesses, you lose.");
            break;
        }
        println!("{guesses} guesses left.");
    }

}