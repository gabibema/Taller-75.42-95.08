use crate::utils::errores::error_path_invalido;
use std::env::args;
use std::fs::read_to_string;
use std::io::Error;

const CANTIDAD_ARGUMENTOS: usize = 2;

pub fn leer_archivo() -> Result<String, Error> {
    let args: Vec<String> = args().collect();
    if args.len() != CANTIDAD_ARGUMENTOS {
        return Err(error_path_invalido());
    }

    read_to_string(&args[1])
}
