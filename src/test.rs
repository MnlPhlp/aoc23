use std::time::Duration;

use aoc23::{calc_day, Task};

fn test_day(day: usize, exp1: &str, exp2: &str) {
    let mut res1 = "".into();
    let mut res2 = "".into();
    let mut duration = Duration::default();
    calc_day(day, &mut res1, &mut res2, &mut duration, false, Task::Both);
    assert_eq!(exp1, res1, "task 1 gave wrong result");
    assert_eq!(exp2, res2, "task 2 gave wrong result");
    assert!(duration < Duration::from_secs(1), "took longer than 1 sec");
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
