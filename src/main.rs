mod day_one;
mod day_two;
mod day_three;

fn main() {
    match day_three::part_one() {
        Ok(v) => println!("{}", v),
        Err(e) => println!("{:?}", e)
    }
}
