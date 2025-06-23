use crate::own_crates::fecha::Fecha;
use rand::{Rng, distributions::Alphanumeric};
use std::{collections::HashMap, fmt};

#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(dead_code)]
pub enum OperacionFiat {
    FIATINGRESAR,
    FIATRETIRAR,
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(dead_code)]
pub enum CompraVentaCripto {
    CRIPTOCOMPRAR,
    CRIPTOVENDER,
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(dead_code)]
pub enum EnviarRecibirCripto {
    CRIPTORECIBIR,
    CRIPTORETIRAR,
}
#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(dead_code)]
pub enum OperacionCripto {
    COMPRAVENTA(CompraVentaCripto),
    RECIBIRRETIRAR(EnviarRecibirCripto),
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(dead_code)]
pub enum ErroresApp {
    MontoInsuficiente,
    UsuarioExistente,
    UsuarioInexistente,
    UsuarioNoValidado,
    NoExisteBalance,
    CriptoYaExiste,
    CriptoNoExiste,
    RedCriptoInvalida,
}

impl fmt::Display for ErroresApp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErroresApp::MontoInsuficiente => {
                write!(f, "Monto insuficiente para procesar la operacion")
            }
            ErroresApp::UsuarioExistente => {
                write!(f, "El usuario ya se encuentra registrado en la plataforma")
            }
            ErroresApp::UsuarioInexistente => {
                write!(f, "El usuario no encuentra registrado en la plataforma")
            }
            ErroresApp::NoExisteBalance => {
                write!(f, "El usuario no dispone de registros de balance")
            }
            ErroresApp::CriptoYaExiste => {
                write!(f, "La criptomoneda ya existe en el sistema")
            }
            ErroresApp::CriptoNoExiste => {
                write!(f, "La criptomoneda no existe en el sistema")
            }
            ErroresApp::UsuarioNoValidado => {
                write!(f, "Debe validar el usuario para realizar esta operacion")
            }
            ErroresApp::RedCriptoInvalida => {
                write!(f, "La criptomoneda seleccionada no opera en esa red")
            }
        }
    }
}

#[allow(dead_code)]
trait Detalle {
    fn detalle(&self) -> String;
}

#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)]
pub enum DetalleOperacion<'a> {
    CRIPTOCOMPRAR(&'a str, f64, f64),
    CRIPTOVENDER(&'a str, f64, f64),
    CRIPTORECIBIR(&'a str, f64, f64, &'a str),
    CRIPTORETIRAR(&'a str, f64, f64, &'a str, String),
    FIATINGRESAR(f64),
    FIATRETIRAR(f64, &'a str),
}

impl<'a> Detalle for DetalleOperacion<'a> {
    fn detalle(&self) -> String {
        match self {
            DetalleOperacion::FIATINGRESAR(monto) => {
                format!("Operacion: Ingresar dinero fiat - Monto: {}", monto)
            }
            DetalleOperacion::FIATRETIRAR(monto, medio) => {
                format!(
                    "Operacion: Retirar dinero fiat - Monto: {} - Medio: {}",
                    monto, medio
                )
            }
            DetalleOperacion::CRIPTOCOMPRAR(cripto, monto, cotizacion) => {
                format!(
                    "Operacion: Comprar criptomoneda - Criptomoneda: {} - Monto: {} - Cotizacion: {}",
                    cripto, monto, cotizacion
                )
            }
            DetalleOperacion::CRIPTOVENDER(cripto, monto, cotizacion) => {
                format!(
                    "Operacion: Vender criptomoneda - Criptomoneda: {} - Monto: {} - Cotizacion: {}",
                    cripto, monto, cotizacion
                )
            }
            DetalleOperacion::CRIPTORETIRAR(cripto, monto, cotizacion, blockchain, hash) => {
                format!(
                    "Operacion: Retirar criptomoneda - Criptomoneda: {} - Monto: {} - Cotizacion: {} - Blockchain: {} - Hash: {}",
                    cripto, monto, cotizacion, blockchain, hash
                )
            }
            DetalleOperacion::CRIPTORECIBIR(cripto, monto, cotizacion, blockchain) => {
                format!(
                    "Operacion: Recibir criptomoneda - Criptomoneda: {} - Monto: {} - Cotizacion: {} - Blockchain: {}",
                    cripto, monto, cotizacion, blockchain
                )
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)]
pub struct Operacion<'d> {
    id: String,
    fecha: Fecha,
    usuario: u128,
    tipo: DetalleOperacion<'d>,
}

impl<'a> fmt::Display for Operacion<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Fecha: {} - Usuario: {} - DETALLE -> [{}]",
            self.fecha,
            self.usuario,
            self.tipo.detalle()
        )
    }
}

