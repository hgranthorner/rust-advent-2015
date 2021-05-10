mod day_one;

fn main() {
    match day_one::part_two() {
        Ok(v) => println!("{}", v),
        Err(e) => println!("{:?}", e)
    }
}
