use crate::own_crates::fecha::Fecha;
use std::collections::HashMap;
use std::fmt::{self};

#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(dead_code)]
pub enum ErroresApp {
    DatosInvalidos,
    MetodoNoDisponible,
    MontoInsuficiente,
    MejorSuscripcionDisponible,
    NoActiva,
    UsuarioNoExiste,
    UsuarioSinSuscripciones,
    UsuarioSinSuscripcionActiva,
}

impl fmt::Display for ErroresApp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErroresApp::DatosInvalidos => write!(f, "No fue posible validar sus datos"),
            ErroresApp::MetodoNoDisponible => write!(f, "Método de pago no disponible"),
            ErroresApp::MontoInsuficiente => write!(f, "Monto insuficiente para procesar el pago"),
            ErroresApp::MejorSuscripcionDisponible => {
                write!(f, "Usted tiene la mejor suscripcion disponible.")
            }
            ErroresApp::NoActiva => write!(f, "No puede cancelar una suscripcion inactiva!"),
            ErroresApp::UsuarioNoExiste => write!(f, "El usuario no existe"),
            ErroresApp::UsuarioSinSuscripciones => {
                write!(f, "El usuario no registra suscripciones")
            }
            ErroresApp::UsuarioSinSuscripcionActiva => {
                write!(f, "El usuario no registra suscripciones activas")
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(dead_code)]
pub enum TipoSuscripcion {
    BASICA,
    CLASICA,
    SUPER,
}
#[allow(dead_code)]
impl TipoSuscripcion {
    pub fn superior(&self) -> &TipoSuscripcion {
        match self {
            TipoSuscripcion::BASICA => &TipoSuscripcion::CLASICA,
            TipoSuscripcion::CLASICA => &TipoSuscripcion::SUPER,
            TipoSuscripcion::SUPER => &TipoSuscripcion::SUPER,
        }
    }

    pub fn inferior(&self) -> &TipoSuscripcion {
        match self {
            TipoSuscripcion::BASICA => &TipoSuscripcion::BASICA,
            TipoSuscripcion::CLASICA => &TipoSuscripcion::BASICA,
            TipoSuscripcion::SUPER => &TipoSuscripcion::CLASICA,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            TipoSuscripcion::BASICA => "BASICA".to_string(),
            TipoSuscripcion::CLASICA => "CLASICA".to_string(),
            TipoSuscripcion::SUPER => "SUPER".to_string(),
        }
    }

