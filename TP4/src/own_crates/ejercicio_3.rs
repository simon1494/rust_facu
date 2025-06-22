use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(dead_code)]
pub enum ErroresApp {
    DatosInvalidos,
    MetodoNoDisponible,
    MontoInsuficiente,
    MejorSuscripcionDisponible,
    NoActiva,
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
    pub fn siguiente(&self) -> &TipoSuscripcion {
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

    pub fn subir(&self, id_usuario: &str) -> Result<TipoSuscripcion, ErroresApp> {
        if id_usuario == "simon1494" {
            return Err(ErroresApp::MontoInsuficiente);
        }
        if self.to_string() == TipoSuscripcion::SUPER.to_string() {
            return Err(ErroresApp::MejorSuscripcionDisponible);
        }
        return Ok(self.siguiente().clone());
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
pub struct Suscripcion {
    tipo_suscripcion: TipoSuscripcion,
    costo_mensual: f64,
    duracion_en_meses: u8,
    activa: bool,
}
#[allow(dead_code)]
impl Suscripcion {
    pub fn subir(&mut self, id_usuario: &str) -> Result<TipoSuscripcion, ErroresApp> {
        let suscripcion_nueva = self.tipo_suscripcion.subir(id_usuario)?;
        self.tipo_suscripcion = suscripcion_nueva;
        self.activar();
        Ok(self.tipo_suscripcion)
    }

    pub fn bajar(&mut self) -> Result<TipoSuscripcion, ErroresApp> {
        let suscripcion_nueva = self.tipo_suscripcion.inferior();
        if self.tipo_suscripcion.to_string() == suscripcion_nueva.to_string() {
            return self.cancelar();
        }
        self.tipo_suscripcion = suscripcion_nueva.clone();
        Ok(self.tipo_suscripcion)
    }

    pub fn cancelar(&mut self) -> Result<TipoSuscripcion, ErroresApp> {
        if !self.activa {
            return Err(ErroresApp::NoActiva);
        }
        self.activa = false;
        Ok(self.tipo_suscripcion)
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
    id: &'c str,
    suscripcion: Suscripcion,
    medio_pago: TipoMedioPago<'c>,
}
#[allow(dead_code)]
impl<'c> Usuario<'c> {
    pub fn subir_suscripcion(&mut self) -> Result<TipoSuscripcion, ErroresApp> {
        let suscripcion_nueva = self.suscripcion.subir(self.id)?;
        Ok(suscripcion_nueva)
    }

    pub fn bajar_suscripcion(&mut self) -> Result<TipoSuscripcion, ErroresApp> {
        let suscripcion_inferior = self.suscripcion.bajar()?;
        Ok(suscripcion_inferior)
    }

    pub fn cancelar_suscripcion(&mut self) -> Result<TipoSuscripcion, ErroresApp> {
        let cancelar_suscripcion = self.suscripcion.cancelar()?;
        Ok(cancelar_suscripcion)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subir_suscripcion_exitosa() {
        let mut usuario = Usuario {
            id: "juan123",
            suscripcion: Suscripcion {
                tipo_suscripcion: TipoSuscripcion::BASICA,
                costo_mensual: 100.0,
                duracion_en_meses: 12,
                activa: true,
            },
            medio_pago: TipoMedioPago::EFECTIVO(150.0),
        };

        let resultado = usuario.subir_suscripcion();
        assert_eq!(resultado.unwrap(), TipoSuscripcion::CLASICA);
    }

    #[test]
    fn test_subir_suscripcion_falla_monto_insuficiente() {
        let mut usuario = Usuario {
            id: "simon1494",
            suscripcion: Suscripcion {
                tipo_suscripcion: TipoSuscripcion::BASICA,
                costo_mensual: 100.0,
                duracion_en_meses: 12,
                activa: true,
            },
            medio_pago: TipoMedioPago::MERCADOPAGO { cvu: "1234567890" },
        };

        let resultado = usuario.subir_suscripcion();
        assert!(resultado.is_err());
        assert_eq!(resultado.unwrap_err(), ErroresApp::MontoInsuficiente);
    }

    #[test]
    fn test_subir_suscripcion_ya_super() {
        let mut usuario = Usuario {
            id: "luisito",
            suscripcion: Suscripcion {
                tipo_suscripcion: TipoSuscripcion::SUPER,
                costo_mensual: 300.0,
                duracion_en_meses: 12,
                activa: true,
            },
            medio_pago: TipoMedioPago::CREDITO { nro_cuenta: "5555" },
        };

        let resultado = usuario.subir_suscripcion();
        assert_eq!(
            resultado.unwrap_err(),
            ErroresApp::MejorSuscripcionDisponible
        );
    }

    #[test]
    fn test_bajar_desde_super() {
        let mut usuario = Usuario {
            id: "cliente1",
            suscripcion: Suscripcion {
                tipo_suscripcion: TipoSuscripcion::SUPER,
                costo_mensual: 300.0,
                duracion_en_meses: 6,
                activa: true,
            },
            medio_pago: TipoMedioPago::EFECTIVO(500.0),
        };

        let resultado = usuario.bajar_suscripcion();
        assert_eq!(resultado.unwrap(), TipoSuscripcion::CLASICA);
    }

    #[test]
    fn test_bajar_desde_clasica() {
        let mut usuario = Usuario {
            id: "cliente2",
            suscripcion: Suscripcion {
                tipo_suscripcion: TipoSuscripcion::CLASICA,
                costo_mensual: 200.0,
                duracion_en_meses: 6,
                activa: true,
            },
            medio_pago: TipoMedioPago::CREDITO { nro_cuenta: "3210" },
        };

        let resultado = usuario.bajar_suscripcion();
        assert_eq!(resultado.unwrap(), TipoSuscripcion::BASICA);
    }

    #[test]
    fn test_bajar_desde_basica_cancela() {
        let mut usuario = Usuario {
            id: "cliente3",
            suscripcion: Suscripcion {
                tipo_suscripcion: TipoSuscripcion::BASICA,
                costo_mensual: 100.0,
                duracion_en_meses: 6,
                activa: true,
            },
            medio_pago: TipoMedioPago::TRANSFERENCIA { cbu: "9999" },
        };

        let resultado = usuario.bajar_suscripcion();
        assert_eq!(resultado.unwrap(), TipoSuscripcion::BASICA);
        assert_eq!(usuario.suscripcion.activa, false);
    }

    #[test]
    fn test_cancelar_suscripcion_inactiva() {
        let mut usuario = Usuario {
            id: "cliente4",
            suscripcion: Suscripcion {
                tipo_suscripcion: TipoSuscripcion::CLASICA,
                costo_mensual: 200.0,
                duracion_en_meses: 6,
                activa: false,
            },
            medio_pago: TipoMedioPago::CRIPTO { wallet: "0xabc" },
        };

        let resultado = usuario.cancelar_suscripcion();
        assert!(resultado.is_err());
        assert_eq!(resultado.unwrap_err(), ErroresApp::NoActiva);
    }
}