#[allow(dead_code)]
impl<'a> Operacion<'a> {
    pub fn new(usuario: u128, tipo: DetalleOperacion<'a>) -> Self {
        fn generar_random_id(longitud: usize) -> String {
            let random_str: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(longitud)
                .map(char::from)
                .collect();
            random_str
        }

        let fecha = Fecha::hoy();
        let id = generar_random_id(10);
        Operacion {
            id,
            fecha,
            usuario,
            tipo,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(dead_code)]
pub struct Usuario<'c> {
    nombre: &'c str,
    apellido: &'c str,
    email: &'c str,
    dni: u128,
    validado: bool,
}

impl<'c> Usuario<'c> {
    pub fn new(nombre: &'c str, apellido: &'c str, email: &'c str, dni: u128) -> Usuario<'c> {
        Usuario {
            nombre,
            apellido,
            email,
            dni,
            validado: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)]
pub struct Balance<'s> {
    id_usuario: u128,
    balance_criptos: HashMap<&'s str, f64>,
    balance_fiat: f64,
}

impl<'s> Balance<'s> {
    pub fn new(id_usuario: u128, criptomonedas: Vec<&'s str>) -> Balance<'s> {
        let balance_criptos: HashMap<&'s str, f64> = criptomonedas
            .into_iter()
            .map(|clave| (clave, 0.0))
            .collect();

        let balance_fiat = 0.0;

        Balance {
            id_usuario,
            balance_criptos,
            balance_fiat,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)]
pub struct Blockchain<'g> {
    nombre: &'g str,
    prefijo: &'g str,
}

#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)]
pub struct Criptomoneda<'f> {
    nombre: &'f str,
    prefijo: &'f str,
    cotizacion: f64,
    blockchain_disponibles: HashMap<&'f str, Blockchain<'f>>,
}

#[allow(dead_code)]
pub struct Plataforma<'p> {
    usuarios: HashMap<u128, Usuario<'p>>,
    balances: HashMap<u128, Balance<'p>>,
    criptomonedas: HashMap<&'p str, Criptomoneda<'p>>,
    operaciones: Vec<Operacion<'p>>,
}

#[allow(dead_code)]
impl<'p> Plataforma<'p> {
    pub fn new() -> Self {
        let usuarios: HashMap<u128, Usuario<'p>> = HashMap::new();
        let operaciones: Vec<Operacion<'p>> = Vec::new();
        let criptomonedas: HashMap<&'p str, Criptomoneda<'p>> = HashMap::new();
        let balances: HashMap<u128, Balance<'p>> = HashMap::new();

        Plataforma {
            usuarios,
            balances,
            operaciones,
            criptomonedas,
        }
    }

    pub fn agregar_criptomoneda(
        &mut self,
        criptomoneda: Criptomoneda<'p>,
    ) -> Result<&'p str, ErroresApp> {
        let prefijo = criptomoneda.prefijo;
        if !self.criptomoneda_existe_en_sistema(prefijo) {
            self.criptomonedas.insert(prefijo, criptomoneda);
            return Ok(prefijo);
        }
        Err(ErroresApp::CriptoYaExiste)
    }

