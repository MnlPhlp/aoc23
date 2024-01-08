use std::time::Duration;

use aoc23::{calc_day, Task};

fn test_day(day: usize, exp1: &str, exp2: &str) {
    let mut res1 = "".into();
    let mut res2 = "".into();
    let mut duration = Duration::default();
    calc_day(
        day,
        &mut res1,
        &mut res2,
        &mut duration,
        false,
        Task::Both,
        false,
    );
    assert_eq!(exp1, res1, "task 1 gave wrong result");
    assert_eq!(exp2, res2, "task 2 gave wrong result");
}

#[test]
pub fn day1() {
    test_day(1, "54450", "54265")
}

#[test]
pub fn day2() {
    test_day(2, "2169", "60948")
}

#[test]
pub fn day3() {
    test_day(3, "544664", "84495585")
}

#[test]
pub fn day4() {
    test_day(4, "22488", "7013204")
}

#[test]
pub fn day5() {
    test_day(5, "178159714", "100165128")
}

#[test]
pub fn day6() {
    test_day(6, "1155175", "35961505")
}

#[test]
pub fn day7() {
    test_day(7, "247961593", "248750699")
}

#[test]
pub fn day8() {
    test_day(8, "21389", "21083806112641")
}

#[test]
pub fn day9() {
    test_day(9, "1939607039", "1041")
}

#[test]
pub fn day10() {
    test_day(10, "6733", "435")
}

#[test]
pub fn day11() {
    test_day(11, "9556712", "678626199476")
}

#[test]
pub fn day12() {
    test_day(12, "7939", "850504257483930")
}

#[test]
pub fn day13() {
    test_day(13, "35691", "39037")
}

#[test]
pub fn day14() {
    test_day(14, "108792", "99118")
}

#[test]
pub fn day15() {
    test_day(15, "508552", "265462")
}

#[test]
pub fn day16() {
    test_day(16, "7074", "7530")
}

#[test]
pub fn day17() {
    test_day(17, "907", "1057")
}
