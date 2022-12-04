use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let input_file = "input/day1.txt";
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let mut cur_calories: i32 = 0;
    let mut cur_index: i32 = 1;
    let mut top_calories: i32 = 0;
    let mut top_index: i32 = 1;

    let mut iterator = reader.lines().enumerate().peekable();

    while let Some((_, line)) = iterator.next() {
        let line = line.unwrap();
        if line == "" || iterator.peek().is_none() {
            println!("Finished reading index {} with {} calories", cur_index, cur_calories);
            if cur_calories > top_calories {
                println!("Found new record of {} calories", cur_calories);
                top_calories = cur_calories;
                top_index = cur_index;
            }
            cur_calories = 0;
            cur_index += 1;
        } else {
            cur_calories += line.to_string().parse::<i32>().unwrap_or(0);
        }
    }
    println!("Top calories was {} at index {}", top_calories, top_index);

}