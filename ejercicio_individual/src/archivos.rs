use std::env::args;
use std::fs::read_to_string;
use std::io::Error;
use crate::utils::errores::error_path_invalido;

const CANTIDAD_ARGUMENTOS: usize = 2;

pub fn leer_archivo() -> Result<String, Error> {
    let args: Vec<String> = args().collect();
    if args.len() != CANTIDAD_ARGUMENTOS {
        return Err(error_path_invalido())
    }

    let archivo_leido = match read_to_string(&args[1]) {
        Err(error) => return Err(error),
        Ok(archivo_leido) => archivo_leido,
    };

    Ok(archivo_leido)
}