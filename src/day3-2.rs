use std::io::{BufRead, BufReader};
use std::fs::File;
use std::convert::TryInto;

fn main() {
    let input_file = "input/day3.txt";
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);
    let mut n = 0;
    let mut g = 0;
    let mut rucksacks: [[i32;52];3] = [[0;52];3];
    let mut total = 0;

    let mut iterator = reader.lines().enumerate();
    while let Some((l, line)) = iterator.next() {
        let line = line.unwrap();
        println!("Reading line {}: {}, group {}, rucksack {}", l, line, g + 1, n + 1);
        for ch in line.chars() {
            let s:u32 = match ch {
                'a'..='z' => (ch as u32) - 96, // 1-26
                'A'..='Z' => 26 + (ch as u32) - 64, // 27-52
                _ => panic!()
            };
            rucksacks[n as usize][(s - 1) as usize] = 1;
            print!("{}={} ", ch, s);
        }
        println!("");
        n += 1;
        if n == 3 {
            g += 1;
            println!("Rucksack 1: {:?}", rucksacks[0]);
            println!("Rucksack 2: {:?}", rucksacks[1]);
            println!("Rucksack 3: {:?}", rucksacks[2]);
            let mut badge: Option<u32> = None;
            for i in 1..53 {
                let p: usize = i  - 1;
                if rucksacks[0][p] == 1 && rucksacks[1][p] == 1 && rucksacks[2][p] == 1 {
                    badge = Some(i.try_into().unwrap());
                    println!("Found badge {}", badge.unwrap());
                    break;
                }
            }

            if badge == None { println!("BADGE NOT FOUND!!!!!!!!!!!!!!"); }
            total += badge.unwrap();
            n = 0;
            rucksacks = [[0;52];3];
        }
    }

    println!("Total: {}", total);
}