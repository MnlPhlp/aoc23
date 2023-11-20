#[derive(Clone, Copy)]
pub enum Task {
    One,
    Two,
    Both,
}

pub trait DaySolver {
    fn solve(&self, input: &str, _test: bool, _task: Task) -> (String, String) {
        ("Not".into(), "Implemented".into())
    }
}
