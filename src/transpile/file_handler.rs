use std::fs;
use std::fs::File;
use std::io::Write;

/// Gets all content of the script file
pub(crate) fn get_script_content(file_path: String) -> Vec<String> {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    contents.split("\n").map(str::to_string).collect()
}

pub(crate) fn write_string(path: String, data: String) -> std::io::Result<()> {
    let mut f = File::create(path)?;
    f.write_all(data.as_bytes())
}