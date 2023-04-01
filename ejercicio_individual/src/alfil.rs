use crate::pieza::Pieza;
use crate::utils::Color;

pub(crate) struct Alfil {
    posicion: (i8, i8),
    color: Color,
}

impl Alfil {
    pub(crate) fn new(posicion: (i8, i8), color: Color) -> Self {
        Alfil { color, posicion }
    }
}

impl Pieza for Alfil {
    fn puede_capturar(&self, pieza: &dyn Pieza) -> bool {
        if pieza.color() == self.color() {
            return false;
        }

        let x_diff: u8 = self.posicion().0.abs_diff(pieza.posicion().0);
        let y_diff: u8 = self.posicion().1.abs_diff(pieza.posicion().1);

        x_diff == y_diff
    }

    fn color(&self) -> &Color {
        &self.color
    }

    fn posicion(&self) -> &(i8, i8) {
        &self.posicion
    }
}

#[cfg(test)]
mod tests {
    use crate::peon::Peon;
    use super::*;

    #[test]
    fn alfil_captura() {
        let alfil = Alfil::new((1,1), Color::Negro);
        let peon = Peon::new((3,3), Color::Blanco);

        assert_eq!(alfil.puede_capturar(&peon), true);
    }

    #[test]
    fn alfil_no_captura() {
        let alfil = Alfil::new((1,2), Color::Negro);
        let peon = Peon::new((3,3), Color::Blanco);

        assert_eq!(alfil.puede_capturar(&peon), false);
    }
}

