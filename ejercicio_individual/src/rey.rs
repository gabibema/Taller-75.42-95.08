use crate::pieza::Pieza;
use crate::utils::Color;

pub(crate) struct Rey {
    posicion: (i8,i8),
    color: Color
}


impl Rey{
    pub(crate) fn new(posicion: (i8,i8), color: Color) -> Self {
        Rey {posicion, color }
    }
}


impl Pieza for Rey {
    fn puede_capturar(&self, pieza: &Box<dyn Pieza>) -> bool {
        if pieza.color() == self.color(){ return false; }

        let x_diff: u8 = self.posicion().0.abs_diff(pieza.posicion().0);
        let y_diff: u8 = self.posicion().1.abs_diff(pieza.posicion().1);

        x_diff <= 1 && y_diff <= 1
    }

    fn color(&self) -> &Color {
        &self.color
    }

    fn posicion(&self) -> &(i8,i8){
        &self.posicion
    }
}

