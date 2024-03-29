mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;

use crate::aoc_lib::DayFn;

pub fn get_day(day: u8) -> (Option<DayFn>, Option<DayFn>) {
    match day {
        1 => (Some(day01::part1), Some(day01::part2)),
        2 => (Some(day02::part1), Some(day02::part2)),
        3 => (Some(day03::part1), Some(day03::part2)),
        4 => (Some(day04::part1), Some(day04::part2)),
        5 => (Some(day05::part1), Some(day05::part2)),
        6 => (Some(day06::part1), Some(day06::part2)),
        7 => (Some(day07::part1), Some(day07::part2)),
        8 => (Some(day08::part1), Some(day08::part2)),
        9 => (Some(day09::part1), Some(day09::part2)),
        10 => (Some(day10::only_part), None),
        11 => (Some(day11::part1), Some(day11::part2)),
        12 => (Some(day12::part1), Some(day12::part2)),
        13 => (Some(day13::part1), Some(day13::part2)),
        14 => (Some(day14::part1), Some(day14::part2)),
        15 => (Some(day15::part1), Some(day15::part2)),
        16 => (Some(day16::part1), Some(day16::part2)),
        17 => (Some(day17::part1), Some(day17::part2)),
        18 => (None, None),
        19 => (None, None),
        20 => (None, None),
        21 => (None, None),
        22 => (None, None),
        23 => (None, None),
        24 => (None, None),
        25 => (None, None),
        _ => {
            println!("Unknown day: {}", day);
            (None, None)
        }
    }
}
