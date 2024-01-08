use brain_fuck_interperter::*;

fn inputchar() {
    let mut buff:String = String::new();
    std::io::stdin().read_line(&mut buff).expect("wrong dumbass");
}

fn compute_token(mut memory:Vec<u8>, tape: Vec<char>) {
    let mut pos = 0;
    for token in tape {
        match token {
            '+' => {memory[pos] += 1},
            '-' => {memory[pos] -= 1},
            '>' => {pos += 1},
            '<' => {pos -= 1},
            '.' => {print!("{}",memory[pos] as char)},
            ',' => {},
            '[' => {},
            ']' => {},
            _ => {}
        }
    }
}

fn main() {
    let con = get_file_contents("main.bff").unwrap();

    let stream: Vec<char> = Vec::from(con).into_iter().map(|x| x as char).collect();
    let mut mem = vec![0u8;30000];

    compute_token(mem, stream);
}
