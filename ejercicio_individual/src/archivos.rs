use std::env::args;
use std::fs::read_to_string;
use std::io::{Error, ErrorKind::InvalidInput};


const CANTIDAD_ARGUMENTOS: usize = 2;

pub fn leer_archivo() -> Result<String, Error> {
    let args: Vec<String> = args().collect();
    if args.len() != CANTIDAD_ARGUMENTOS {
        return Err(Error::new(InvalidInput, "[ERROR] No se ingresó el path del tablero." ));
    }

    let archivo_leido = match read_to_string(&args[1]) {
        Err(error) => return Err(error),
        Ok(archivo_leido) => archivo_leido,
    };

    if archivo_leido.is_empty() {
        return Err(Error::new(InvalidInput, "[ERROR] El tablero está vacío"));
    }

    Ok(archivo_leido)
}