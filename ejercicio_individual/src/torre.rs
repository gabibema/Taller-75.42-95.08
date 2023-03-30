use crate::pieza::{Pieza, Color};

pub(crate) struct Torre {
    posicion: (usize,usize),
    color: Color
}

impl Torre{
    pub(crate) fn new(posicion: (usize, usize), color: Color) -> Self {
        Torre { color, posicion }
    }
}


impl Pieza for Torre {
    fn puede_capturar(&self, pieza: &dyn Pieza) -> bool {
        if pieza.color() == self.color(){
            return true;
        }
        return false;

    }

    fn color(&self) -> &Color {
        return &self.color;
    }

    fn posicion(&self) -> &(usize,usize){
        return &self.posicion;
    }
}