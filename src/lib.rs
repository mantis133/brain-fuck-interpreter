use std::{fs, fmt::Display};
mod error;



// Errors occur when: </br>
// The given file contains characters that are not value UTF-8 </br>
// The file path doesnt already exist </br>
pub fn get_file_contents(filepath:&str) -> Result<String,error::Error>{
    let contents = fs::read_to_string(filepath)?;
    
    
    Ok(String::new())
}

fn get_tokens(){}