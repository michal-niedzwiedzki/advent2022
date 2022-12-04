use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let input_file = "input/day2.txt";
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let mut score = 0;

    // A - Rock
    // B - Paper
    // C - Scisors

    // X - loose
    // Y - draw
    // Z - win

    let mut iterator = reader.lines().enumerate();
    while let Some((_, line)) = iterator.next() {
        let line = line.unwrap();
        let challenge = line.chars().nth(0).unwrap();
        let result = line.chars().nth(2).unwrap();
        let response = match (challenge, result) {
            // rock
            ('B', 'X') | ('A', 'Y') | ('C', 'Z') => 'A',
            // paper
            ('C', 'X') | ('B', 'Y') | ('A', 'Z') => 'B',
            // scisors
            ('A', 'X') | ('C', 'Y') | ('B', 'Z') => 'C',
            // otherwise
            _ => panic!()
        };
        let points = match result {
            'X' => 0,
            'Y' => 3,
            'Z' => 6,
            _ => panic!()
        };
        let bonus: u32 = 3 - (67 - (response as u32));
        score += points + bonus;

        println!("Channenge: {}, Result: {}, Response: {}, Bonus: {} = Score: {}", challenge, result, response, bonus, score);
    }

    println!("Score: {}", score);
}