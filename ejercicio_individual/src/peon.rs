use crate::pieza::Color;
use crate::pieza::Pieza;

pub(crate) struct Peon {
     posicion: (usize, usize),
     color: Color
}

impl Peon{
    pub(crate) fn new(posicion: (usize, usize), color: Color) -> Self {
        Peon { color, posicion }
    }
}

impl Pieza for Peon {
    fn puede_capturar(&self, pieza: &dyn Pieza) -> bool {

        if self.color == *pieza.color() { return false };
        return true;
        /*
        match self.color {
            Color::White => ox == x - 1 && (oy == y - 1 || oy == y + 1),
            Color::Black => ox == x + 1 && (oy == y - 1 || oy == y + 1),
        }*/
    }

    fn color(&self) -> &Color {
        return &self.color;
    }

    fn posicion(&self) -> &(usize,usize){
        return &self.posicion;
    }
}