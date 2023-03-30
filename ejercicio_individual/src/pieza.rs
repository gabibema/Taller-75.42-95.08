use std::io::{Error, ErrorKind::InvalidInput};
use crate::peon::Peon;
use crate::rey::Rey;
use crate::alfil::Alfil;
use crate::torre::Torre;
use crate::dama::Dama;
use crate::caballo::Caballo;


const REY_B: char = 'r';
const PEON_B: char = 'p';
const DAMA_B: char = 'd';
const ALFIL_B: char = 'a';
const CABALLO_B: char = 'c';
const TORRE_B: char = 't';

const REY_N: char = 'R';
const PEON_N: char = 'P';
const DAMA_N: char = 'D';
const ALFIL_N: char = 'A';
const CABALLO_N: char = 'C';
const TORRE_N: char = 'T';

pub enum Color{
    Blanco,
    Negro
}

impl PartialEq for Color {
    fn eq(&self, otro: &Self) -> bool {
        match (self, otro) {
            (Color::Blanco, Color::Blanco) => true,
            (Color::Negro, Color::Negro) => true,
            _ => false,
        }
    }
}

pub trait Pieza {
    fn puede_capturar(&self, other: &dyn Pieza) -> bool;
    fn color(&self) -> &Color;
    fn posicion(&self) -> &(i8,i8);
}

pub fn crear_pieza(posicion: (i8,i8), pieza: char) -> Result< Box<dyn Pieza>,Error> {
    let color : Color = if pieza.is_uppercase() { Color::Blanco} else { Color::Negro } ;

    match pieza {
        PEON_B | PEON_N => Ok(Box::new(Peon::new(posicion, color ))),
        REY_B | REY_N => Ok(Box::new(Rey::new(posicion, color ))),
        ALFIL_B | ALFIL_N => Ok(Box::new(Alfil::new(posicion, color ))),
        CABALLO_N | CABALLO_B => Ok(Box::new(Caballo::new(posicion, color ))),
        DAMA_B | DAMA_N => Ok(Box::new(Dama::new(posicion, color ))),
        TORRE_B | TORRE_N => Ok(Box::new(Torre::new(posicion, color ))),
        _ => Err(Error::new(InvalidInput, "[ERROR] El tablero contiene un carácter inválido"))
    }
}

