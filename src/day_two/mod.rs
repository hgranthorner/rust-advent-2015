use std::io::BufRead;

pub fn part_one() -> std::io::Result<i32> { 
    let file = std::fs::File::open("/Users/grant/Dev/rust-advent/src/day_two/input.txt")?;
    let mut total_area = 0;
    for line_result in std::io::BufReader::new(file).lines() {
        let line = line_result?;
        let mut splt = line.split("x").map(| v | v.parse::<i32>().unwrap());
        let l = splt.next().expect("Data is well formed");
        let w = splt.next().expect("Data is well formed");
        let h = splt.next().expect("Data is well formed");
        let area = 2*l*w + 2*w*h + 2*h*l;
        let extra = std::cmp::min(l*w, std::cmp::min(l*h, h*w));
        total_area += area + extra;
    }
    Ok(total_area)
}
pub fn part_two() -> std::io::Result<i32> { 
    let file = std::fs::File::open("/Users/grant/Dev/rust-advent/src/day_two/input.txt")?;
    let mut total_area = 0;
    for line_result in std::io::BufReader::new(file).lines() {
        let line = line_result?;
        let mut splt = line.split("x").map(| v | v.parse::<i32>().unwrap());
        let l = splt.next().expect("Data is well formed");
        let w = splt.next().expect("Data is well formed");
        let h = splt.next().expect("Data is well formed");
        let (x1, x2) = smallest_two(l, w, h);
        let len_wrap = x1 * 2 + x2 * 2;
        let len_ribbon = l * w * h;
        total_area += len_wrap + len_ribbon;
    }
    Ok(total_area)
}

fn smallest_two(l: i32, w: i32, h: i32) -> (i32, i32) {
    let mut nums = vec![l, w, h];
    nums.sort_unstable();
    return (nums[0], nums[1]);
}
