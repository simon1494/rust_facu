use chrono::prelude::*;
use std::collections::HashMap;
use std::fmt::{self};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Fecha {
    dia: u32,
    mes: u32,
    ano: u32,
}
#[allow(dead_code)]
impl Fecha {
    pub fn new(dia: u32, mes: u32, ano: u32) -> Fecha {
        return Fecha { dia, mes, ano };
    }

    pub fn hoy() -> Self {
        let mut f = Fecha::new(1, 1, 1999);
        f.setear_hoy();
        f
    }

    pub fn es_fecha_valida(&self) -> bool {
        //Se chequean limites generales
        if !(1..=31).contains(&self.dia) || !(1..=13).contains(&self.mes) {
            return false;
        }
        //Condicion si el mes no es febrero
        if self.mes != 2 {
            //Si es mes largo
            if self.es_mes_corto() && self.dia > 30 {
                return false;
            //Si es mes corto
            } else if self.dia > 31 {
                return false;
            }
        //Condicion si el mes es febrero
        } else {
            // Si no es bisiesto y tiene 29
            if self.dia > 28 && !self.es_bisiesto() {
                return false;
            }
        }
        return true;
    }

    pub fn es_mes_corto(&self) -> bool {
        return [4, 6, 9, 11].contains(&self.mes);
    }

    pub fn es_bisiesto(&self) -> bool {
        return (self.ano % 4 == 0 && self.ano % 100 != 0) || (self.ano % 400 == 0);
    }

    pub fn es_mayor(&self, otra_fecha: Fecha) -> bool {
        if self.ano > otra_fecha.ano {
            return true;
        } else if self.mes > otra_fecha.mes {
            return true;
        } else if self.dia > otra_fecha.dia {
            return true;
        }
        return false;
    }

    pub fn restar_dias(&mut self, mut cantidad_dias: i32) {
        while cantidad_dias != 0 {
            self.dia -= 1;
            cantidad_dias -= 1;

            if self.dia == 0 {
                self.mes -= 1;
                if self.mes == 0 {
                    self.ano -= 1;
                    self.mes = 12;
                }
                self.dia = 30;
                if self.mes == 2 {
                    if self.es_bisiesto() {
                        self.dia -= 1;
                    } else {
                        self.dia -= 2;
                    }
                } else if !self.es_mes_corto() {
                    self.dia += 1;
                }
            }
        }
    }

    pub fn sumar_dias(&mut self, mut cantidad_dias: u32) {
        while cantidad_dias != 0 {
            let mut lim_dias = 30;
            if self.mes != 2 {
                if !self.es_mes_corto() {
                    lim_dias += 1;
                }
            } else {
                if !self.es_bisiesto() {
                    lim_dias -= 2
                } else {
                    lim_dias -= 1;
                }
            }

            while self.dia <= lim_dias {
                self.dia += 1;
                cantidad_dias -= 1;
                if cantidad_dias == 0 {
                    break;
                }
            }

            if cantidad_dias == 0 {
                break;
            }

            self.mes += 1;
            if self.mes == 13 {
                self.ano += 1;
                self.mes = 1;
            }
            self.dia = 1;
        }
    }

    pub fn set_fecha(&mut self, dia: u32, mes: u32, ano: u32) {
        self.dia = dia;
        self.mes = mes;
        self.ano = ano;
    }

    pub fn setear_hoy(&mut self) {
        let ahora = Local::now().date_naive();
        self.dia = ahora.day();
        self.mes = ahora.month();
        self.ano = ahora.year() as u32;
    }

    pub fn to_string(&self) -> String {
        format!("{}/{}/{}", self.dia, self.mes, self.ano)
    }
}

