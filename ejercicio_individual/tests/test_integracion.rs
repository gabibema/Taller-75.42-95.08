use ejercicio_individual::operaciones::{crear_piezas, estado_piezas};
use ejercicio_individual::utils::constantes::*;
use ejercicio_individual::utils::errores::{error_pieza_invalida, error_piezas_invalidas};
use std::fs::read_to_string;

#[test]
fn prueba_tablero_vacio() {
    let mut tablero: String = match read_to_string("./src/tableros/tablero_vacio.txt") {
        Ok(resultado) => resultado,
        Err(_error) => return assert!(false, "El archivo no se pudo leer correctamente"),
    };

    match crear_piezas(&mut tablero) {
        Ok(_resultado) => assert!(false, "El mapa no debe ser válido"),
        Err(error) => assert_eq!(error.to_string(), error_piezas_invalidas().to_string()),
    };
}

#[test]
fn prueba_tablero_invalido() {
    let mut tablero: String = match read_to_string("./src/tableros/tablero_invalido.txt") {
        Ok(resultado) => resultado,
        Err(_error) => return assert!(false, "El archivo no se pudo leer correctamente"),
    };

    match crear_piezas(&mut tablero) {
        Ok(_resultado) => assert!(false, "Las piezas deberían ser invalidas"),
        Err(error) => assert_eq!(error.to_string(), error_pieza_invalida().to_string()),
    };
}

#[test]
fn prueba_tablero_con_piezas_blanca_captura() {
    let mut tablero: String = match read_to_string("./src/tableros/tablero1.txt") {
        Ok(resultado) => resultado,
        Err(_error) => return assert!(false, "El archivo no se pudo leer correctamente"),
    };

    let piezas = match crear_piezas(&mut tablero) {
        Ok(resultado) => resultado,
        Err(_error) => return assert!(false, "Las piezas no se pudieron leer correctamente"),
    };

    assert_eq!(estado_piezas(&*piezas[0], &*piezas[1]), BLANCA_CAPTURA)
}

#[test]
fn prueba_tablero_con_piezas_negra_captura() {
    let mut tablero: String = match read_to_string("./src/tableros/tablero2.txt") {
        Ok(resultado) => resultado,
        Err(_error) => return assert!(false, "El archivo no se pudo leer correctamente"),
    };

    let piezas = match crear_piezas(&mut tablero) {
        Ok(resultado) => resultado,
        Err(_error) => return assert!(false, "Las piezas no se pudieron leer correctamente"),
    };

    assert_eq!(estado_piezas(&*piezas[0], &*piezas[1]), NEGRA_CAPTURA)
}

#[test]
fn prueba_tablero_con_piezas_empate() {
    let mut tablero: String = match read_to_string("../src/tableros/tablero3.txt") {
        Ok(resultado) => resultado,
        Err(_error) => return (),
    };

    let piezas = match crear_piezas(&mut tablero) {
        Ok(resultado) => resultado,
        Err(_error) => return (),
    };

    assert_eq!(estado_piezas(&*piezas[0], &*piezas[1]), AMBAS_CAPTURAN)
}

#[test]
fn prueba_tablero_con_piezas_sin_captura() {
    let mut tablero: String = match read_to_string("../src/tableros/tablero4.txt") {
        Ok(resultado) => resultado,
        Err(_error) => return (),
    };

    let piezas = match crear_piezas(&mut tablero) {
        Ok(resultado) => resultado,
        Err(_error) => return (),
    };

    assert_eq!(estado_piezas(&*piezas[0], &*piezas[1]), NO_CAPTURA)
}
