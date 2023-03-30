use crate::pieza::{Pieza, Color};

pub(crate) struct Caballo {
    posicion: (usize,usize),
    color: Color
}

impl Caballo{
    pub(crate) fn new(posicion: (usize, usize), color: Color) -> Self {
        Caballo { color, posicion }
    }
}


impl Pieza for Caballo {
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