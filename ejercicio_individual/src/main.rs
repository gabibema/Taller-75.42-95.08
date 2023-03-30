use std::io::Error;
use ejercicio_individual::archivos::leer_archivo;
use ejercicio_individual::pieza::crear_pieza;

fn main() -> Result<(), Error>{

    let tablero : String = match leer_archivo(){
            Ok(resultado) => resultado,
            Err(error) => return Err(error),
    };

    //let piezas = crear_piezas(&tablero);
    Ok(())
}