    pub fn crear_usuario(
        &mut self,
        nombre: &'p str,
        apellido: &'p str,
        email: &'p str,
        dni: u128,
    ) -> Result<u128, ErroresApp> {
        if self.usuario_existe(dni) {
            return Err(ErroresApp::UsuarioExistente);
        };

        let nuevo_usuario: Usuario = Usuario::new(nombre, apellido, email, dni);

        let criptomonedas_en_sistema = self.recuperar_criptomonedas_en_sistema();
        let nuevo_balance: Balance = Balance::new(dni, criptomonedas_en_sistema);

        self.usuarios.insert(nuevo_usuario.dni, nuevo_usuario);
        self.balances.insert(nuevo_usuario.dni, nuevo_balance);

        return Ok(nuevo_usuario.dni);
    }

    pub fn vender_cripto(
        &mut self,
        dni: u128,
        cripto: &'p str,
        monto: f64,
    ) -> Result<Operacion, ErroresApp> {
        self.usuario_dispone_fondos_cripto(dni, cripto, monto)?;

        // Recupero cotizacion de la criptomoneda
        let cotizacion = self.recuperar_datos_criptomoneda(cripto)?.cotizacion;

        // Recupero balance cripto del usuario y disminuyo el monto
        let balance_cripto = self.recuperar_balance_cripto_de_usuario(dni, cripto)?;
        *balance_cripto -= monto;

        // Modifico balance fiat aumentando el monto
        let cripto_a_fiat = monto * cotizacion;
        self.ingresar_fiat(dni, cripto_a_fiat)?;

        // Genero detalle y registro nueva operacion
        let detalle = DetalleOperacion::CRIPTOVENDER(cripto, monto, cotizacion);
        Ok(self.registrar_operacion(dni, detalle))
    }

    pub fn comprar_cripto(
        &mut self,
        dni: u128,
        cripto: &'p str,
        monto: f64,
    ) -> Result<Operacion, ErroresApp> {
        // Realizo chequeos de usuario
        self.realizar_chequeos_de_usuario(dni, true, true)?;
        self.usuario_dispone_fondos_fiat(dni, monto)?;

        // Recupero datos de la criptomoneda
        let cotizacion = self.recuperar_datos_criptomoneda(cripto)?.cotizacion;

        // Modifico balance fiat aumentando el monto
        let cripto_a_fiat = monto * cotizacion;
        let medio = "Balanceo interno";
        self.retirar_fiat(dni, cripto_a_fiat, medio)?;

        // Recupero balance de la cripto y aumento el monto
        let balance_cripto = self.recuperar_balance_cripto_de_usuario(dni, cripto)?;
        *balance_cripto += monto;

        // Genero detalle y registro nueva operacion
        let detalle = DetalleOperacion::CRIPTOCOMPRAR(cripto, monto, cotizacion);
        Ok(self.registrar_operacion(dni, detalle))
    }

    pub fn retirar_cripto(
        &mut self,
        dni: u128,
        cripto: &'p str,
        monto: f64,
        blockchain: &'p str,
    ) -> Result<Operacion, ErroresApp> {
        // Chequeo que la criptomoneda opere en la blockchain ingresada
        if !self.criptomoneda_opera_en_blockchain(cripto, blockchain)? {
            return Err(ErroresApp::RedCriptoInvalida);
        }
        self.realizar_chequeos_de_usuario(dni, true, true)?;
        self.usuario_dispone_fondos_cripto(dni, cripto, monto)?;

        // Recupero cotizacion de la cripto
        let cotizacion = self.recuperar_datos_criptomoneda(cripto)?.cotizacion;

        // Recupero balance mutable de usuario
        let balance_cripto = self.recuperar_balance_cripto_de_usuario(dni, cripto)?;
        // Modifico balance cripto disminuyendo el monto
        *balance_cripto -= monto;

        // Genero detalle y registro nueva operacion
        let detalle = DetalleOperacion::CRIPTORETIRAR(
            cripto,
            monto,
            cotizacion,
            blockchain,
            self.simular_envio_hash(),
        );
        Ok(self.registrar_operacion(dni, detalle))
    }

