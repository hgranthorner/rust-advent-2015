use std::convert::TryInto;

enum Direction {
    Up   ,
    Left ,
    Down ,
    Right
}

impl Direction {
    pub fn from_char(c: char) -> Option<Direction> {
        match c {
            '^' => Some(Direction::Up),
            '<' => Some(Direction::Left),
            'v' => Some(Direction::Down),
            '>' => Some(Direction::Right),
            _ => None
        }
    }
}

pub fn part_one() -> std::io::Result<i32> {
    let input = std::fs::read_to_string("/Users/grant/Dev/rust-advent/src/day_three/input.txt")?;
    let mut positions = std::collections::HashSet::new();
    let mut current_pos = (0, 0);
    positions.insert(current_pos);
    for c in input.chars() {
        if let Some(dir) = Direction::from_char(c) {
            let (x, y) = current_pos;
            current_pos = match dir {
                Direction::Up => (x, y + 1),
                Direction::Left => (x - 1, y),
                Direction::Down => (x, y - 1),
                Direction::Right => (x + 1, y)
            };
            positions.insert(current_pos);
        }
    }

    Ok(positions.len().try_into().unwrap())
}

pub fn part_two() -> std::io::Result<i32> {
    let file = std::fs::File::open("/Users/grant/Dev/rust-advent/src/day_three/sample.txt")?;
    Ok(-1)
}
