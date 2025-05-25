#![warn(clippy::single_char_pattern)]

use std::{num::ParseIntError, str::Split};

fn main() -> Result<(), ParseIntError> {
    let time = String::from("12:34:56");
    let seconds = time_to_seconds(&time)?;
    println!("{time} is {seconds}s");

    Ok(())
}

fn time_to_seconds(time: &str) -> Result<u32, ParseIntError> {
    let mut parts: Split<'_, &str> = time.split(":");

    let hours = parts.next().map_or(Ok(0), str::parse)?;
    let minutes = parts.next().map_or(Ok(0), str::parse)?;
    let mut seconds = parts.next().map_or(Ok(0), str::parse)?;
    seconds += minutes * 60 + hours * 3600;

    Ok(seconds)
}
