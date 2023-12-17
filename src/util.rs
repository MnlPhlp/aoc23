use std::{fs::File, io::Read};

pub fn read_input(day: usize, test: bool) -> String {
    let path = format!(
        "./inputs/day{day:0>2}{}.txt",
        if test { "_test" } else { "" }
    );
    let mut file = File::open(&path).unwrap_or_else(|_| panic!("File '{path}' not found"));
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");
    #[cfg(target_os = "windows")]
    {
        contents = contents.replace("\r\n", "\n");
    }
    contents
}
