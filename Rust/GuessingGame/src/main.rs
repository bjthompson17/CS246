#![allow(non_snake_case)]

/*
This is a random number module that I have made myself. 
It uses the current system time in microseconds modified by the previously generated
number to generate a new random integer, which is then haphazardly bound to the
disired range ... in absolute randomness. Can't really seed this generator, sorry.
*/
pub mod my_rand {
    use std::time::{SystemTime,UNIX_EPOCH};
    pub struct Generator {
        prev_i32:i32
    }

    impl Generator {
        pub fn new() -> Generator {
            Generator { prev_i32:0 }
        }

        pub fn int(&mut self,min:i32, max:i32) -> i32 {
            let t = SystemTime::now().duration_since(UNIX_EPOCH)
                                    .expect("This is the past. The unix epoch hasn't happend yet.")
                                    .as_micros()
                                    * (self.prev_i32 as u128 + 1); 
                                    // + 1 so that we don't ever multiply by 0 and get stuck there
            let diff = max - min;
            self.prev_i32 = ((t % diff.abs() as u128) as i32) * diff.signum() + min;
            return self.prev_i32;
        }

    }
}

// Basic input macro. Returns user input as a string
macro_rules! input {
    ($($arg:tt),*) => { 
      {
        use std::io::Write;
        print!($($arg),*);
        std::io::stdout().flush().unwrap();
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        buffer.pop(); // pop the newline character off.
        buffer
      }
    };
  }

fn main() {
    let mut rand = my_rand::Generator::new();
    let number = rand.int(1,100);
    let mut tries = 1;
    let mut guess:i32 = -1;
    println!("I'm thinking of a number between 1 and 100. How many tries will it take you to guess it?");
    while guess != number {
        if guess != -1 {
            if guess > number {
                println!("Too high.")
            } else {
                println!("Too low.")
            }
            tries += 1;
        }
        guess = input!("Try number {}: ",tries).trim().parse::<i32>().unwrap();
    }
    if tries > 8 {
        println!("You need to work on your guessing skills. It took you {} tries.", tries);
    } else if tries > 4 {
        println!("Not bad. You got it in {} tries.", tries);
    } else if tries > 1 {
        println!("That was pretty lucky! You got it in {} tries.", tries);
    }else {
        println!("You lucky son of a ... well nevermind what! First try!");
    }
}