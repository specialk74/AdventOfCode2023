pub fn from_slice_to_vec<T>(input: &str) -> Vec<T>
    where T: std::str::FromStr, <T as std::str::FromStr>::Err : std::fmt::Debug
{
     let result: Vec<T> = input
        .split_whitespace()
        .map(|x| x.parse::<T>().unwrap())
        .collect();
    result  
}