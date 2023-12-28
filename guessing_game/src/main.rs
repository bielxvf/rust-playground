use rand::Rng;
use std::cmp::Ordering;

fn main() {
    const MIN: u32 = 1;
    const MAX: u32 = 100;

    let mut guess_sum = 0;
    let mut n_loops = 0;

    for n in 1..=10 {
        println!("    Running game {n}");
        let secret_number = rand::thread_rng().gen_range(MIN..=MAX);
        println!("        Secret number is: {secret_number}");
        let mut min_guess = MIN;
        let mut max_guess = MAX;
        let mut guess_count = 0;
        loop {
            let guess: u32 = guess_in_range(min_guess, max_guess);
            guess_count += 1;
            match guess.cmp(&secret_number) {
                Ordering::Less => {
                    println!("        Too small");
                    min_guess = guess;
                }
                Ordering::Greater => {
                    println!("        Too big");
                    max_guess = guess;
                }
                Ordering::Equal => {
                    println!("        You win!");
                    break;
                }
            }
        }
        guess_sum += guess_count;
        println!("        Guessed {guess_count} times");
        n_loops = n;
    }
    let avg: u32 = guess_sum / n_loops;
    println!("Avg guess_count: {}", avg);
}

fn guess_in_range(min: u32, max: u32) -> u32 {
    let guess = (min + max) / 2;
    println!("        Guessed: {guess}");
    guess
}
