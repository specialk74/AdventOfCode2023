use std::fs::read_to_string;

pub fn my_read_file(filename: &str) -> Result<String, ()> {
    if let Ok(lines) = read_to_string(filename) {
        return Ok(lines);
    }
    
    println!("{} Not Founded", filename);
    return Err(());
}

pub fn from_slice_to_vec_u64(input: &str) -> Vec<u64>{
     let result = input
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    result  
}