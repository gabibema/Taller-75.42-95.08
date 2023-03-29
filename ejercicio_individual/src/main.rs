use std::env::read_to_string;


fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let tablero = read_to_string(path);
}
