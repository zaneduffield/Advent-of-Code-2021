use std::time::Instant;

use advent_of_code_2021::*;

macro_rules! run_day {
    ($m:ident, $d:expr, $input: expr) => {
        let instant = Instant::now();
        println!(
            "day {}\n  part 1: {}\n  part 2: {}",
            $d,
            $m::part_1($input),
            $m::part_2($input)
        );
        println!("done in {:?}\n", instant.elapsed());
    };
}

pub fn main() {
    let instant = Instant::now();
    run_day!(day_1, "1", include_str!("../../input/2021/day1.txt"));
    run_day!(day_2, "2", include_str!("../../input/2021/day2.txt"));
    run_day!(day_3, "3", include_str!("../../input/2021/day3.txt"));
    run_day!(day_4, "4", include_str!("../../input/2021/day4.txt"));
    run_day!(day_5, "5", include_str!("../../input/2021/day5.txt"));
    run_day!(day_6, "6", include_str!("../../input/2021/day6.txt"));
    run_day!(day_7, "7", include_str!("../../input/2021/day7.txt"));
    run_day!(day_8, "8", include_str!("../../input/2021/day8.txt"));
    run_day!(day_9, "9", include_str!("../../input/2021/day9.txt"));
    run_day!(day_10, "10", include_str!("../../input/2021/day10.txt"));
    run_day!(day_11, "11", include_str!("../../input/2021/day11.txt"));
    run_day!(day_12, "12", include_str!("../../input/2021/day12.txt"));
    run_day!(day_13, "13", include_str!("../../input/2021/day13.txt"));
    run_day!(day_14, "14", include_str!("../../input/2021/day14.txt"));
    run_day!(day_15, "15", include_str!("../../input/2021/day15.txt"));
    run_day!(day_16, "16", include_str!("../../input/2021/day16.txt"));
    run_day!(day_17, "17", include_str!("../../input/2021/day17.txt"));
    run_day!(day_18, "18", include_str!("../../input/2021/day18.txt"));
    run_day!(day_19, "19", include_str!("../../input/2021/day19.txt"));
    run_day!(day_20, "20", include_str!("../../input/2021/day20.txt"));
    run_day!(day_21, "21", include_str!("../../input/2021/day21.txt"));
    run_day!(day_22, "22", include_str!("../../input/2021/day22.txt"));
    run_day!(day_23, "23", include_str!("../../input/2021/day23.txt"));
    run_day!(day_24, "24", include_str!("../../input/2021/day24.txt"));
    run_day!(day_25, "25", include_str!("../../input/2021/day25.txt"));

    println!("done in {:?}", instant.elapsed());
}
