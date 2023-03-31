use std::io::Error;
use crate::utils::{Color, constantes::*};
use crate::caballo::Caballo;
use crate::torre::Torre;
use crate::alfil::Alfil;
use crate::peon::Peon;
use crate::dama::Dama;
use crate::rey::Rey;
use crate::utils::errores::error_pieza_invalida;

pub trait Pieza {
    fn puede_capturar(&self, other: &Box<dyn Pieza>) -> bool;
    fn color(&self) -> &Color;
    fn posicion(&self) -> &(i8,i8);
}

pub fn crear_pieza(posicion: (i8,i8), pieza: char) -> Result< Box<dyn Pieza>,Error> {
    let color : Color = if pieza.is_uppercase() { Color::Blanco} else { Color::Negro } ;

    match pieza {
        PEON_B      | PEON_N => Ok(Box::new(Peon::new(posicion, color ))),
        ALFIL_B     | ALFIL_N => Ok(Box::new(Alfil::new(posicion, color ))),
        CABALLO_B   | CABALLO_N => Ok(Box::new(Caballo::new(posicion, color ))),
        TORRE_B     | TORRE_N => Ok(Box::new(Torre::new(posicion, color ))),
        DAMA_B      | DAMA_N => Ok(Box::new(Dama::new(posicion, color ))),
        REY_B       | REY_N => Ok(Box::new(Rey::new(posicion, color ))),
        _ => Err(error_pieza_invalida())
    }
}

