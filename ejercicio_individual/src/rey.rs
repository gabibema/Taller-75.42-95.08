use crate::pieza::{Pieza, Color};

pub(crate) struct Rey {
    posicion: (usize,usize),
    color: Color
}


impl Rey{
    pub(crate) fn new(posicion: (usize, usize), color: Color) -> Self {
        Rey { posicion, color }
    }
}


impl Pieza for Rey {
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