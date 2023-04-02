pub mod alfil;
pub mod archivos;
pub mod caballo;
pub mod dama;
pub mod operaciones;
pub mod peon;
pub mod pieza;
pub mod rey;
pub mod torre;
pub mod utils;

pub use archivos::leer_archivo;
pub use operaciones::estado_piezas;
pub use operaciones::mostrar_resultado;
