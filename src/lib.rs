use std::fs;
mod error;


pub enum Settings {
    Ascii(Vec<u8>),
    UTF8(Vec<u32>)
}

impl std::fmt::Display for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Settings::Ascii(memory) => {write!(f,"{:?}", memory)},
            Settings::UTF8(memory) => {write!(f,"{:?}", memory)}
        }
    }
}

// Errors occur when: </br>
// The given file contains characters that are not value UTF-8 </br>
// The file path doesnt already exist </br>
pub fn get_file_contents(filepath:&str) -> Result<String,error::Error>{
    let contents = fs::read_to_string(filepath)?;
    
    Ok(contents)
}


// might want to remove type casting later.
pub fn get_settings(file_contents:&String) -> Result<Settings,error::Error>{
    let lines:Vec<&str> = file_contents.lines().into_iter().collect();
    let settings_line = String::from(lines[0]);

    let kwargs = settings_line.replace("_", "");
    let kwargs:Vec<&str> = kwargs.split(" - ").collect();

    match kwargs[0].parse(){
        Ok(size) => {
            return match kwargs[1].trim() {
                "u8" => {Ok(Settings::Ascii(vec![0;size]))},
                "u32" => {Ok(Settings::UTF8(vec![0;size]))},
                _ => {Err(error::Error::Syntax("Missing either array size or character encoding method on FIRST LINE".to_string()))}
            }
        },
        Err(_) => {// fails when the size is non rust numeric
            return Err(error::Error::Syntax("Given array size is non numeric".to_string()));
        }
    }
}

pub fn get_tokens(file_contents:String){
    
}