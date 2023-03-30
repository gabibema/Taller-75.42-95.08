use crate::pieza::{Pieza, Color};

pub(crate) struct Caballo {
    posicion: (i8,i8),
    color: Color
}

impl Caballo{
    pub(crate) fn new(posicion: (i8,i8), color: Color) -> Self {
        Caballo { color, posicion }
    }
}


impl Pieza for Caballo {
    fn puede_capturar(&self, pieza: &dyn Pieza) -> bool {
        if pieza.color() == self.color(){ return false; }

        let x_diff: u8 = self.posicion().0.abs_diff(pieza.posicion().0);
        let y_diff: u8 = self.posicion().1.abs_diff(pieza.posicion().1);

        match(x_diff, y_diff){
            (x, y) => (x == 1 && y == 2) || (x == 2 && y == 1)
        }
    }

    fn color(&self) -> &Color {
        return &self.color;
    }

    fn posicion(&self) -> &(i8,i8){
        return &self.posicion;
    }
}