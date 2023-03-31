use crate::pieza::Pieza;
use crate::utils::Color;

pub(crate) struct Peon {
     posicion: (i8,i8),
     color: Color
}

impl Peon{
    pub(crate) fn new(posicion: (i8,i8), color: Color) -> Self {
        Peon { color, posicion }
    }
}

impl Pieza for Peon {
    fn puede_capturar(&self, pieza: &Box<dyn Pieza>) -> bool {
        if self.color() == pieza.color() { return false };

        let x_diff: u8 = self.posicion().0.abs_diff(pieza.posicion().0);
        let y_diff: i8 = self.posicion().1 - pieza.posicion().1;

        match self.color {
            Color::Negro => x_diff == 1 && y_diff == 1,
            Color::Blanco => x_diff == 1 && y_diff == -1,
        }
    }

    fn color(&self) -> &Color {
        return &self.color;
    }

    fn posicion(&self) -> &(i8,i8){
        return &self.posicion;
    }
}