    pub fn recibir_cripto(
        &mut self,
        dni: u128,
        cripto: &'p str,
        monto: f64,
        blockchain: &'p str,
    ) -> Result<Operacion, ErroresApp> {
        // Chequeo que la criptomoneda opere en la blockchain ingresada
        if !self.criptomoneda_opera_en_blockchain(cripto, blockchain)? {
            return Err(ErroresApp::RedCriptoInvalida);
        }

        // Realizo chequeos de usuario
        self.realizar_chequeos_de_usuario(dni, true, false)?;

        // Recupero cotizacion de la cripto
        let cotizacion = self.recuperar_datos_criptomoneda(cripto)?.cotizacion;

        // Recupero balance cripto de usuario y lo aumento
        let balance_cripto = self.recuperar_balance_cripto_de_usuario(dni, cripto)?;
        *balance_cripto += monto;

        // Genero detalle y registro nueva operacion
        let detalle = DetalleOperacion::CRIPTORECIBIR(cripto, monto, cotizacion, blockchain);
        Ok(self.registrar_operacion(dni, detalle))
    }

    pub fn retirar_fiat(
        &mut self,
        dni: u128,
        monto: f64,
        medio: &'p str,
    ) -> Result<Operacion, ErroresApp> {
        // Realizo chequeos de usuario
        self.realizar_chequeos_de_usuario(dni, true, true)?;
        self.usuario_dispone_fondos_fiat(dni, monto)?;

        // Recupero balance fiat y lo modifico
        let balance_usuario = self.recuperar_balance_de_usuario(dni)?;
        balance_usuario.balance_fiat -= monto;

        // Genero detalle y registro nueva operacion
        let detalle = DetalleOperacion::FIATRETIRAR(monto, medio);
        Ok(self.registrar_operacion(dni, detalle))
    }

    pub fn ingresar_fiat(&mut self, dni: u128, monto: f64) -> Result<Operacion, ErroresApp> {
        // Realizo chequeos de usuario
        self.realizar_chequeos_de_usuario(dni, true, false)?;

        // Recupero balance fiat y lo modifico
        let balance_usuario = self.recuperar_balance_de_usuario(dni)?;
        balance_usuario.balance_fiat += monto;

        // Genero detalle y registro nueva operacion
        let detalle = DetalleOperacion::FIATINGRESAR(monto);
        Ok(self.registrar_operacion(dni, detalle))
    }

    fn registrar_operacion(&mut self, dni: u128, detalle: DetalleOperacion<'p>) -> Operacion<'p> {
        // Genero nueva operacion y su detalle
        let nueva_operacion = Operacion::new(dni, detalle);

        // Genero nueva operacion y su detalle
        self.operaciones.push(nueva_operacion.clone());
        nueva_operacion
    }

    fn usuario_existe(&self, dni: u128) -> bool {
        self.usuarios.contains_key(&dni)
    }

    fn usuario_validado(&self, dni: u128) -> bool {
        let usuario = self.usuarios.get(&dni);
        match usuario {
            Some(usuario) => usuario.validado,
            _ => false,
        }
    }

    fn usuario_dispone_fondos_fiat(&mut self, dni: u128, monto: f64) -> Result<bool, ErroresApp> {
        let balance = self.recuperar_balance_de_usuario(dni)?;
        let balance_fiat: f64 = balance.balance_fiat;
        if balance_fiat < monto {
            return Err(ErroresApp::MontoInsuficiente);
        }
        Ok(true)
    }

    fn usuario_dispone_fondos_cripto(
        &mut self,
        dni: u128,
        criptomoneda: &'p str,
        monto: f64,
    ) -> Result<bool, ErroresApp> {
        let balance = self.recuperar_balance_de_usuario(dni)?;
        let balance_cripto: &f64 = balance.balance_criptos.get(criptomoneda).unwrap();
        if *balance_cripto < monto {
            return Err(ErroresApp::MontoInsuficiente);
        }
        Ok(true)
    }

    fn recuperar_balance_cripto_de_usuario(
        &mut self,
        dni: u128,
        criptomoneda: &str,
    ) -> Result<&mut f64, ErroresApp> {
        let balance = self.recuperar_balance_de_usuario(dni)?;
        let balance_cripto = balance.balance_criptos.get_mut(criptomoneda);
        match balance_cripto {
            Some(balance_cripto) => Ok(balance_cripto),
            None => Err(ErroresApp::NoExisteBalance),
        }
    }

    fn recuperar_balance_de_usuario(&mut self, dni: u128) -> Result<&mut Balance<'p>, ErroresApp> {
        let balance = self.balances.get_mut(&dni);
        match balance {
            Some(balance) => Ok(balance),
            None => Err(ErroresApp::NoExisteBalance),
        }
    }

