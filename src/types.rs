#[derive(Clone, Copy)]
pub enum Task {
    One,
    Two,
    Both,
}

pub trait DaySolver {
    fn solve(&self, input: &str, test: bool, task: Task) -> (String, String) {
        let mut res1 = "".into();
        let mut res2 = "".into();
        if !matches!(task, Task::Two) {
            res1 = self.solve1(input, test);
        }
        if !matches!(task, Task::One) {
            res2 = self.solve2(input, test);
        }
        (res1, res2)
    }

    fn solve1(&self, input: &str, test: bool) -> String;
    fn solve2(&self, input: &str, test: bool) -> String;
}

macro_rules! test_print {
    ($test:expr,$($arg:tt)*) => {
        if $test {
            println!($($arg)*)
        }
    };
}
pub(crate) use test_print;