    pub fn get_costo(&self) -> f64 {
        match self {
            TipoSuscripcion::BASICA => 100.0,
            TipoSuscripcion::CLASICA => 200.0,
            TipoSuscripcion::SUPER => 300.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(dead_code)]
pub enum TipoMedioPago<'a> {
    EFECTIVO(f64),
    MERCADOPAGO { cvu: &'a str },
    CREDITO { nro_cuenta: &'a str },
    TRANSFERENCIA { cbu: &'a str },
    CRIPTO { wallet: &'a str },
}
#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(dead_code)]
pub struct Suscripcion<'c> {
    nombre_usuario: &'c str,
    tipo_suscripcion: TipoSuscripcion,
    costo_mensual: f64,
    medio_pago: TipoMedioPago<'c>,
    fecha_inicio: Fecha,
    duracion_en_meses: u8,
    activa: bool,
}
#[allow(dead_code)]
impl<'c> Suscripcion<'c> {
    pub fn new(
        nombre_usuario: &'c str,
        tipo_suscripcion: TipoSuscripcion,
        medio_pago: TipoMedioPago<'c>,
        fecha_inicio: Fecha,
    ) -> Self {
        Self {
            nombre_usuario,
            tipo_suscripcion,
            costo_mensual: tipo_suscripcion.get_costo(),
            medio_pago,
            fecha_inicio,
            duracion_en_meses: 12,
            activa: true,
        }
    }

    pub fn superior(&mut self) -> &TipoSuscripcion {
        self.tipo_suscripcion.superior()
    }

    pub fn inferior(&mut self) -> &TipoSuscripcion {
        self.tipo_suscripcion.inferior()
    }

    pub fn cancelar(&mut self) {
        if self.activa {
            self.activa = false;
        }
    }

    fn activar(&mut self) {
        if !self.activa {
            self.activa = true;
        }
    }

    pub fn to_string(&self) -> String {
        self.tipo_suscripcion.to_string()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(dead_code)]
pub struct PagoElectronico<'b> {
    tipo_pago: TipoMedioPago<'b>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(dead_code)]
pub struct Usuario<'c> {
    id: u128,
    nombre_usuario: &'c str,
}
#[allow(dead_code)]
impl<'c> Usuario<'c> {
    pub fn new(id: u128, nombre: &'c str) -> Self {
        Self {
            id: id,
            nombre_usuario: nombre,
        }
    }
}

#[allow(dead_code)]
pub struct StreamingRust<'d> {
    usuarios: Vec<Usuario<'d>>,
    suscripciones: HashMap<&'d str, Vec<Suscripcion<'d>>>,
}

#[allow(dead_code)]
impl<'d> StreamingRust<'d> {
    pub fn crear_usuario(
        &mut self,
        id: u128,
        nombre_usuario: &'d str,
        tipo_suscripcion: TipoSuscripcion,
        medio_pago: TipoMedioPago<'d>,
    ) {
        let hoy = Fecha::hoy();
        let nuevo_usuario: Usuario = Usuario::new(id, nombre_usuario);
        let nueva_suscripcion: Suscripcion =
            Suscripcion::new(nombre_usuario, tipo_suscripcion, medio_pago, hoy);

        self.usuarios.push(nuevo_usuario);
        self.suscripciones
            .insert(nombre_usuario, vec![nueva_suscripcion]);
    }

    pub fn subir_suscripcion_a_usuario(
        &mut self,
        nombre_usuario: &'d str,
        medio_pago: TipoMedioPago<'d>,
    ) -> Result<TipoSuscripcion, ErroresApp> {
        //Obtengo vector de las suscripciones del usuario o elevo error
        let suscripciones = self
            .suscripciones
            .get_mut(nombre_usuario)
            .ok_or(ErroresApp::UsuarioSinSuscripciones)?;

        //Obtengo suscripcion activa
        let suscripcion_actual = suscripciones.iter_mut().find(|x| x.activa);

        match suscripcion_actual {
            Some(suscripcion_actual) => {
                suscripcion_actual.cancelar(); // Cancelo la suscripcion actual

                if suscripcion_actual.tipo_suscripcion == TipoSuscripcion::SUPER {
                    return Err(ErroresApp::MejorSuscripcionDisponible);
                } //Si el usuario tiene la mayor suscripcion posible elevo error

                let siguiente = suscripcion_actual.tipo_suscripcion.superior(); //Obtengo la siguiente suscripcion
                let hoy = Fecha::hoy();
                let nueva_suscripcion =
                    Suscripcion::new(nombre_usuario, siguiente.clone(), medio_pago, hoy); // Instancio una nueva suscripcion activa con los datos locales
                suscripciones.push(nueva_suscripcion); // Pusheo el vector con la nueva suscripcion
                Ok(nueva_suscripcion.tipo_suscripcion.clone())
            }
            None => Err(ErroresApp::UsuarioSinSuscripcionActiva),
        }
    }

    pub fn bajar_suscripcion_a_usuario(
        &mut self,
        nombre_usuario: &'d str,
        medio_pago: TipoMedioPago<'d>,
    ) -> Result<TipoSuscripcion, ErroresApp> {
        //Obtengo vector de las suscripciones del usuario o elevo error
        let suscripciones = self
            .suscripciones
            .get_mut(nombre_usuario)
            .ok_or(ErroresApp::UsuarioSinSuscripciones)?;

        //Obtengo suscripcion activa
        let suscripcion_actual = suscripciones.iter_mut().find(|x| x.activa);

        match suscripcion_actual {
            Some(suscripcion_actual) => {
                suscripcion_actual.cancelar(); // Cancelo la suscripcion actual
                if suscripcion_actual.tipo_suscripcion == TipoSuscripcion::BASICA {
                    return Ok(TipoSuscripcion::BASICA);
                } //Si el usuario tiene la mayor suscripcion posible elevo error

                let siguiente = suscripcion_actual.tipo_suscripcion.inferior(); //Obtengo la suscripcion inferior
                let hoy = Fecha::hoy();
                let nueva_suscripcion =
                    Suscripcion::new(nombre_usuario, siguiente.clone(), medio_pago, hoy); // Instancio una nueva suscripcion activa con los datos locales
                suscripciones.push(nueva_suscripcion); // Pusheo el vector con la nueva suscripcion
                Ok(nueva_suscripcion.tipo_suscripcion.clone())
            }
            None => Err(ErroresApp::UsuarioSinSuscripcionActiva),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn instanciar_plataforma() -> StreamingRust<'static> {
        StreamingRust {
            usuarios: vec![],
            suscripciones: HashMap::new(),
        }
    }

    fn mockear_medio_pago() -> TipoMedioPago<'static> {
        TipoMedioPago::EFECTIVO(999.0)
    }

    #[test]
    fn test_subir_suscripcion_correctamente() {
        let mut sistema = instanciar_plataforma();
        let usuario = "simon";
        sistema.crear_usuario(1, usuario, TipoSuscripcion::BASICA, mockear_medio_pago());

        let resultado = sistema.subir_suscripcion_a_usuario(usuario, mockear_medio_pago());
        assert_eq!(resultado.unwrap(), TipoSuscripcion::CLASICA);
    }

    #[test]
    fn test_subir_suscripcion_con_superior_maxima() {
        let mut sistema = instanciar_plataforma();
        let usuario = "simon";
        sistema.crear_usuario(1, usuario, TipoSuscripcion::SUPER, mockear_medio_pago());

        let resultado = sistema.subir_suscripcion_a_usuario(usuario, mockear_medio_pago());
        assert_eq!(
            resultado.unwrap_err(),
            ErroresApp::MejorSuscripcionDisponible
        );
    }

    #[test]
    fn test_bajar_suscripcion_correctamente() {
        let mut sistema = instanciar_plataforma();
        let usuario = "simon";
        sistema.crear_usuario(1, usuario, TipoSuscripcion::SUPER, mockear_medio_pago());

        let resultado = sistema.bajar_suscripcion_a_usuario(usuario, mockear_medio_pago());
        assert_eq!(resultado.unwrap(), TipoSuscripcion::CLASICA);
    }

    #[test]
    fn test_bajar_suscripcion_minima() {
        let mut sistema = instanciar_plataforma();
        let usuario = "simon";
        sistema.crear_usuario(1, usuario, TipoSuscripcion::BASICA, mockear_medio_pago());

        let resultado = sistema.bajar_suscripcion_a_usuario(usuario, mockear_medio_pago());
        assert_eq!(resultado.unwrap(), TipoSuscripcion::BASICA);
    }

    #[test]
    fn test_error_usuario_no_registrado() {
        let mut sistema = instanciar_plataforma();
        let resultado = sistema.subir_suscripcion_a_usuario("fantasma", mockear_medio_pago());
        assert_eq!(resultado.unwrap_err(), ErroresApp::UsuarioSinSuscripciones);
    }
}
