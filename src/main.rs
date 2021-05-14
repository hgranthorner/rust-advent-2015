mod day_one;
mod day_two;
mod day_three;
mod day_four;

fn main() {
    match day_four::part_one() {
        Ok(v) => println!("{}", v),
        Err(e) => println!("{:?}", e)
    }
}
