mod archivos;

use std::io::Error;
use crate::archivos::leer_archivo;

fn main() -> Result<(), Error>{

    let tablero : String = match leer_archivo(){
            Ok(resultado) => resultado,
            Err(error) => return Err(error),
    };

    Ok(())
}
