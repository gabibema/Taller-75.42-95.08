use ejercicio_individual::archivos::leer_archivo;
use ejercicio_individual::operaciones::{crear_piezas, mostrar_resultado};
use std::io::Error;


///Repositorio del seguimiento de la implementación:
///
fn main() -> Result<(), Error> {
    let mut tablero: String = match leer_archivo() {
        Ok(resultado) => resultado,
        Err(error) => return Err(error),
    };

    let piezas = match crear_piezas(&mut tablero) {
        Ok(resultado) => resultado,
        Err(error) => return Err(error),
    };

    mostrar_resultado(&piezas);
    Ok(())
}
