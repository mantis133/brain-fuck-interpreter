use brain_fuck_interperter::*;




fn main() {
    let con = get_file_contents("main.bff").unwrap();
    let settings = get_settings(&con);
    let stream = get_stream(&con,);
    
    let mem = vec![0u8;30000];

    compute_token_ascii(mem, stream.unwrap());
}
