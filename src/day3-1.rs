use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let input_file = "input/day3.txt";
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);
    let mut total = 0;

    let mut iterator = reader.lines().enumerate();
    while let Some((_, line)) = iterator.next() {
        let line = line.unwrap();
        let size = line.chars().count();
        let half = size / 2;
        let c1 = &line[0..half];
        let c2 = &line[half..size];

        println!("Rucksack: {}", line);

        let mut shared: Option<char> = None;
        for i1 in c1.chars() {
            for i2 in c2.chars() {
                if i1 == i2 {
                    shared = Some(i1);
                    break;
                }
            if shared != None { break };
            }
        }
        if shared != None {
            let sh = shared.unwrap();

            let score = match sh {
                'a'..='z' => (sh as u32) - 96,
                'A'..='Z' => 26 + (sh as u32) - 64,
                _ => panic!()
            };
            total += score;
            println!("Match: {}, Score: {}", sh, score);
        }
    }
    println!("Total: {}", total);

}