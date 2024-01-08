use brain_fuck_interperter::*;

fn inputchar() -> char {
    let mut buff:String = String::new();
    std::io::stdin().read_line(&mut buff).expect("wrong dumbass");
    return buff.chars().nth(0).unwrap()
}

fn compute_token_ascii(mut memory:Vec<u8>, tape: Vec<char>) {
    let mut pos = 0;
    let mut loop_stack: Vec<usize> = Vec::new();
    for token in tape {
        match token {
            '+' => {memory[pos] += 1},
            '-' => {memory[pos] -= 1},
            '>' => {pos += 1},
            '<' => {pos -= 1},
            '.' => {print!("{}",memory[pos] as char)},
            ',' => {memory[pos] = inputchar() as u8},
            '[' => {if memory[pos] != 0 {loop_stack.push(pos)}},
            ']' => {if memory[pos] == 0 {loop_stack.pop();}},
            _ => {}
        }
    }
}

fn main() {
    let con = get_file_contents("main.bff").unwrap();

    let stream: Vec<char> = Vec::from(con).into_iter().map(|x| x as char).collect();
    let mut mem = vec![0u8;30000];

    compute_token_ascii(mem, stream);
}
