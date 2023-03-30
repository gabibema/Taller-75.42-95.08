use crate::pieza::{Pieza, Color};

pub(crate) struct Alfil {
    posicion: (usize,usize),
    color: Color
}

impl Alfil{
    pub(crate) fn new(posicion: (usize, usize), color: Color) -> Self {
        Alfil { color, posicion }
    }
}


impl Pieza for Alfil {
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