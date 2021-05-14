pub fn part_one() -> std::io::Result<i32> {
    let input = std::fs::read_to_string("/Users/grant/Dev/rust-advent/src/day_four/input.txt")?
        .replace("\n", "");
    let mut suffix = 0;
    let mut hash = "1111111".to_string();
    let prefix = "00000".to_string();
    let test = "00000";
    while hash[..5] != *test {
        suffix = suffix + 1;
        let hash_input = format!("{}{}", input, suffix);
        hash = format!("{:?}", md5::compute(hash_input));
    }
    Ok(suffix)
}

pub fn part_two() -> std::io::Result<i32> {
    let file = std::fs::File::open("/Users/grant/Dev/rust-advent/src/day_four/sample.txt")?;
    Ok(-1)
}
