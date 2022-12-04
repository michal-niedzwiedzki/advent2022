use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let input_file = "input/day2.txt";
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let mut score = 0;

    // A, X - Rock
    // B, Y - Paper
    // C, Z - Scisors

    let mut iterator = reader.lines().enumerate();
    while let Some((_, line)) = iterator.next() {
        let line = line.unwrap();
        let challenge = line.chars().nth(0).unwrap();
        let response = line.chars().nth(2).unwrap();

        let result = match (challenge, response) {
            // win
            ('A', 'Y') | ('B', 'Z') | ('C', 'X') => 6,
            // loose
            ('A', 'Z') | ('B', 'X') | ('C', 'Y') => 0,
            // draw
            ('A', 'X') | ('B', 'Y') | ('C', 'Z') => 3,
            // otherwise
            _ => panic!()
        };
        let bonus: u32 = 3 - (90 - (response as u32));
        score += result + bonus;

        println!("Channenge: {}, Response: {}, Result: {}, Bonus: {} = Score: {}", challenge, response, result, bonus, score);
    }

    println!("Score: {}", score);
}