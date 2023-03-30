use crate::pieza::Color;
use crate::pieza::Captura;



pub(crate) struct Peon {
    posicion: (u8, u8),
    color: Color
}

impl Captura for Peon {
    fn puede_capturar(&self, pieza: &dyn Captura) -> bool {

        if self.color == pieza.color { return false };
        /*
        match self.color {
            Color::White => ox == x - 1 && (oy == y - 1 || oy == y + 1),
            Color::Black => ox == x + 1 && (oy == y - 1 || oy == y + 1),
        }*/
    }

    fn color(&self) -> &Color {
        return &self.color;
    }
}