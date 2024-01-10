use brain_fuck_interpreter::*;



fn main() {
    // APPNAME -- main.bff 

    let filename = match sort_it_out(){
        Ok(path) => {path},
        Err(error) => {std::process::exit(1)}
    };

    let file_contents = match get_file_contents(&filename){
        Ok(content) => {content},
        Err(error) => {std::process::exit(2)}
    };

    let config = match get_settings(&file_contents){
        Ok(config) => {config},
        Err(error) => {std::process::exit(3)}
    };

    let token_stream = match get_stream(&file_contents){
        Ok(tokens) => {tokens},
        Err(error) => {std::process::exit(4)}
    };
    
    config.run(token_stream);
}
