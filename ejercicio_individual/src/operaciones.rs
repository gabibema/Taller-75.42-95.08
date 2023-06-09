use crate::pieza::{crear_pieza, Pieza, Vacio};
use crate::utils::constantes::*;
use crate::utils::errores::error_piezas_invalidas;
use crate::utils::Color;
use std::io::Error;

///Valida que el tablero sólo contenga 2 piezas de diferente color y el tamaño sea el correcto.
fn validar_tablero(
    ultima_fila: i8,
    piezas: [Box<dyn Pieza>; MAX_PIEZAS],
    cantidad: usize,
) -> Result<[Box<dyn Pieza>; MAX_PIEZAS], Error> {
    if ultima_fila != MAX_TABLERO
        || cantidad != MAX_PIEZAS
        || piezas[0].color() == piezas[1].color()
    {
        return Err(error_piezas_invalidas());
    }

    Ok(piezas)
}

///Devuelve un vector de piezas a partir del tablero recibido.
pub fn crear_piezas(tablero: &mut str) -> Result<[Box<dyn Pieza>; MAX_PIEZAS], Error> {
    let mut piezas: [Box<dyn Pieza>; MAX_PIEZAS] = [Vacio::new(), Vacio::new()];
    let mut posicion: (i8, i8) = (0, 0);
    let mut indice: usize = 0;

    for caracter in tablero.chars() {
        if caracter == SALTO_LINEA || caracter == ESPACIO {
            continue;
        };

        if caracter != VACIO {
            if indice == MAX_PIEZAS {
                return Err(error_piezas_invalidas());
            };

            match crear_pieza(posicion, caracter) {
                Err(error) => return Err(error),
                Ok(pieza) => {
                    piezas[indice] = pieza;
                    indice += 1;
                }
            };
        }

        posicion.1 += 1;
        if posicion.1 == MAX_TABLERO {
            posicion.1 = 0;
            posicion.0 += 1;
        }
    }

    validar_tablero(posicion.0, piezas, indice)
}

///Devuelve NO_CAPTURA si la pieza 1 no puede capturar a la pieza 2,
///NEGRA_CAPTURA si la pieza 1 (negra) captura a la pieza2
///BLANCA_CAPTURA si la pieza 1 (blanca) captura a la pieza2
fn estado_pieza(pieza_1: &dyn Pieza, pieza_2: &dyn Pieza) -> i8 {
    if !pieza_1.puede_capturar(pieza_2) {
        return NO_CAPTURA;
    }

    match pieza_1.color() {
        Color::Negro => NEGRA_CAPTURA,
        Color::Blanco => BLANCA_CAPTURA,
    }
}

///Devuelve la suma de pieza1 captura a pieza2 y viceversa.
pub fn estado_piezas(pieza_1: &dyn Pieza, pieza_2: &dyn Pieza) -> i8 {
    estado_pieza(pieza_1, pieza_2) + estado_pieza(pieza_2, pieza_1)
}

///Muestra por pantalla el estado de la partida
/// MENSAJE_NINGUNA si ninguna pieza puede capturar
/// MENSAJE_EMPATE si ambas piezas pueden capturar
/// MENSAJE_NEGRA si sólo la pieza negra captura a la blanca
/// MENSAJE_BLANCA si sólo la pieza blanca captura a la negra
pub fn mostrar_resultado(piezas: &[Box<dyn Pieza>; MAX_PIEZAS]) {
    match estado_piezas(&*piezas[0], &*piezas[1]) {
        NO_CAPTURA => println!("{}", MENSAJE_NINGUNA),
        NEGRA_CAPTURA => println!("{}", MENSAJE_NEGRA),
        BLANCA_CAPTURA => println!("{}", MENSAJE_BLANCA),
        _ => println!("{}", MENSAJE_EMPATE),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::peon::Peon;
    use crate::torre::Torre;

    #[test]
    fn estado_blanca_captura() {
        let pieza1 = Peon::new((1, 1), Color::Blanco);
        let pieza2 = Torre::new((0, 0), Color::Negro);

        assert_eq!(estado_pieza(&pieza1, &pieza2), BLANCA_CAPTURA);
    }

    #[test]
    fn estado_blancas_ganan() {
        let pieza1 = Peon::new((1, 1), Color::Blanco);
        let pieza2 = Torre::new((0, 0), Color::Negro);

        assert_eq!(estado_piezas(&pieza1, &pieza2), BLANCA_CAPTURA);
    }

    #[test]
    fn estado_negras_ganan() {
        let pieza1 = Peon::new((3, 1), Color::Blanco);
        let pieza2 = Torre::new((2, 1), Color::Negro);

        assert_eq!(estado_piezas(&pieza1, &pieza2), NEGRA_CAPTURA);
    }
}
