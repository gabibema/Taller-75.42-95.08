pub mod constantes {
    pub const REY_B: char = 'r';
    pub const PEON_B: char = 'p';
    pub const DAMA_B: char = 'd';
    pub const ALFIL_B: char = 'a';
    pub const CABALLO_B: char = 'c';
    pub const TORRE_B: char = 't';

    pub const REY_N: char = 'R';
    pub const PEON_N: char = 'P';
    pub const DAMA_N: char = 'D';
    pub const ALFIL_N: char = 'A';
    pub const CABALLO_N: char = 'C';
    pub const TORRE_N: char = 'T';

    pub const SALTO_LINEA: char = '\n';
    pub const VACIO: char = '_';
    pub const ESPACIO: char = ' ';

    pub const MAX_TABLERO: i8 = 8;
    pub const MAX_PIEZAS: usize = 2;

    pub const NO_CAPTURA: i8 = 0;
    pub const BLANCA_CAPTURA: i8 = 1;
    pub const NEGRA_CAPTURA: i8 = 2;
    pub const AMBAS_CAPTURAN: i8 = 3;

    pub const MENSAJE_NINGUNA: char = 'P';
    pub const MENSAJE_BLANCA: char = 'B';
    pub const MENSAJE_NEGRA: char = 'N';
    pub const MENSAJE_EMPATE: char = 'E';
}


pub mod errores {
    use std::io::Error;
    use std::io::ErrorKind::{InvalidData, InvalidInput};

    pub fn error_piezas_invalidas() -> Error {
        Error::new(
            InvalidData,
            "[ERROR] El tablero debe contener dos piezas de diferente color únicamente",
        )
    }
    pub fn error_pieza_invalida() -> Error {
        Error::new(
            InvalidInput,
            "[ERROR] El tablero contiene un carácter inválido",
        )
    }
    pub fn error_path_invalido() -> Error {
        Error::new(InvalidInput, "[ERROR] No se ingresó el path del tablero.")
    }
}

pub enum Color {
    Blanco,
    Negro,
}

impl PartialEq for Color {
    fn eq(&self, otro: &Self) -> bool {
        matches!(
            (self, otro),
            (Color::Blanco, Color::Blanco) | (Color::Negro, Color::Negro)
        )
    }
}
