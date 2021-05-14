#![allow(dead_code)]
mod day_one;
mod day_two;
mod day_three;
mod day_four;

fn main() {
    match day_four::part_two() {
        Ok(v) => println!("{}", v),
        Err(e) => println!("{:?}", e)
    }
}
