pub use std::io::BufRead;
pub use std::io::BufReader;

#[macro_export]
macro_rules! read_lines {
    ($expression:expr) => {
        std::io::BufReader::new(std::fs::File::open($expression)?)
            .lines()
            .filter(|l| l.is_ok())
            .map(|l| l.unwrap())
    };
}
