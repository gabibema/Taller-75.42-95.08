mod archivos;
mod pieza;

use std::io::Error;
use crate::archivos::leer_archivo;
use crate::pieza::crear_piezas;

fn main() -> Result<(), Error>{

    let tablero : String = match leer_archivo(){
            Ok(resultado) => resultado,
            Err(error) => return Err(error),
    };

    //let piezas = crear_piezas(&tablero);

    Ok(())
}
