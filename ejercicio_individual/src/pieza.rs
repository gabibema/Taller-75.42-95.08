
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

pub enum Pieza{
    Peon {posicion: (u8,u8), color: Color},
    Rey {posicion: (u8,u8), color: Color},
    Alfil {posicion: (u8,u8), color: Color},
    Caballo {posicion: (u8,u8), color: Color},
    Dama {posicion: (u8,u8), color: Color},
    Torre {posicion: (u8,u8), color: Color},
    Vacia
}

pub fn crear_pieza(posicion: (u8,u8), pieza: char) -> Pieza {
    let color : Color = if pieza.is_uppercase() { Color::Blanco} else { Color::Negro } ;

    match pieza {
        PEON_B | PEON_N => Pieza::Peon { posicion, color },
        REY_B | REY_N => Pieza::Rey { posicion, color },
        ALFIL_B | ALFIL_N => Pieza::Alfil { posicion, color },
        CABALLO_N | CABALLO_B => Pieza::Caballo { posicion, color },
        DAMA_B | DAMA_N => Pieza::Dama { posicion, color },
        TORRE_B | TORRE_N => Pieza::Torre { posicion, color },
        _ => Pieza::Vacia
    }
}

