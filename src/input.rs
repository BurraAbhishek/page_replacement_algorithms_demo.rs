pub fn input() -> String {
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read inpupt");
    buffer
}

pub fn input_u8() -> u8 {
    let buffer = input();
    let value: u8 = buffer
        .trim()
        .parse()
        .expect("The input is not an integer between 0 and 255");
    value
}

#[macro_export]
macro_rules! vec_strings {
    ($($x:expr), *) => (vec![$($x.to_string()), *]);
}