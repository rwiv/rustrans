use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

pub fn read_lines(file_path: &Path) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    reader.lines().collect()
}

pub fn write_lines(vec: &Vec<String>, file_path: &Path) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    for s in vec {
        writeln!(file, "{}", s)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::path::get_project_root_path;

    #[test]
    fn test_read_lines() {
        let project_root = get_project_root_path();
        let file_path = project_root.join("tests").join("test.txt");
        let result = read_lines(&file_path);
        println!("{:?}", result);
    }

    #[test]
    fn test_write_vec() -> io::Result<()> {
        let project_root = get_project_root_path();
        let file_path = project_root.join("tests").join("test.txt");
        let strings = read_lines(&file_path)?;

        let new_file_path = project_root.join("tests").join("test2.txt");
        write_lines(&strings, &new_file_path)?;
        Ok(())
    }
}
