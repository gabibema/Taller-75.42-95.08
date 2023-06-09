use crate::pieza::Pieza;
use crate::utils::Color;

pub(crate) struct Torre {
    posicion: (i8, i8),
    color: Color,
}

impl Torre {
    pub(crate) fn new(posicion: (i8, i8), color: Color) -> Self {
        Torre { color, posicion }
    }
}

///Implementación de la pieza Torre con su movimiento de captura correspondiente
impl Pieza for Torre {
    fn puede_capturar(&self, pieza: &dyn Pieza) -> bool {
        if pieza.color() == self.color() {
            return false;
        }

        let x_diff: u8 = self.posicion().0.abs_diff(pieza.posicion().0);
        let y_diff: u8 = self.posicion().1.abs_diff(pieza.posicion().1);

        match (x_diff, y_diff) {
            (0, _) => true,
            (_, 0) => true,
            (_, _) => false,
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
    use crate::peon::Peon;

    #[test]
    fn torre_captura() {
        let torre = Torre::new((0, 0), Color::Blanco);
        let peon = Peon::new((0, 4), Color::Negro);

        assert_eq!(torre.puede_capturar(&peon), true);
    }

    #[test]
    fn torre_no_captura() {
        let torre = Torre::new((0, 0), Color::Blanco);
        let peon = Peon::new((1, 2), Color::Negro);

        assert_eq!(torre.puede_capturar(&peon), false);
    }
}
