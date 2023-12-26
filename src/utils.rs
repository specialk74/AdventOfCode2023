use std::fs::read_to_string;

pub fn my_read_file(filename: &str) -> Result<String, ()> {
    if let Ok(lines) = read_to_string(filename) {
        return Ok(lines);
    }
    
    println!("{} Not Founded", filename);
    return Err(());
}