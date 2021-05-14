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
    let input = std::fs::read_to_string("/Users/grant/Dev/rust-advent/src/day_three/input.txt")?;
    let mut santa = std::collections::HashSet::new();
    let mut robot = std::collections::HashSet::new();
    let mut santa_position = (0, 0);
    let mut robot_position = (0, 0);
    santa.insert(santa_position);
    for ( i, c ) in input.chars().enumerate() {
        if i % 2 == 0 {
            santa_position = move_and_record(&mut santa, santa_position, c);
        } else {
            robot_position = move_and_record(&mut robot, robot_position, c);
        }
    }

    let total = santa.union(&robot).collect::<Vec<_>>().len();
    Ok(total.try_into().unwrap())
}

fn move_and_record(sleigh: &mut std::collections::HashSet<(i32, i32)>, current_pos: (i32, i32), c: char) -> (i32, i32) {
    let (x, y) = current_pos;
    if let Some(dir) = Direction::from_char(c) {
        let pos = match dir {
            Direction::Up => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Down => (x, y - 1),
            Direction::Right => (x + 1, y)
        };
        sleigh.insert(pos);
        return pos;
    }
    current_pos
}
