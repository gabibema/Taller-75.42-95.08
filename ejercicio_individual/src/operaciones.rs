use std::io::Error;
use crate::utils::constantes::*;
use crate::pieza::{crear_pieza, Pieza};
use crate::utils::Color;
use crate::utils::errores::error_piezas_invalidas;


fn validar_tablero (ultima_fila: i8, piezas: Vec<Box<dyn Pieza>>) -> Result<Vec<Box<dyn Pieza>>,Error> {
    if ultima_fila != MAX_TABLERO || piezas.len() != MAX_PIEZAS || piezas[0].color() == piezas[1].color() {
        return Err(error_piezas_invalidas())
    }

    Ok(piezas)
}

pub fn crear_piezas(tablero: &mut String) -> Result<Vec<Box<dyn Pieza>>,Error>{
    let mut piezas: Vec<Box<dyn Pieza>> = Vec::new();
    let mut columna:i8 = 0;
    let mut fila:i8 = 0;

    for caracter in tablero.chars() {
        if  caracter == SALTO_LINEA || caracter == ESPACIO {continue};

        if caracter != VACIO {
            match crear_pieza((fila, columna), caracter){
                Err(error) => return Err(error),
                Ok(pieza) => piezas.push(pieza),
            };
        }

        columna += 1;
        if columna == MAX_TABLERO {
            columna = 0;
            fila += 1;
        }
    }

    validar_tablero(fila, piezas)
}

fn estado_pieza(pieza_1 : &Box<dyn Pieza>, pieza_2 : &Box<dyn Pieza>) -> i8{
    if !pieza_1.puede_capturar(pieza_2){ return NO_CAPTURA }

    match pieza_1.color() {
        Color::Negro => NEGRA_CAPTURA,
        Color::Blanco => BLANCA_CAPTURA
    }
}

fn estado_piezas(pieza_1 : &Box<dyn Pieza>, pieza_2 : &Box<dyn Pieza>) -> i8{
    return estado_pieza(pieza_1, pieza_2) + estado_pieza(pieza_2, pieza_1)
}

pub fn mostrar_resultado(piezas: &Vec<Box<dyn Pieza>>){
    match estado_piezas(&piezas[0], &piezas[1]){
        NO_CAPTURA => println!("{}",MENSAJE_NINGUNA),
        NEGRA_CAPTURA => println!("{}",MENSAJE_NEGRA),
        BLANCA_CAPTURA => println!("{}",MENSAJE_BLANCA),
        _ => println!("{}",MENSAJE_EMPATE)
    }
}