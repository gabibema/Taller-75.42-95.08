use std::io::Error;
use crate::utils::constantes::*;
use crate::pieza::{crear_pieza, Pieza};

pub fn crear_piezas(tablero: &mut String) -> Result<Vec<Box<dyn Pieza>>,Error>{
    let mut piezas: Vec<Box<dyn Pieza>> = Vec::new();
    let mut columna:i8 = 0;
    let mut fila:i8 = 0;

    for caracter in tablero.chars() {
        if caracter != VACIO && caracter != SALTO_LINEA{
            match crear_pieza((fila, columna), caracter){
                Err(error) => return Err(error),
                Ok(pieza) => piezas.push(pieza),
            };
        }

        if columna == MAX_TABLERO {
            columna = 0;
            fila += 1;
        } else {
            columna += 1;
        }
    }
    return Ok(piezas);
}