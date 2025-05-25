#![warn(clippy::single_char_pattern)]

use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let time = String::from("12:34:56");
    let mut parts = time.split(":");

    let hours = parts.next().map_or(Ok(0), str::parse)?;
    let minutes = parts.next().map_or(Ok(0), str::parse)?;
    let mut seconds = parts.next().map_or(Ok(0), str::parse)?;
    seconds += minutes * 60 + hours * 3600;

    println!("{time} is {seconds}s");
    Ok(())
}
