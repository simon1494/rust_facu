use crate::ejercicios::fecha::Fecha;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{self};
use std::fs;
use std::fs::File;
use std::io::Write;

#[derive(Debug, Deserialize)]
pub struct Archivito<'a> {
    #[serde(borrow)]
    pub suscripciones: HashMap<&'a str, Vec<Suscripcion<'a>>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
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
    UsuarioExistente,
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
            ErroresApp::UsuarioExistente => {
                write!(f, "Este nombre de usuario ya existe")
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
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

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum TipoMedioPago<'a> {
    EFECTIVO(f64),
    MERCADOPAGO { cvu: &'a str },
    CREDITO { nro_cuenta: &'a str },
    TRANSFERENCIA { cbu: &'a str },
    CRIPTO { wallet: &'a str },
}
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
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
    ruta_archivo: String,
}

#[allow(dead_code)]
impl<'d> StreamingRust<'d> {
    pub fn new(ruta_archivo: &'d str) -> Self {
        let mut sistema = StreamingRust {
            usuarios: vec![],
            suscripciones: HashMap::new(),
            ruta_archivo: ruta_archivo.to_string(),
        };
        sistema.cargar_suscripciones(ruta_archivo);
        sistema
    }

    pub fn crear_usuario(
        &mut self,
        id: u128,
        nombre_usuario: &'d str,
        tipo_suscripcion: TipoSuscripcion,
        medio_pago: TipoMedioPago<'d>,
    ) -> Result<&str, ErroresApp> {
        if self.suscripciones.contains_key(nombre_usuario) {
            return Err(ErroresApp::UsuarioExistente);
        }

        let hoy = Fecha::hoy();
        let nuevo_usuario: Usuario = Usuario::new(id, nombre_usuario);
        let nueva_suscripcion: Suscripcion =
            Suscripcion::new(nombre_usuario, tipo_suscripcion, medio_pago, hoy);

        self.usuarios.push(nuevo_usuario);
        self.suscripciones
            .insert(nombre_usuario, vec![nueva_suscripcion]);

        self.guardar_suscripciones(&self.ruta_archivo);
        return Ok(nuevo_usuario.nombre_usuario);
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
                self.guardar_suscripciones(&self.ruta_archivo);
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
                self.guardar_suscripciones(&self.ruta_archivo);
                Ok(nueva_suscripcion.tipo_suscripcion.clone())
            }
            None => Err(ErroresApp::UsuarioSinSuscripcionActiva),
        }
    }

    pub fn cancelar_suscripcion_a_usuario(
        &mut self,
        nombre_usuario: &'d str,
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
                Ok(suscripcion_actual.tipo_suscripcion.clone())
            }
            None => Err(ErroresApp::UsuarioSinSuscripcionActiva),
        }
    }

    pub fn mayor_medio_pago(&self, solo_activas: bool) -> String {
        let mut conteo: HashMap<String, usize> = HashMap::new();

        self.suscripciones
            .values()
            .flat_map(|sus_vec| sus_vec.iter())
            .filter(|s| !solo_activas || s.activa)
            .for_each(|s| {
                let clave = match s.medio_pago {
                    TipoMedioPago::EFECTIVO(_) => "EFECTIVO".to_string(),
                    TipoMedioPago::MERCADOPAGO { .. } => "MERCADOPAGO".to_string(),
                    TipoMedioPago::CREDITO { .. } => "CREDITO".to_string(),
                    TipoMedioPago::TRANSFERENCIA { .. } => "TRANSFERENCIA".to_string(),
                    TipoMedioPago::CRIPTO { .. } => "CRIPTO".to_string(),
                };
                *conteo.entry(clave).or_insert(0) += 1;
            });

        conteo
            .into_iter()
            .max_by_key(|(_, cantidad)| *cantidad)
            .map(|(medio, _)| format!("{}", medio))
            .unwrap_or_else(|| "No hay suscripciones activas".to_string())
    }

    pub fn mayor_suscripcion(&self, solo_activas: bool) -> String {
        let mut conteo: HashMap<String, usize> = HashMap::new();

        self.suscripciones
            .values()
            .flat_map(|sus_vec| sus_vec.iter())
            .filter(|s| !solo_activas || s.activa)
            .for_each(|s| {
                let clave = s.tipo_suscripcion.to_string();
                *conteo.entry(clave).or_insert(0) += 1;
            });

        conteo
            .into_iter()
            .max_by_key(|(_, cantidad)| *cantidad)
            .map(|(tipo, _)| format!("{}", tipo))
            .unwrap_or_else(|| "No hay suscripciones activas".to_string())
    }

    pub fn guardar_suscripciones(&self, ruta: &str) {
        if let Ok(json) = serde_json::to_string_pretty(&self.suscripciones) {
            if let Ok(mut file) = File::create(ruta) {
                let _ = file.write_all(json.as_bytes());
            }
        }
    }

    pub fn cargar_suscripciones(&mut self, ruta: &str) {
        if let Ok(contenido) = fs::read_to_string(ruta) {
            let contenido_2 = Box::leak(contenido.into_boxed_str()); // Tengo que usar esto si no los lifetimes me rompen las pelotas
            if let Ok(mapa) =
                serde_json::from_str::<HashMap<&'d str, Vec<Suscripcion<'d>>>>(contenido_2)
            {
                self.suscripciones = mapa;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn oaaaa(ruta: &str) -> StreamingRust<'static> {
        StreamingRust {
            usuarios: vec![],
            suscripciones: HashMap::new(),
            ruta_archivo: ruta.to_string(),
        }
    }

    fn mockear_medio_pago() -> TipoMedioPago<'static> {
        TipoMedioPago::EFECTIVO(999.0)
    }

    #[test]
    fn test_crear_usuario_valido() {
        let ruta = "test_platanata1.json";
        let mut sistema = oaaaa(ruta);
        let resultado =
            sistema.crear_usuario(1, "valido", TipoSuscripcion::CLASICA, mockear_medio_pago());
        assert_eq!(resultado.unwrap(), "valido");
        let _ = std::fs::remove_file(ruta);
    }

    #[test]
    fn test_crear_usuario_duplicado() {
        let ruta = "test_platanata2.json";
        let mut sistema = oaaaa(ruta);
        sistema
            .crear_usuario(1, "repetido", TipoSuscripcion::BASICA, mockear_medio_pago())
            .unwrap();

        let resultado =
            sistema.crear_usuario(2, "repetido", TipoSuscripcion::SUPER, mockear_medio_pago());
        assert_eq!(resultado.unwrap_err(), ErroresApp::UsuarioExistente);
        let _ = std::fs::remove_file(ruta);
    }

    #[test]
    fn test_subir_suscripcion_valido() {
        let ruta = "test_platanata3.json";
        let mut sistema = oaaaa(ruta);
        let usuario = "simon";
        sistema
            .crear_usuario(1, usuario, TipoSuscripcion::BASICA, mockear_medio_pago())
            .unwrap();

        let resultado = sistema.subir_suscripcion_a_usuario(usuario, mockear_medio_pago());
        assert_eq!(resultado.unwrap(), TipoSuscripcion::CLASICA);
        let _ = std::fs::remove_file(ruta);
    }

    #[test]
    fn test_subir_suscripcion_con_superior_maxima() {
        let ruta = "test_platanata4.json";
        let mut sistema = oaaaa(ruta);
        let usuario = "simon";
        sistema
            .crear_usuario(1, usuario, TipoSuscripcion::SUPER, mockear_medio_pago())
            .unwrap();

        let resultado = sistema.subir_suscripcion_a_usuario(usuario, mockear_medio_pago());
        assert_eq!(
            resultado.unwrap_err(),
            ErroresApp::MejorSuscripcionDisponible
        );
        let _ = std::fs::remove_file(ruta);
    }

    #[test]
    fn test_bajar_suscripcion_valido() {
        let ruta = "test_platanata5.json";
        let mut sistema = oaaaa(ruta);
        let usuario = "simon";
        sistema
            .crear_usuario(1, usuario, TipoSuscripcion::SUPER, mockear_medio_pago())
            .unwrap();

        let resultado = sistema.bajar_suscripcion_a_usuario(usuario, mockear_medio_pago());
        assert_eq!(resultado.unwrap(), TipoSuscripcion::CLASICA);
        let _ = std::fs::remove_file(ruta);
    }

    #[test]
    fn test_bajar_suscripcion_minima() {
        let ruta = "test_platanata6.json";
        let mut sistema = oaaaa(ruta);
        let usuario = "simon";
        sistema
            .crear_usuario(1, usuario, TipoSuscripcion::BASICA, mockear_medio_pago())
            .unwrap();

        let resultado = sistema.bajar_suscripcion_a_usuario(usuario, mockear_medio_pago());
        assert_eq!(resultado.unwrap(), TipoSuscripcion::BASICA);
        let _ = std::fs::remove_file(ruta);
    }

    #[test]
    fn test_error_usuario_no_registrado() {
        let ruta = "test_platanata7.json";
        let mut sistema = oaaaa(ruta);
        let resultado = sistema.subir_suscripcion_a_usuario("fantasma", mockear_medio_pago());
        assert_eq!(resultado.unwrap_err(), ErroresApp::UsuarioSinSuscripciones);
        let _ = std::fs::remove_file(ruta);
    }

    #[test]
    fn test_mayor_medio_pago_todas() {
        let ruta = "test_platanata8.json";
        let mut sistema = oaaaa(ruta);
        sistema
            .crear_usuario(
                1,
                "simon",
                TipoSuscripcion::CLASICA,
                TipoMedioPago::EFECTIVO(500.0),
            )
            .unwrap();
        sistema
            .crear_usuario(
                2,
                "luna",
                TipoSuscripcion::CLASICA,
                TipoMedioPago::CREDITO { nro_cuenta: "123" },
            )
            .unwrap();
        sistema
            .subir_suscripcion_a_usuario("simon", TipoMedioPago::EFECTIVO(999.0))
            .unwrap();
        sistema
            .subir_suscripcion_a_usuario("luna", TipoMedioPago::EFECTIVO(999.0))
            .unwrap();

        let resultado = sistema.mayor_medio_pago(false);
        assert!(resultado.contains("EFECTIVO"));
        let _ = std::fs::remove_file(ruta);
    }

    #[test]
    fn test_mayor_medio_pago_solo_activas() {
        let ruta = "test_platanata9.json";
        let mut sistema = oaaaa(ruta);
        sistema
            .crear_usuario(
                1,
                "sol",
                TipoSuscripcion::CLASICA,
                TipoMedioPago::CREDITO { nro_cuenta: "456" },
            )
            .unwrap();
        sistema
            .crear_usuario(
                2,
                "leo",
                TipoSuscripcion::CLASICA,
                TipoMedioPago::EFECTIVO(200.0),
            )
            .unwrap();
        sistema
            .subir_suscripcion_a_usuario("sol", TipoMedioPago::EFECTIVO(999.0))
            .unwrap();
        sistema
            .subir_suscripcion_a_usuario("leo", TipoMedioPago::EFECTIVO(999.0))
            .unwrap();

        let resultado = sistema.mayor_medio_pago(true);
        assert_eq!(resultado, "EFECTIVO");
        let _ = std::fs::remove_file(ruta);
    }

    #[test]
    fn test_mayor_suscripcion_todas() {
        let ruta = "test_platanata10.json";
        let mut sistema = oaaaa(ruta);
        sistema
            .crear_usuario(1, "ana", TipoSuscripcion::BASICA, mockear_medio_pago())
            .unwrap();
        sistema
            .crear_usuario(2, "carlo", TipoSuscripcion::BASICA, mockear_medio_pago())
            .unwrap();
        sistema
            .crear_usuario(3, "juan", TipoSuscripcion::BASICA, mockear_medio_pago())
            .unwrap();
        sistema
            .subir_suscripcion_a_usuario("ana", mockear_medio_pago())
            .unwrap();
        sistema
            .subir_suscripcion_a_usuario("carlo", mockear_medio_pago())
            .unwrap();

        let resultado = sistema.mayor_suscripcion(false);
        assert!(resultado.contains("BASICA"));
        let _ = std::fs::remove_file(ruta);
    }

    #[test]
    fn test_mayor_suscripcion_solo_activas() {
        let ruta = "test_platanata11.json";
        let mut sistema = oaaaa(ruta);
        sistema
            .crear_usuario(1, "maria", TipoSuscripcion::CLASICA, mockear_medio_pago())
            .unwrap();
        sistema
            .crear_usuario(2, "juan", TipoSuscripcion::CLASICA, mockear_medio_pago())
            .unwrap();
        sistema
            .subir_suscripcion_a_usuario("maria", mockear_medio_pago())
            .unwrap();
        sistema
            .subir_suscripcion_a_usuario("juan", mockear_medio_pago())
            .unwrap();

        let resultado = sistema.mayor_suscripcion(true);
        assert_eq!(resultado, "SUPER");
        let _ = std::fs::remove_file(ruta);
    }

    #[test]
    fn test_mayor_medio_pago_sin_suscripciones() {
        let ruta = "test_platanata12.json";
        let sistema = oaaaa(ruta);
        let resultado = sistema.mayor_medio_pago(true);
        assert_eq!(resultado, "No hay suscripciones activas");
        let _ = std::fs::remove_file(ruta);
    }

    #[test]
    fn test_mayor_suscripcion_sin_suscripciones() {
        let ruta = "test_platanata13.json";
        let sistema = oaaaa(ruta);
        let resultado = sistema.mayor_suscripcion(true);
        assert_eq!(resultado, "No hay suscripciones activas");
        let _ = std::fs::remove_file(ruta);
    }

    #[test]
    fn test_cancelar_suscripcion_activa() {
        let ruta = "test_platanata14.json";
        let mut sistema = oaaaa(ruta);
        let usuario = "lara";
        sistema
            .crear_usuario(1, usuario, TipoSuscripcion::CLASICA, mockear_medio_pago())
            .unwrap();

        let resultado = sistema.cancelar_suscripcion_a_usuario(usuario);
        assert_eq!(resultado.unwrap(), TipoSuscripcion::CLASICA);
        let resultado_error = sistema.cancelar_suscripcion_a_usuario(usuario);
        assert_eq!(
            resultado_error.unwrap_err(),
            ErroresApp::UsuarioSinSuscripcionActiva
        );
        let _ = std::fs::remove_file(ruta);
    }

    #[test]
    fn test_cancelar_suscripcion_sin_activa() {
        let ruta = "test_platanata15.json";
        let mut sistema = oaaaa(ruta);
        let usuario = "maxi";
        sistema
            .crear_usuario(1, usuario, TipoSuscripcion::CLASICA, mockear_medio_pago())
            .unwrap();
        sistema.cancelar_suscripcion_a_usuario(usuario).unwrap(); // la cancela

        let resultado = sistema.cancelar_suscripcion_a_usuario(usuario);
        assert_eq!(
            resultado.unwrap_err(),
            ErroresApp::UsuarioSinSuscripcionActiva
        );
        let _ = std::fs::remove_file(ruta);
    }

    #[test]
    fn test_subir_suscripcion_sin_activa() {
        let ruta = "test_platanata16.json";
        let mut sistema = oaaaa(ruta);
        let usuario = "natalia";
        sistema
            .crear_usuario(1, usuario, TipoSuscripcion::BASICA, mockear_medio_pago())
            .unwrap();
        sistema.cancelar_suscripcion_a_usuario(usuario).unwrap(); // se queda sin activa

        let resultado = sistema.subir_suscripcion_a_usuario(usuario, mockear_medio_pago());
        assert_eq!(
            resultado.unwrap_err(),
            ErroresApp::UsuarioSinSuscripcionActiva
        );
        let _ = std::fs::remove_file(ruta);
    }

    #[test]
    fn test_bajar_suscripcion_sin_activa() {
        let ruta = "test_platanata17.json";
        let mut sistema = oaaaa(ruta);
        let usuario = "enzo";
        sistema
            .crear_usuario(1, usuario, TipoSuscripcion::CLASICA, mockear_medio_pago())
            .unwrap();
        sistema.cancelar_suscripcion_a_usuario(usuario).unwrap();

        let resultado = sistema.bajar_suscripcion_a_usuario(usuario, mockear_medio_pago());
        assert_eq!(
            resultado.unwrap_err(),
            ErroresApp::UsuarioSinSuscripcionActiva
        );
        let _ = std::fs::remove_file(ruta);
    }

    #[test]
    fn test_solo_una_activa_por_usuario() {
        let ruta = "test_platanata18.json";
        let mut sistema = oaaaa(ruta);
        let usuario = "ramiro";
        sistema
            .crear_usuario(1, usuario, TipoSuscripcion::BASICA, mockear_medio_pago())
            .unwrap();

        sistema
            .subir_suscripcion_a_usuario(usuario, mockear_medio_pago())
            .unwrap();
        sistema
            .subir_suscripcion_a_usuario(usuario, mockear_medio_pago())
            .unwrap();

        let activas = sistema
            .suscripciones
            .get(usuario)
            .unwrap()
            .iter()
            .filter(|s| s.activa)
            .count();

        assert_eq!(activas, 1);
        let _ = std::fs::remove_file(ruta);
    }

    #[test]
    fn test_suscripcion_cancelar_y_activar() {
        let fecha = Fecha::hoy();
        let mut sus = Suscripcion::new("ana", TipoSuscripcion::BASICA, mockear_medio_pago(), fecha);

        assert!(sus.activa);
        sus.cancelar();
        assert!(!sus.activa);

        sus.activar();
    }

    #[test]
    fn test_guardar_y_cargar() {
        let ruta = "test_platanata19.json";
        let mut sistema = StreamingRust::new(ruta);

        let usuario = "pepito";
        let medio = mockear_medio_pago();

        sistema
            .crear_usuario(1, usuario, TipoSuscripcion::CLASICA, medio)
            .unwrap();

        sistema.guardar_suscripciones(ruta);

        let mut nuevo_sistema = StreamingRust::new(ruta);
        nuevo_sistema.cargar_suscripciones(ruta);

        let sus = nuevo_sistema.suscripciones.get(usuario).unwrap();
        assert_eq!(sus.len(), 1);
        assert_eq!(sus[0].tipo_suscripcion, TipoSuscripcion::CLASICA);

        let _ = std::fs::remove_file(ruta);
    }

    #[test]
    fn test_tipo_suscripcion_superior_inferior() {
        assert_eq!(
            TipoSuscripcion::BASICA.superior(),
            &TipoSuscripcion::CLASICA
        );
        assert_eq!(TipoSuscripcion::CLASICA.superior(), &TipoSuscripcion::SUPER);
        assert_eq!(TipoSuscripcion::SUPER.superior(), &TipoSuscripcion::SUPER);

        assert_eq!(TipoSuscripcion::SUPER.inferior(), &TipoSuscripcion::CLASICA);
        assert_eq!(
            TipoSuscripcion::CLASICA.inferior(),
            &TipoSuscripcion::BASICA
        );
        assert_eq!(TipoSuscripcion::BASICA.inferior(), &TipoSuscripcion::BASICA);
    }

    #[test]
    fn test_tipo_suscripcion_costo() {
        assert_eq!(TipoSuscripcion::BASICA.to_string(), "BASICA");
        assert_eq!(TipoSuscripcion::CLASICA.to_string(), "CLASICA");
        assert_eq!(TipoSuscripcion::SUPER.to_string(), "SUPER");

        assert_eq!(TipoSuscripcion::BASICA.get_costo(), 100.0);
        assert_eq!(TipoSuscripcion::CLASICA.get_costo(), 200.0);
        assert_eq!(TipoSuscripcion::SUPER.get_costo(), 300.0);
    }

    #[test]
    fn test_tipo_suscripcion_to_string() {
        assert_eq!(TipoSuscripcion::BASICA.to_string(), "BASICA");
        assert_eq!(TipoSuscripcion::CLASICA.to_string(), "CLASICA");
        assert_eq!(TipoSuscripcion::SUPER.to_string(), "SUPER");
    }

    #[test]
    fn test_mostrar_errores() {
        assert_eq!(
            ErroresApp::DatosInvalidos.to_string(),
            "No fue posible validar sus datos"
        );
        assert_eq!(
            ErroresApp::MetodoNoDisponible.to_string(),
            "Método de pago no disponible"
        );
        assert_eq!(
            ErroresApp::MontoInsuficiente.to_string(),
            "Monto insuficiente para procesar el pago"
        );
        assert_eq!(
            ErroresApp::MejorSuscripcionDisponible.to_string(),
            "Usted tiene la mejor suscripcion disponible."
        );
        assert_eq!(
            ErroresApp::NoActiva.to_string(),
            "No puede cancelar una suscripcion inactiva!"
        );
        assert_eq!(
            ErroresApp::UsuarioNoExiste.to_string(),
            "El usuario no existe"
        );
        assert_eq!(
            ErroresApp::UsuarioSinSuscripciones.to_string(),
            "El usuario no registra suscripciones"
        );
        assert_eq!(
            ErroresApp::UsuarioSinSuscripcionActiva.to_string(),
            "El usuario no registra suscripciones activas"
        );
        assert_eq!(
            ErroresApp::UsuarioExistente.to_string(),
            "Este nombre de usuario ya existe"
        );
    }
}
