pub fn part_one() -> std::io::Result<i32> {
    let input = std::fs::read_to_string("/Users/grant/Dev/rust-advent/src/day_one/input.txt")?;
    let mut floor = 0;
    for c in input.chars() {
        if c == '(' {
            floor += 1;
        } else if c == ')' {
            floor -= 1;
        }
    }
    Ok(floor)
}

pub fn part_two() -> std::io::Result<i32> {
    let input = std::fs::read_to_string("/Users/grant/Dev/rust-advent/src/day_one/input.txt")?;
    let mut position = 1;
    let mut floor = 0;
    for c in input.chars() {
        if c == '(' {
            floor += 1;
        } else if c == ')' {
            floor -= 1;
        }
        if floor == -1 {
            return Ok(position);
        }
        position += 1;
    }
    Ok(-1)
}