impl fmt::Display for Fecha {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}/{}", self.dia, self.mes, self.ano)
    }
}

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
    UsuarioExistente,
    SinSuscripcionesActivas,
    SinSuscripciones,
    SinMediosPagoActivos,
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
            ErroresApp::SinSuscripciones => {
                write!(f, "La plataforma no tiene suscripciones")
            }
            ErroresApp::SinSuscripcionesActivas => {
                write!(f, "No hay suscripciones activas")
            }
            ErroresApp::SinMediosPagoActivos => {
                write!(
                    f,
                    "No se encontraron suscripciones activas con medios de pago"
                )
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

    pub fn mayor_medio_pago(&self, solo_activas: bool) -> Result<String, ErroresApp> {
        // Compruebo que existan suscripciones en la plataforma o propago error custom
        if self.suscripciones.is_empty() {
            return Err(ErroresApp::SinSuscripciones);
        }

        // Instancio hashmap de conteo y lo populo
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

        // Si estoy contando medios de pago de suscripciones activas, compruebo que existan suscripciones activas, si no, propago error
        if conteo.is_empty() && solo_activas {
            return Err(ErroresApp::SinSuscripcionesActivas);
        }

        // Obtengo la clave del medio de pago mas numeroso (ya sea activo si el flag de activa esta activado, o no activo en caso contrario)
        let mayor_medio = conteo.iter().max_by_key(|clave| clave.1).unwrap();
        Ok(mayor_medio.0.clone())
    }

    pub fn mayor_suscripcion(&self, solo_activas: bool) -> Result<String, ErroresApp> {
        // Compruebo que existan suscripciones en la plataforma o propago error custom
        if self.suscripciones.is_empty() {
            return Err(ErroresApp::SinSuscripciones);
        }

        // Instancio hashmap de conteo y lo populo
        let mut conteo: HashMap<String, usize> = HashMap::new();
        self.suscripciones
            .values()
            .flat_map(|sus_vec| sus_vec.iter())
            .filter(|s| !solo_activas || s.activa)
            .for_each(|s| {
                let clave = s.tipo_suscripcion.to_string();
                *conteo.entry(clave).or_insert(0) += 1;
            });

        // Si estoy contando suscripciones activas, compruebo que existan suscripciones activas, si no, propago error
        if conteo.is_empty() && solo_activas {
            return Err(ErroresApp::SinSuscripcionesActivas);
        }

        // Obtengo la clave de la suscripcion mas numerosa (ya sea activa si el flag de activa esta activado, o no activa en caso contrario)
        let mayor_suscripcion = conteo.iter().max_by_key(|clave| clave.1).unwrap();
        Ok(mayor_suscripcion.0.clone())
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Informe<'g> {
    nombre_usuario: &'g str,
    usuario: Usuario<'g>,
    historial: Vec<Suscripcion<'g>>,
}

#[allow(dead_code)]
impl<'d> StreamingRust<'d> {
    pub fn get_historial(&self, nombre_usuario: &'d str) -> Result<Informe<'d>, ErroresApp> {
        // Compruebo que el usuario exista o arrojo error de usuario inexistente que declaro en ErroresApp
        // Si el usuario existe, recupero sus datos que luego podria llegar a usar para el informe
        let usuario = self
            .usuarios
            .iter()
            .find(|u| u.nombre_usuario == nombre_usuario);
        if usuario.is_none() {
            return Err(ErroresApp::UsuarioNoExiste);
        };

        // Compruebo que el usuario tenga suscripciones, desenvuelvo y clono; si no, arrojo error de usuario sin suscripciones que declaro en ErroresApp
        let historial = self.suscripciones.get(&nombre_usuario);
        if historial.is_none() {
            return Err(ErroresApp::UsuarioSinSuscripciones);
        }
        let mut historial = historial.unwrap().clone();

        // Ordeno el vector de suscripciones el campo inicio fecha y su metodo es mayor. Uso Ordering para explicarle a Rust es el orden de precedencia de una u otra.
        historial.sort_by(|a, b| {
            if a.fecha_inicio.es_mayor(b.fecha_inicio) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Less
            }
        });

        //Si esta todo ok, desenvuelvo el usuario que recupere al principio, lo clono y preparo un informe con el y con su historial
        let usuario = usuario.unwrap().clone();

        let informe = Informe {
            nombre_usuario,
            usuario,
            historial,
        };

        // Retorno informe
        Ok(informe)
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
    fn test_crear_usuario_valido() {
        let mut sistema = instanciar_plataforma();
        let resultado =
            sistema.crear_usuario(1, "valido", TipoSuscripcion::CLASICA, mockear_medio_pago());
        assert_eq!(resultado.unwrap(), "valido");
    }

    #[test]
    fn test_crear_usuario_duplicado() {
        let mut sistema = instanciar_plataforma();
        sistema
            .crear_usuario(1, "repetido", TipoSuscripcion::BASICA, mockear_medio_pago())
            .unwrap();

        let resultado =
            sistema.crear_usuario(2, "repetido", TipoSuscripcion::SUPER, mockear_medio_pago());
        assert_eq!(resultado.unwrap_err(), ErroresApp::UsuarioExistente);
    }

    #[test]
    fn test_subir_suscripcion_valido() {
        let mut sistema = instanciar_plataforma();
        let usuario = "simon";
        sistema
            .crear_usuario(1, usuario, TipoSuscripcion::BASICA, mockear_medio_pago())
            .unwrap();

        let resultado = sistema.subir_suscripcion_a_usuario(usuario, mockear_medio_pago());
        assert_eq!(resultado.unwrap(), TipoSuscripcion::CLASICA);
    }

    #[test]
    fn test_subir_suscripcion_con_superior_maxima() {
        let mut sistema = instanciar_plataforma();
        let usuario = "simon";
        sistema
            .crear_usuario(1, usuario, TipoSuscripcion::SUPER, mockear_medio_pago())
            .unwrap();

        let resultado = sistema.subir_suscripcion_a_usuario(usuario, mockear_medio_pago());
        assert_eq!(
            resultado.unwrap_err(),
            ErroresApp::MejorSuscripcionDisponible
        );
    }

    #[test]
    fn test_bajar_suscripcion_valido() {
        let mut sistema = instanciar_plataforma();
        let usuario = "simon";
        sistema
            .crear_usuario(1, usuario, TipoSuscripcion::SUPER, mockear_medio_pago())
            .unwrap();

        let resultado = sistema.bajar_suscripcion_a_usuario(usuario, mockear_medio_pago());
        assert_eq!(resultado.unwrap(), TipoSuscripcion::CLASICA);
    }

    #[test]
    fn test_bajar_suscripcion_minima() {
        let mut sistema = instanciar_plataforma();
        let usuario = "simon";
        sistema
            .crear_usuario(1, usuario, TipoSuscripcion::BASICA, mockear_medio_pago())
            .unwrap();

        let resultado = sistema.bajar_suscripcion_a_usuario(usuario, mockear_medio_pago());
        assert_eq!(resultado.unwrap(), TipoSuscripcion::BASICA);
    }

    #[test]
    fn test_error_usuario_no_registrado() {
        let mut sistema = instanciar_plataforma();
        let resultado = sistema.subir_suscripcion_a_usuario("fantasma", mockear_medio_pago());
        assert_eq!(resultado.unwrap_err(), ErroresApp::UsuarioSinSuscripciones);
    }

    #[test]
    fn test_cancelar_suscripcion_activa() {
        let mut sistema = instanciar_plataforma();
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
    }

    #[test]
    fn test_cancelar_suscripcion_sin_activa() {
        let mut sistema = instanciar_plataforma();
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
    }

    #[test]
    fn test_subir_suscripcion_sin_activa() {
        let mut sistema = instanciar_plataforma();
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
    }

    #[test]
    fn test_bajar_suscripcion_sin_activa() {
        let mut sistema = instanciar_plataforma();
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
    }

    #[test]
    fn test_solo_una_activa_por_usuario() {
        let mut sistema = instanciar_plataforma();
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
    fn test_tipo_suscripcion_to_string_y_costo() {
        assert_eq!(TipoSuscripcion::BASICA.to_string(), "BASICA");
        assert_eq!(TipoSuscripcion::CLASICA.to_string(), "CLASICA");
        assert_eq!(TipoSuscripcion::SUPER.to_string(), "SUPER");

        assert_eq!(TipoSuscripcion::BASICA.get_costo(), 100.0);
        assert_eq!(TipoSuscripcion::CLASICA.get_costo(), 200.0);
        assert_eq!(TipoSuscripcion::SUPER.get_costo(), 300.0);
    }

    #[test]
    fn test_errores_app_display() {
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

    #[test]
    fn test_suscripcion_new_y_to_string() {
        let fecha = Fecha::hoy();
        let sus = Suscripcion::new("leo", TipoSuscripcion::CLASICA, mockear_medio_pago(), fecha);

        assert_eq!(sus.nombre_usuario, "leo");
        assert_eq!(sus.tipo_suscripcion, TipoSuscripcion::CLASICA);
        assert_eq!(sus.costo_mensual, 200.0);
        assert_eq!(sus.duracion_en_meses, 12);
        assert_eq!(sus.activa, true);
        assert_eq!(sus.to_string(), "CLASICA");
    }

    #[test]
    fn test_suscripcion_cancelar() {
        let fecha = Fecha::hoy();
        let mut sus = Suscripcion::new("ana", TipoSuscripcion::BASICA, mockear_medio_pago(), fecha);

        assert!(sus.activa);
        sus.cancelar();
        assert!(!sus.activa);
    }

    #[test]
    fn test_get_historial_usuario_existente_con_suscripciones() {
        let mut sistema = instanciar_plataforma();
        let usuario = "simon";
        sistema
            .crear_usuario(1, usuario, TipoSuscripcion::BASICA, mockear_medio_pago())
            .unwrap();

        let resultado = sistema.get_historial(usuario);
        assert!(resultado.is_ok());
        let informe = resultado.unwrap();
        assert_eq!(informe.nombre_usuario, usuario);
        assert_eq!(informe.usuario.nombre_usuario, usuario);
        assert!(!informe.historial.is_empty());
    }

    #[test]
    fn test_get_historial_usuario_inexistente() {
        let sistema = instanciar_plataforma();
        let resultado = sistema.get_historial("pablito");
        assert_eq!(resultado.unwrap_err(), ErroresApp::UsuarioNoExiste);
    }

    #[test]
    fn test_get_historial_usuario_existente_sin_suscripciones() {
        let usuario = "basouuura";
        let mut sistema = instanciar_plataforma();
        sistema.usuarios.push(Usuario::new(1, usuario));
        let resultado = sistema.get_historial(usuario);
        assert_eq!(resultado.unwrap_err(), ErroresApp::UsuarioSinSuscripciones);
    }

    #[test]
    fn test_get_historial_usuario_existente_con_varias_suscripciones() {
        let mut sistema = instanciar_plataforma();
        let usuario = "usuario_platudo";
        sistema
            .crear_usuario(1, usuario, TipoSuscripcion::BASICA, mockear_medio_pago())
            .unwrap();
        sistema
            .subir_suscripcion_a_usuario(usuario, mockear_medio_pago())
            .unwrap();
        sistema
            .subir_suscripcion_a_usuario(usuario, mockear_medio_pago())
            .unwrap();

        let resultado = sistema.get_historial(usuario);
        assert!(resultado.is_ok());
        let informe = resultado.unwrap();
        assert_eq!(informe.nombre_usuario, usuario);
        assert!(informe.historial.len() >= 3);

        for i in 1..informe.historial.len() {
            assert!(
                !informe.historial[i]
                    .fecha_inicio
                    .es_mayor(informe.historial[i - 1].fecha_inicio),
                "Historial no está ordenado cronológicamente"
            );
            println!("{:?}", informe)
        }
    }

    #[test]
    fn test_fecha_es_bisiesto() {
        let fecha: Fecha = Fecha::new(29, 02, 2024);
        assert_eq!(true, fecha.es_bisiesto());

        let fecha: Fecha = Fecha::new(29, 02, 2020);
        assert_eq!(true, fecha.es_bisiesto());

        let fecha: Fecha = Fecha::new(29, 02, 1900);
        assert_eq!(false, fecha.es_bisiesto());

        let fecha: Fecha = Fecha::new(29, 02, 2021);
        assert_eq!(false, fecha.es_bisiesto());
    }

    #[test]
    fn test_fecha_es_mayor() {
        // TEST 29-02-2024 > 28-02-2024
        let fecha: Fecha = Fecha::new(29, 02, 2024);
        let otra_fecha: Fecha = Fecha::new(28, 02, 2024);
        assert_eq!(true, fecha.es_mayor(otra_fecha));

        // TEST 01-02-2024 > 28-02-2024
        let fecha: Fecha = Fecha::new(1, 02, 2024);
        let otra_fecha: Fecha = Fecha::new(28, 02, 2024);
        assert_eq!(false, fecha.es_mayor(otra_fecha));

        // TEST 01-02-2024 > 28-02-1985
        let fecha: Fecha = Fecha::new(1, 02, 2024);
        let otra_fecha: Fecha = Fecha::new(28, 02, 1985);
        assert_eq!(true, fecha.es_mayor(otra_fecha));

        // TEST 01-02-2024 > 28-01-2024
        let fecha: Fecha = Fecha::new(1, 02, 2024);
        let otra_fecha: Fecha = Fecha::new(28, 01, 2024);
        assert_eq!(true, fecha.es_mayor(otra_fecha));
    }

    #[test]
    fn test_fecha_es_fecha_valida() {
        let fecha: Fecha = Fecha::new(1, 1, 2025);
        assert_eq!(true, fecha.es_fecha_valida());

        let fecha: Fecha = Fecha::new(30, 4, 1985);
        assert_eq!(true, fecha.es_fecha_valida());

        let fecha: Fecha = Fecha::new(31, 12, 2001);
        assert_eq!(true, fecha.es_fecha_valida());

        let fecha: Fecha = Fecha::new(29, 2, 2024);
        assert_eq!(true, fecha.es_fecha_valida());

        let fecha: Fecha = Fecha::new(29, 02, 1900);
        assert_eq!(false, fecha.es_fecha_valida());

        let fecha: Fecha = Fecha::new(0, 1, 2025);
        assert_eq!(false, fecha.es_fecha_valida());

        let fecha: Fecha = Fecha::new(31, 2, 2025);
        assert_eq!(false, fecha.es_fecha_valida());

        let fecha: Fecha = Fecha::new(31, 4, 2025);
        assert_eq!(false, fecha.es_fecha_valida());

        let fecha: Fecha = Fecha::new(1, 1, 2026);
        assert_eq!(true, fecha.es_fecha_valida());
    }

    #[test]
    fn test_fecha_to_string() {
        let fecha: Fecha = Fecha::new(1, 1, 2025);
        assert_eq!("1/1/2025".to_string(), fecha.to_string());
    }

    #[test]
    fn test_fecha_restar_dias() {
        let mut fecha: Fecha = Fecha::new(1, 02, 2024);
        fecha.restar_dias(5);
        assert_eq!("27/1/2024", fecha.to_string());

        let mut fecha: Fecha = Fecha::new(1, 02, 2024);
        fecha.restar_dias(600);
        assert_eq!("11/6/2022", fecha.to_string());

        let mut fecha: Fecha = Fecha::new(4, 05, 2025);
        fecha.restar_dias(5498);
        assert_eq!("15/4/2010", fecha.to_string());

        let mut fecha: Fecha = Fecha::new(1, 02, 2024);
        fecha.restar_dias(0);
        assert_eq!("1/2/2024", fecha.to_string());
    }

    #[test]
    fn test_fecha_sumar_dias() {
        let mut fecha: Fecha = Fecha::new(1, 02, 2024);
        fecha.sumar_dias(5);
        assert_eq!("6/2/2024", fecha.to_string());

        let mut fecha: Fecha = Fecha::new(4, 05, 2025);
        fecha.sumar_dias(600);
        assert_eq!("25/12/2026", fecha.to_string());

        let mut fecha: Fecha = Fecha::new(4, 05, 2025);
        fecha.sumar_dias(5498);
        assert_eq!("23/5/2040", fecha.to_string());

        let mut fecha: Fecha = Fecha::new(1, 02, 2024);
        fecha.sumar_dias(0);
        assert_eq!("1/2/2024", fecha.to_string());
    }
    #[test]

    fn test_mayor_suscripcion_con_varias_suscripciones() {
        let mut sistema = instanciar_plataforma();
        sistema
            .crear_usuario(1, "pepe", TipoSuscripcion::BASICA, mockear_medio_pago())
            .unwrap();
        sistema
            .crear_usuario(2, "pipi", TipoSuscripcion::CLASICA, mockear_medio_pago())
            .unwrap();
        sistema
            .crear_usuario(3, "popo", TipoSuscripcion::CLASICA, mockear_medio_pago())
            .unwrap();

        let resultado = sistema.mayor_suscripcion(false);
        assert_eq!(resultado.unwrap(), "CLASICA");
    }

    #[test]
    fn test_mayor_suscripcion_sin_suscripciones() {
        let sistema = instanciar_plataforma();
        let resultado = sistema.mayor_suscripcion(false);
        assert_eq!(resultado.unwrap_err(), ErroresApp::SinSuscripciones);
    }

    #[test]
    fn test_mayor_suscripcion_activa_con_suscripciones_activas() {
        let mut sistema = instanciar_plataforma();
        sistema
            .crear_usuario(1, "pepe", TipoSuscripcion::BASICA, mockear_medio_pago())
            .unwrap();
        sistema
            .crear_usuario(2, "pipi", TipoSuscripcion::CLASICA, mockear_medio_pago())
            .unwrap();
        sistema
            .crear_usuario(3, "popo", TipoSuscripcion::CLASICA, mockear_medio_pago())
            .unwrap();

        let resultado = sistema.mayor_suscripcion(true);
        assert_eq!(resultado.unwrap(), "CLASICA");
    }

    #[test]
    fn test_mayor_suscripcion_activa_sin_suscripciones_activas() {
        let mut sistema = instanciar_plataforma();
        sistema
            .crear_usuario(1, "pepe", TipoSuscripcion::BASICA, mockear_medio_pago())
            .unwrap();
        sistema
            .crear_usuario(2, "pipi", TipoSuscripcion::CLASICA, mockear_medio_pago())
            .unwrap();

        // Cancelar todas las suscripciones activas
        for usuario in &["pepe", "pipi"] {
            sistema.cancelar_suscripcion_a_usuario(usuario).unwrap();
        }

        let resultado = sistema.mayor_suscripcion(true);
        assert_eq!(resultado.unwrap_err(), ErroresApp::SinSuscripcionesActivas);
    }

    #[test]
    fn test_mayor_medio_pago_con_suscripciones() {
        let mut sistema = instanciar_plataforma();
        sistema
            .crear_usuario(
                1,
                "elber_galarga",
                TipoSuscripcion::BASICA,
                TipoMedioPago::EFECTIVO(100.0),
            )
            .unwrap();
        sistema
            .crear_usuario(
                2,
                "elber_galarga2",
                TipoSuscripcion::CLASICA,
                TipoMedioPago::CREDITO { nro_cuenta: "123" },
            )
            .unwrap();
        sistema
            .crear_usuario(
                3,
                "elber_galarga3",
                TipoSuscripcion::SUPER,
                TipoMedioPago::CREDITO { nro_cuenta: "456" },
            )
            .unwrap();

        let resultado = sistema.mayor_medio_pago(false);
        assert_eq!(resultado.unwrap(), "CREDITO");
    }

    #[test]
    fn test_mayor_medio_pago_sin_suscripciones() {
        let sistema = instanciar_plataforma();
        let resultado = sistema.mayor_medio_pago(false);
        assert_eq!(resultado.unwrap_err(), ErroresApp::SinSuscripciones);
    }

    #[test]
    fn test_mayor_medio_pago_activo_con_suscripciones_activas() {
        let mut sistema = instanciar_plataforma();
        sistema
            .crear_usuario(
                1,
                "elber_galarga",
                TipoSuscripcion::BASICA,
                TipoMedioPago::EFECTIVO(100.0),
            )
            .unwrap();
        sistema
            .crear_usuario(
                2,
                "elber_galarga2",
                TipoSuscripcion::CLASICA,
                TipoMedioPago::CREDITO { nro_cuenta: "123" },
            )
            .unwrap();
        sistema
            .crear_usuario(
                3,
                "elber_galarga3",
                TipoSuscripcion::SUPER,
                TipoMedioPago::CREDITO { nro_cuenta: "456" },
            )
            .unwrap();

        let resultado = sistema.mayor_medio_pago(true);
        assert_eq!(resultado.unwrap(), "CREDITO");
    }

    #[test]
    fn test_mayor_medio_pago_activo_sin_suscripciones_activas() {
        let mut sistema = instanciar_plataforma();
        sistema
            .crear_usuario(
                1,
                "elber_galarga",
                TipoSuscripcion::BASICA,
                TipoMedioPago::EFECTIVO(100.0),
            )
            .unwrap();
        sistema
            .crear_usuario(
                2,
                "elber_galarga2",
                TipoSuscripcion::CLASICA,
                TipoMedioPago::CREDITO { nro_cuenta: "123" },
            )
            .unwrap();

        // Cancelar todas las suscripciones activas
        for usuario in &["elber_galarga", "elber_galarga2"] {
            sistema.cancelar_suscripcion_a_usuario(usuario).unwrap();
        }

        let resultado = sistema.mayor_medio_pago(true);
        assert_eq!(resultado.unwrap_err(), ErroresApp::SinSuscripcionesActivas);
    }
}
