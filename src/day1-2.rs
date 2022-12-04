use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let input_file = "input/day1.txt";
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let mut cur_calories: i32 = 0;

    let mut calories = Vec::<i32>::new();

    let mut iterator = reader.lines().enumerate().peekable();

    while let Some((_, line)) = iterator.next() {
        let line = line.unwrap();
        if line == "" || iterator.peek().is_none() {
            println!("Finished reading {} calories", cur_calories);
            calories.push(cur_calories);
            calories.sort();
            calories.reverse();
            let top1 = calories.get(0).unwrap();
            let top2 = calories.get(1).unwrap();
            let top3 = calories.get(2).unwrap();
            calories = vec![*top1, *top2, *top3];
            cur_calories = 0;
        } else {
            cur_calories += line.to_string().parse::<i32>().unwrap();
        }
    }

    let top1 = calories.get(0).unwrap();
    let top2 = calories.get(1).unwrap();
    let top3 = calories.get(2).unwrap();
    println!("Top 3 calories was {}", top1 + top2 + top3);

}