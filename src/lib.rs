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

pub fn get_stream(file_contents:&String) -> Result<Vec<char>,error::Error>{
    let res:Vec<&str> = file_contents.lines().into_iter().collect();
    match res.split_first(){
        Some((_,stream)) => {
            let stream:Vec<char> = Vec::from(stream.join(""))
                                    .into_iter()
                                    .map(|x| x as char)
                                    .collect();
            return Ok(stream)
        },
        None => {
            return Err(error::Error::Syntax("File is Empty".to_string()))
        }
    }
}

pub fn inputchar() -> char {
    let mut buff:String = String::new();
    std::io::stdin().read_line(&mut buff).expect("wrong dumbass");
    return buff.chars().nth(0).unwrap()
}

pub fn compute_token_ascii(mut memory:Vec<u8>, tape: Vec<char>) {
    let mut pointer_pos = 0;
    let mut tape_pos = 0;
    let mut loop_stack: Vec<usize> = Vec::new();
    let mut skipping = 0;
    let memory_len = memory.len();
    while tape_pos < tape.len() {
        let token = &tape[tape_pos];
        let current_value = &mut memory[pointer_pos];
        match token {
            '+' => {if skipping == 0{*current_value += 1}},
            '-' => {if skipping == 0{*current_value -= 1}},
            '>' => {if skipping == 0{if pointer_pos + 1 > (memory_len - 1){pointer_pos = 0} else {pointer_pos += 1}}},
            '<' => {if skipping == 0{if pointer_pos - 1 > (memory_len - 1){pointer_pos = memory_len - 1} else {pointer_pos -= 1}}},
            '.' => {if skipping == 0{print!("{}",*current_value as char)}},
            ',' => {if skipping == 0{*current_value = inputchar() as u8}},
            '[' => {if *current_value != 0 {loop_stack.push(pointer_pos); if skipping >= 1{skipping += 1}} else {skipping += 1}},
            ']' => {if *current_value == 0 {loop_stack.pop();if skipping >= 1{skipping -= 1}} else {tape_pos = loop_stack[loop_stack.len()-1];}},
            _ => {}
        }
        tape_pos += 1;
    }
}