    fn criptomoneda_existe_en_sistema(&self, prefijo: &'p str) -> bool {
        self.criptomonedas.contains_key(prefijo)
    }

    fn recuperar_criptomonedas_en_sistema(&self) -> Vec<&'p str> {
        self.criptomonedas.values().map(|c| c.prefijo).collect()
    }

    fn recuperar_datos_criptomoneda(
        &self,
        prefijo: &'p str,
    ) -> Result<&Criptomoneda<'p>, ErroresApp> {
        let cripto = self.criptomonedas.get(prefijo);
        match cripto {
            Some(cripto) => Ok(cripto),
            None => Err(ErroresApp::CriptoNoExiste),
        }
    }

    fn criptomoneda_opera_en_blockchain(
        &self,
        criptomoneda: &'p str,
        blockchain: &'p str,
    ) -> Result<bool, ErroresApp> {
        let cripto = self.criptomonedas.get(criptomoneda);
        match cripto {
            Some(cripto) => Ok(cripto.blockchain_disponibles.contains_key(blockchain)),
            None => Err(ErroresApp::CriptoNoExiste),
        }
    }

    fn simular_envio_hash(&self) -> String {
        fn generar_random_hash(longitud: usize) -> String {
            let random_str: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(longitud)
                .map(char::from)
                .collect();
            random_str
        }

        generar_random_hash(20)
    }

    fn recuperar_usuario(&mut self, dni: u128) -> Result<&mut Usuario<'p>, ErroresApp> {
        match self.usuarios.get_mut(&dni) {
            Some(usuario) => Ok(usuario),
            None => Err(ErroresApp::UsuarioInexistente),
        }
    }

    pub fn validar_usuario(&mut self, dni: u128) -> Result<u128, ErroresApp> {
        let usuario = self.recuperar_usuario(dni)?;
        usuario.validado = true;
        Ok(usuario.dni)
    }

    fn realizar_chequeos_de_usuario(
        &self,
        dni: u128,
        existencia: bool,
        validez: bool,
    ) -> Result<(), ErroresApp> {
        // Usuario existe?
        if existencia && !self.usuario_existe(dni) {
            return Err(ErroresApp::UsuarioInexistente);
        };
        //Usuario es valido?
        if validez && !self.usuario_validado(dni) {
            return Err(ErroresApp::UsuarioNoValidado);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn instanciar_plataforma<'p>() -> Plataforma<'p> {
        let mut plataforma = Plataforma::new();
        let blockchain = Blockchain {
            nombre: "Ethereum",
            prefijo: "ETH",
        };
        let mut blockchains = HashMap::new();
        blockchains.insert("ETH", blockchain);
        let cripto = Criptomoneda {
            nombre: "Bitcoin",
            prefijo: "BTC",
            cotizacion: 50000.0,
            blockchain_disponibles: blockchains.clone(),
        };
        plataforma.agregar_criptomoneda(cripto).unwrap();
        plataforma
            .crear_usuario("Simon", "Bierozko", "asdf@gmail@.com", 123)
            .unwrap();
        plataforma.validar_usuario(123).unwrap();
        plataforma.ingresar_fiat(123, 100000.0).unwrap();
        plataforma
    }

    #[test]
    fn test_usuario_existe() {
        let plataforma = instanciar_plataforma();
        assert!(plataforma.usuario_existe(123));
        assert!(!plataforma.usuario_existe(999));
    }

    #[test]
    fn test_usuario_validado() {
        let plataforma = instanciar_plataforma();
        assert!(plataforma.usuario_validado(123));
    }

    #[test]
    fn test_usuario_dispone_fondos_fiat() {
        let mut plataforma = instanciar_plataforma();
        assert!(plataforma.usuario_dispone_fondos_fiat(123, 10.0).is_ok());
        assert!(
            plataforma
                .usuario_dispone_fondos_fiat(123, 200000.0)
                .is_err()
        );
    }

    #[test]
    fn test_usuario_dispone_fondos_cripto() {
        let mut plataforma = instanciar_plataforma();
        plataforma.comprar_cripto(123, "BTC", 1.0).unwrap();
        assert!(
            plataforma
                .usuario_dispone_fondos_cripto(123, "BTC", 1.0)
                .is_ok()
        );
        assert!(
            plataforma
                .usuario_dispone_fondos_cripto(123, "BTC", 2.0)
                .is_err()
        );
    }

    #[test]
    fn test_recuperar_balance_cripto_de_usuario() {
        let mut plataforma = instanciar_plataforma();
        assert!(
            plataforma
                .recuperar_balance_cripto_de_usuario(123, "BTC")
                .is_ok()
        );
        assert!(
            plataforma
                .recuperar_balance_cripto_de_usuario(123, "ETH")
                .is_err()
        );
    }

    #[test]
    fn test_recuperar_balance_de_usuario() {
        let mut plataforma = instanciar_plataforma();
        assert!(plataforma.recuperar_balance_de_usuario(123).is_ok());
        assert!(plataforma.recuperar_balance_de_usuario(999).is_err());
    }

    #[test]
    fn test_criptomoneda_existe_en_sistema() {
        let plataforma = instanciar_plataforma();
        assert!(plataforma.criptomoneda_existe_en_sistema("BTC"));
        assert!(!plataforma.criptomoneda_existe_en_sistema("ETH"));
    }

    #[test]
    fn test_recuperar_criptomonedas_en_sistema() {
        let plataforma = instanciar_plataforma();
        let criptos = plataforma.recuperar_criptomonedas_en_sistema();
        assert_eq!(criptos, vec!["BTC"]);
    }

    #[test]
    fn test_recuperar_datos_criptomoneda() {
        let plataforma = instanciar_plataforma();
        assert!(plataforma.recuperar_datos_criptomoneda("BTC").is_ok());
        assert!(plataforma.recuperar_datos_criptomoneda("ETH").is_err());
    }

    #[test]
    fn test_criptomoneda_opera_en_blockchain() {
        let plataforma = instanciar_plataforma();
        assert!(
            plataforma
                .criptomoneda_opera_en_blockchain("BTC", "ETH")
                .unwrap()
        );
        assert!(
            !plataforma
                .criptomoneda_opera_en_blockchain("BTC", "SOL")
                .unwrap()
        );
    }

    #[test]
    fn test_simular_envio_hash() {
        let plataforma = instanciar_plataforma();
        let hash = plataforma.simular_envio_hash();
        assert_eq!(hash.len(), 20);
    }

    #[test]
    fn test_recuperar_usuario() {
        let mut plataforma = instanciar_plataforma();
        assert!(plataforma.recuperar_usuario(123).is_ok());
        assert!(plataforma.recuperar_usuario(999).is_err());
    }

    #[test]
    fn test_realizar_chequeos_de_usuario() {
        let plataforma = instanciar_plataforma();
        assert!(
            plataforma
                .realizar_chequeos_de_usuario(123, true, true)
                .is_ok()
        );
        assert!(
            plataforma
                .realizar_chequeos_de_usuario(999, true, false)
                .is_err()
        );
        assert!(
            plataforma
                .realizar_chequeos_de_usuario(123, true, true)
                .is_ok()
        );
    }

    #[test]
    fn test_vender_cripto() {
        let mut plataforma = instanciar_plataforma();
        plataforma.ingresar_fiat(123, 100000000.0).unwrap();
        plataforma.comprar_cripto(123, "BTC", 5.0).unwrap();
        let op = plataforma.vender_cripto(123, "BTC", 2.0).unwrap();
        assert_eq!(op.usuario, 123);
    }

    #[test]
    fn test_comprar_cripto() {
        let mut plataforma = instanciar_plataforma();
        plataforma.ingresar_fiat(123, 9000000.0).unwrap();
        let op = plataforma.comprar_cripto(123, "BTC", 3.0).unwrap();
        assert_eq!(op.usuario, 123);
    }

    #[test]
    fn test_retirar_cripto() {
        let mut plataforma = instanciar_plataforma();
        plataforma.ingresar_fiat(123, 10000000.0).unwrap();
        plataforma.comprar_cripto(123, "BTC", 5.0).unwrap();
        let op = plataforma.retirar_cripto(123, "BTC", 2.0, "ETH").unwrap();
        assert_eq!(op.usuario, 123);
    }

    #[test]
    fn test_recibir_cripto() {
        let mut plataforma = instanciar_plataforma();
        let op = plataforma.recibir_cripto(123, "BTC", 2.0, "ETH").unwrap();
        assert_eq!(op.usuario, 123);
    }

    #[test]
    fn test_retirar_fiat() {
        let mut plataforma = instanciar_plataforma();
        plataforma.ingresar_fiat(123, 1000.0).unwrap();
        let op = plataforma
            .retirar_fiat(123, 300.0, "Transferencia")
            .unwrap();
        assert_eq!(op.usuario, 123);
    }

    #[test]
    fn test_ingresar_fiat() {
        let mut plataforma = instanciar_plataforma();
        let op = plataforma.ingresar_fiat(123, 500.0).unwrap();
        assert_eq!(op.usuario, 123);
    }

    #[test]
    fn test_agregar_criptomoneda() {
        let mut plataforma = Plataforma::new();
        let blockchain = Blockchain {
            nombre: "Solana",
            prefijo: "SOL",
        };
        let mut blockchains = HashMap::new();
        blockchains.insert("SOL", blockchain);
        let cripto = Criptomoneda {
            nombre: "Solana",
            prefijo: "SOL",
            cotizacion: 1000.0,
            blockchain_disponibles: blockchains,
        };
        let res = plataforma.agregar_criptomoneda(cripto);
        assert!(res.is_ok());
    }

    #[test]
    fn test_crear_usuario() {
        let mut plataforma = Plataforma::new();
        let blockchain = Blockchain {
            nombre: "Ethereum",
            prefijo: "ETH",
        };
        let mut blockchains = HashMap::new();
        blockchains.insert("ETH", blockchain);
        let cripto = Criptomoneda {
            nombre: "Ethereum",
            prefijo: "ETH",
            cotizacion: 1000.0,
            blockchain_disponibles: blockchains,
        };
        plataforma.agregar_criptomoneda(cripto).unwrap();
        let res = plataforma.crear_usuario("Cosme", "Fulanito", "Cosme@gmail.com", 123);
        assert!(res.is_ok());
    }

    #[test]
    fn test_validar_usuario() {
        let mut plataforma = instanciar_plataforma();
        let res = plataforma.validar_usuario(123);
        assert_eq!(res.unwrap(), 123);
    }
}
