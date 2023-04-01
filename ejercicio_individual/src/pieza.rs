use crate::alfil::Alfil;
use crate::caballo::Caballo;
use crate::dama::Dama;
use crate::peon::Peon;
use crate::rey::Rey;
use crate::torre::Torre;
use crate::utils::errores::error_pieza_invalida;
use crate::utils::{constantes::*, Color};
use std::io::Error;

pub trait Pieza {
    fn puede_capturar(&self, other: &dyn Pieza) -> bool;
    fn color(&self) -> &Color;
    fn posicion(&self) -> &(i8, i8);
}

pub fn crear_pieza(posicion: (i8, i8), pieza: char) -> Result<Box<dyn Pieza>, Error> {
    let color: Color = if pieza.is_uppercase() {
        Color::Blanco
    } else {
        Color::Negro
    };

    match pieza {
        PEON_B | PEON_N => Ok(Box::new(Peon::new(posicion, color))),
        ALFIL_B | ALFIL_N => Ok(Box::new(Alfil::new(posicion, color))),
        CABALLO_B | CABALLO_N => Ok(Box::new(Caballo::new(posicion, color))),
        TORRE_B | TORRE_N => Ok(Box::new(Torre::new(posicion, color))),
        DAMA_B | DAMA_N => Ok(Box::new(Dama::new(posicion, color))),
        REY_B | REY_N => Ok(Box::new(Rey::new(posicion, color))),
        _ => Err(error_pieza_invalida()),
    }
}

pub(crate) struct Vacio {
    posicion: (i8, i8),
    color: Color,
}

impl Vacio {
    pub(crate) fn new() -> Box<Self> {
        Box::new(Vacio {
            posicion: (0, 0),
            color: Color::Negro,
        })
    }
}

impl Pieza for Vacio {
    fn puede_capturar(&self, _pieza: &dyn Pieza) -> bool {
        false
    }
    fn color(&self) -> &Color {
        &self.color
    }
    fn posicion(&self) -> &(i8, i8) {
        &self.posicion
    }
}
