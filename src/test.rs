use std::time::Duration;

use crate::{calc_day, types::*};

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