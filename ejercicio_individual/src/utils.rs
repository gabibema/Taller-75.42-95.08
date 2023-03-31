pub(crate) mod constantes {
    pub const REY_B: char = 'r';
    pub const PEON_B: char = 'p';
    pub const DAMA_B: char = 'd';
    pub const ALFIL_B: char = 'a';
    pub const CABALLO_B: char = 'c';
    pub const TORRE_B: char = 't';

    pub  const REY_N: char = 'R';
    pub const PEON_N: char = 'P';
    pub const DAMA_N: char = 'D';
    pub const ALFIL_N: char = 'A';
    pub const CABALLO_N: char = 'C';
    pub const TORRE_N: char = 'T';
}

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