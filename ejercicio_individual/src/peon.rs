use crate::pieza::Pieza;
use crate::utils::Color;

pub(crate) struct Peon {
    posicion: (i8, i8),
    color: Color,
}

impl Peon {
    pub(crate) fn new(posicion: (i8, i8), color: Color) -> Self {
        Peon { color, posicion }
    }
}

///Implementación de la pieza Peón con su movimiento de captura correspondiente
impl Pieza for Peon {
    fn puede_capturar(&self, pieza: &dyn Pieza) -> bool {
        if self.color() == pieza.color() {
            return false;
        };

        let x_diff: u8 = self.posicion().0.abs_diff(pieza.posicion().0);
        let y_diff: i8 = self.posicion().1 - pieza.posicion().1;

        match self.color {
            Color::Negro => x_diff == 1 && y_diff == -1,
            Color::Blanco => x_diff == 1 && y_diff == 1,
        }
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

    #[test]
    fn peon_captura() {
        let peon = Peon::new((4, 4), Color::Negro);
        let peon2 = Peon::new((5, 5), Color::Blanco);

        assert_eq!(peon.puede_capturar(&peon2), true);
    }

    #[test]
    fn peon_no_captura() {
        let peon = Peon::new((4, 4), Color::Negro);
        let peon2 = Peon::new((5, 6), Color::Blanco);

        assert_eq!(peon.puede_capturar(&peon2), false);
    }
}
