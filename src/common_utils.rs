use std::fs;

pub(crate) fn read_file(path: String) -> String {
    let path = format!("/home/illin/RustroverProjects/AdventOfCode2025/input_files/{path}");
    fs::read_to_string(path).expect("Should have been able to read the file")
}