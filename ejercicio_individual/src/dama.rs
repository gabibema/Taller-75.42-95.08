use crate::pieza::Pieza;
use crate::utils::Color;

pub(crate) struct Dama {
    posicion: (i8,i8),
    color: Color
}

impl Dama{
    pub(crate) fn new(posicion: (i8,i8), color: Color) -> Self{
        Dama { color, posicion }
    }
}


impl Pieza for Dama {
    fn puede_capturar(&self, pieza: &Box<dyn Pieza>) -> bool {
        if pieza.color() == self.color(){ return false; }

        let x_diff: u8 = self.posicion().0.abs_diff(pieza.posicion().0);
        let y_diff: u8 = self.posicion().1.abs_diff(pieza.posicion().1);

        match(x_diff, y_diff){
            (0,_) => true,
            (_, 0) => true,
            (x, y) => x == y
        }
    }

    fn color(&self) -> &Color {
        return &self.color;
    }

    fn posicion(&self) -> &(i8,i8){
        return &self.posicion;
    }
}