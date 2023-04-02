use crate::pieza::Pieza;
use crate::utils::Color;

pub(crate) struct Rey {
    posicion: (i8, i8),
    color: Color,
}

impl Rey {
    pub(crate) fn new(posicion: (i8, i8), color: Color) -> Self {
        Rey { posicion, color }
    }
}

///Implementación de la pieza Rey con su movimiento de captura correspondiente
impl Pieza for Rey {
    fn puede_capturar(&self, pieza: &dyn Pieza) -> bool {
        if pieza.color() == self.color() {
            return false;
        }

        let x_diff: u8 = self.posicion().0.abs_diff(pieza.posicion().0);
        let y_diff: u8 = self.posicion().1.abs_diff(pieza.posicion().1);

        x_diff <= 1 && y_diff <= 1
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
    use super::*;
    use crate::peon::Peon;

    #[test]
    fn rey_captura() {
        let rey = Rey::new((0, 0), Color::Blanco);
        let peon = Peon::new((0, 1), Color::Negro);

        assert_eq!(rey.puede_capturar(&peon), true);
    }

    #[test]
    fn rey_no_captura() {
        let rey = Rey::new((0, 0), Color::Blanco);
        let peon = Peon::new((0, 2), Color::Negro);

        assert_eq!(rey.puede_capturar(&peon), false);
    }
}
