use rand::thread_rng;
use rand::Rng;
use text_io::try_read;
use std::io::{self, Write};

fn main() {
    println!("Hello, world!");

    let int = 20;
    println!("num: {}", int);

    let vector: Vec<i32> = vec![2, 5, 3, 1, 9, 6, 7, 8, 0, 4];
    let sorted = sort(vector.clone());

    println!("array: {:?}", vector.clone());
    println!("sorted array: {:?}", sorted);

    let mut rng = thread_rng();
    let rand_int: i32 = rng.gen_range(0, 10);
    println!("random number: {}", rand_int);

    guessing_game();
}

trait Compare<T> {
    fn compare(&self, a: T, b: T) -> i32;
}

struct Comparator;

impl Compare<i32> for Comparator {
    fn compare(&self, a: i32, b: i32) -> i32 {
        if a < b {
            return -1;
        } else if a > b {
            return 1;
        } else {
            return 0;
        }
    }
}

fn sort(mut arr: Vec<i32>) -> Vec<i32> {
    let size = arr.len();

    for i in 0..size {
        let mut min = i;
        for n in i + 1..size {
            if arr[min] > arr[n] {
                min = n;
            }
        }

        let temp = arr[i];
        arr[i] = arr[min];
        arr[min] = temp;
    }
    return arr;
}

fn guessing_game() {
    print!("Pick a positive integer: ");
    io::stdout().flush().unwrap();

    let mut upper_bound = match try_read!() {
        Ok(upper_bound) => upper_bound,
        Err(_error) => 0,
    };
    while upper_bound < 1 {
        println!("ERROR: Invalid input\nPick a positive integer: ");

        upper_bound = match try_read!() {
            Ok(upper_bound) => upper_bound,
            Err(_error) => 0,
        };
    }

    println!("Guess a number between 1 and {}.", upper_bound);
    let mut rng = thread_rng();
    let num: i32 = rng.gen_range(1, upper_bound + 1);
    print!("   > ");
    io::stdout().flush().unwrap();
    let mut guess = match try_read!() {
        Ok(guess) => guess,
        Err(_error) => 0,
    };
    let mut guess_list = vec![guess];

    while guess != num {
        if guess > num {
            println!("Too high!");
        } else {
            println!("Too low!");
        }
        
        print!("   > ");
        io::stdout().flush().unwrap();
        guess = match try_read!() {
            Ok(guess) => guess,
            Err(_error) => 0,
        };
        guess_list.push(guess);
    }

    
    println!("You were able to find the number in {} guesses.", guess_list.len());
    println!("The numbers you guessed were: {:?}", sort(guess_list));
}