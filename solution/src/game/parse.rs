use std::io;

pub type Dimensions = (isize, isize);

pub fn instruction() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

pub fn dimensions(s: &str) -> (isize, isize) {
    let dimensions: Vec<&str> = s
        .trim_end()
        .trim_end_matches(':')
        .split_ascii_whitespace()
        .skip(1)
        .collect();

    (
        dimensions[0].parse().unwrap(),
        dimensions[1].parse().unwrap(),
    )
}

pub fn player() -> u8 {
    match instruction()
        .split_ascii_whitespace()
        .collect::<Vec<&str>>()[2]
    {
        "p1" => 1,
        _ => 2,
    }
}
