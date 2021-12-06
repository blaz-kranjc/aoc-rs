use std::fs;

pub fn get_input(year: i32, day: i32) -> String {
    let filepath = format!(
        "{}/{}/inputs/{:02}.txt",
        env!("CARGO_MANIFEST_DIR"),
        year,
        day
    );
    fs::read_to_string(filepath).expect("File not found")
}
