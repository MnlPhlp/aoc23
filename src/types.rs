#[derive(Clone, Copy)]
pub enum Task {
    One,
    Two,
    Both,
}

pub trait DaySolver<'a> {
    type Input;

    fn parse_input(input: &'a str) -> Self::Input;

    fn solve(&self, input: &'a str, test: bool, task: Task) -> (String, String) {
        let mut res1 = "".into();
        let mut res2 = "".into();
        let input = Self::parse_input(input);
        if !matches!(task, Task::Two) {
            res1 = self.solve1(&input, test);
        }
        if !matches!(task, Task::One) {
            res2 = self.solve2(&input, test);
        }
        (res1, res2)
    }

    fn solve1(&self, input: &Self::Input, test: bool) -> String;
    fn solve2(&self, input: &Self::Input, test: bool) -> String;
}

macro_rules! test_print {
    ($test:expr,$($arg:tt)*) => {
        if $test {
            println!($($arg)*)
        }
    };
}
pub(crate) use test_print;
