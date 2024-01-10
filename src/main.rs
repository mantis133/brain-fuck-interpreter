use brain_fuck_interperter::*;

fn inputchar() -> char {
    let mut buff:String = String::new();
    std::io::stdin().read_line(&mut buff).expect("wrong dumbass");
    return buff.chars().nth(0).unwrap()
}

fn compute_token_ascii(mut memory:Vec<u8>, tape: Vec<char>) {
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

fn main() {
    let con = get_file_contents("main.bff").unwrap();
    let settings = get_settings(&con);
    let stream = get_stream(&con,);
    
    let mem = vec![0u8;30000];

    compute_token_ascii(mem, stream.unwrap());
}
