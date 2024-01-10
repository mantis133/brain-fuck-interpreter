use brain_fuck_interperter::*;


fn main() {
    let con = get_file_contents("main.bff").unwrap();
    let settings = get_settings(&con).unwrap();
    let stream = get_stream(&con,).unwrap();
    
    settings.run(stream);
}
