use crate::pieza::Pieza;
use crate::utils::Color;

pub(crate) struct Dama {
    posicion: (i8, i8),
    color: Color,
}

impl Dama {
    pub(crate) fn new(posicion: (i8, i8), color: Color) -> Self {
        Dama { color, posicion }
    }
}

impl Pieza for Dama {
    fn puede_capturar(&self, pieza: &dyn Pieza) -> bool {
        if pieza.color() == self.color() {
            return false;
        }

        let x_diff: u8 = self.posicion().0.abs_diff(pieza.posicion().0);
        let y_diff: u8 = self.posicion().1.abs_diff(pieza.posicion().1);

        match (x_diff, y_diff) {
            (0, _) => true,
            (_, 0) => true,
            (x, y) => x == y,
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
    fn dama_captura() {
        let dama = Dama::new((2, 2), Color::Negro);
        let peon = Peon::new((4, 4), Color::Blanco);
        let peon2 = Peon::new((2, 4), Color::Blanco);
        let peon3 = Peon::new((4, 2), Color::Blanco);

        assert_eq!(dama.puede_capturar(&peon), true);
        assert_eq!(dama.puede_capturar(&peon2), true);
        assert_eq!(dama.puede_capturar(&peon3), true);
    }

    #[test]
    fn dama_no_captura() {
        let dama = Dama::new((1, 2), Color::Negro);
        let peon = Peon::new((3, 3), Color::Blanco);

        assert_eq!(dama.puede_capturar(&peon), false);
    }
}
