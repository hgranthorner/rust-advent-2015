#!/bin/bash

folder_path="./src/day_$1"

mkdir $folder_path
touch "$folder_path/mod.rs"
touch "$folder_path/input.txt"
touch "$folder_path/sample.txt"

echo "pub fn part_one() -> std::io::Result<i32> {
    let file = std::fs::File::open(\"/Users/grant/Dev/rust-advent/src/day_$1/sample.txt\")?;
    Ok(-1)
}

pub fn part_two() -> std::io::Result<i32> {
    let file = std::fs::File::open(\"/Users/grant/Dev/rust-advent/src/day_$1/sample.txt\")?;
    Ok(-1)
}" > "$folder_path/mod.rs"
