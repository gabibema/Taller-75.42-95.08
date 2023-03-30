use crate::pieza::{Pieza, Color};

pub(crate) struct Alfil {
    posicion: (i8,i8),
    color: Color
}

impl Alfil{
    pub(crate) fn new(posicion: (i8,i8), color: Color) -> Self {
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

    fn posicion(&self) -> &(i8,i8){
        return &self.posicion;
    }
}