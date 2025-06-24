use std::fmt::{self};

#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(dead_code)]
pub enum ErroresApp {
    SinEspacio,
    Vacio,
    AutoInexistente,
}

impl fmt::Display for ErroresApp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErroresApp::SinEspacio => write!(
                f,
                "La capacidad de la consecionaria se encuentra al maximo."
            ),
            ErroresApp::Vacio => write!(f, "No hay autos en la consecionaria"),
            ErroresApp::AutoInexistente => {
                write!(f, "El auto ingresado no existe en la concesionaria")
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum Color {
    ROJO,
    VERDE,
    AZUL,
    AMARILLO,
    BLANCO,
    NEGRO,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Auto {
    pub marca: String,
    pub modelo: String,
    pub ano: u16,
    pub color: Color,
    pub precio_bruto: f64,
}

#[allow(dead_code)]

impl Auto {
    pub fn new(marca: String, modelo: String, ano: u16, color: Color, precio_bruto: f64) -> Auto {
        Auto {
            marca,
            modelo,
            ano,
            color,
            precio_bruto,
        }
    }

    fn calcular_precio(&mut self) -> f64 {
        let mut precio_final: f64 = self.precio_bruto;

        // PERCEPCIONES POR TIPO DE COLOR
        precio_final += self.percepciones_por_color();

        // PERCEPCIONES POR MARCA
        precio_final += self.percepciones_por_marca();

        // PERCEPCIONES POR AÑO
        precio_final += self.percepciones_por_ano();

        return precio_final;
    }
    fn percepciones_por_color(&self) -> f64 {
        let primarios: [Color; 3] = [Color::ROJO, Color::AZUL, Color::AMARILLO];

        // PERCEPCIONES POR TIPO DE COLOR
        if primarios.contains(&self.color) {
            return self.precio_bruto * 25.0 / 100.0;
        }
        return -1.0 * (self.precio_bruto * 10.0 / 100.0);
    }

    fn percepciones_por_marca(&self) -> f64 {
        if self.marca == "BMW".to_string() {
            return self.precio_bruto * 15.0 / 100.0;
        }
        return 0.0;
    }

    fn percepciones_por_ano(&self) -> f64 {
        if self.ano < 2000 {
            return -1.0 * (self.precio_bruto * 5.0 / 100.0);
        }
        return 0.0;
    }

    pub fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

#[allow(dead_code)]
pub struct Concesionaria {
    pub nombre: String,
    pub direccion: String,
    pub capacidad_max: usize,
    pub autos_en_stock: Vec<Auto>,
}

#[allow(dead_code)]
impl Concesionaria {
    pub fn new(
        nombre: String,
        direccion: String,
        capacidad_max: usize,
        autos_en_stock: Vec<Auto>,
    ) -> Concesionaria {
        Concesionaria {
            nombre,
            direccion,
            capacidad_max,
            autos_en_stock,
        }
    }

    pub fn agregar_auto(&mut self, nuevo_auto: Auto) -> Result<String, ErroresApp> {
        if self.autos_en_stock.len() >= self.capacidad_max {
            return Err(ErroresApp::SinEspacio);
        }
        self.autos_en_stock.push(nuevo_auto.clone());
        Ok(nuevo_auto.marca)
    }

    pub fn eliminar_auto(
        &mut self,
        marca: String,
        modelo: String,
        ano: u16,
        color: Color,
    ) -> Result<String, ErroresApp> {
        if self.autos_en_stock.len() == 0 {
            return Err(ErroresApp::Vacio);
        }
        if let Some(indice) = self.obtener_posicion_auto(marca.clone(), modelo, ano, color) {
            self.autos_en_stock.remove(indice);
            return Ok(marca);
        }
        Err(ErroresApp::AutoInexistente)
    }
    pub fn buscar_auto(&self, marca: String, modelo: String, ano: u16, color: Color) -> String {
        if self.autos_en_stock.len() == 0 {
            return "No hay autos en stock".to_string();
        }

        if let Some(indice) = self.obtener_posicion_auto(marca, modelo, ano, color) {
            return Some(self.autos_en_stock[indice].to_string()).unwrap();
        }

        return "No se encontro auto con esas caracteristicas".to_string();
    }

    fn obtener_posicion_auto(
        &self,
        marca: String,
        modelo: String,
        ano: u16,
        color: Color,
    ) -> Option<usize> {
        if let Some(indice) = self.autos_en_stock.iter().position(|auto| {
            auto.marca == marca && auto.modelo == modelo && auto.ano == ano && auto.color == color
        }) {
            return Some(indice);
        }
        return None;
    }
}

#[test]
fn test_auto_percepciones_por_color() {
    let mut auto = Auto::new(
        "BMW".to_string(),
        "Z1".to_string(),
        2001,
        Color::ROJO,
        1000.0,
    );
    assert_eq!(250.0, auto.percepciones_por_color());

    auto.color = Color::BLANCO;
    assert_eq!(-100.0, auto.percepciones_por_color());

    auto.color = Color::AMARILLO;
    auto.precio_bruto = 2000.0;
    assert_eq!(500.0, auto.percepciones_por_color());
}

#[test]
fn test_auto_percepciones_por_marca() {
    let mut auto = Auto::new(
        "BMW".to_string(),
        "Z1".to_string(),
        2001,
        Color::ROJO,
        1000.0,
    );

    assert_eq!(150.0, auto.percepciones_por_marca());

    auto.marca = "Renault".to_string();
    assert_eq!(0.0, auto.percepciones_por_marca());
}

#[test]
fn test_auto_percepciones_por_ano() {
    let mut auto = Auto::new(
        "BMW".to_string(),
        "Z1".to_string(),
        2001,
        Color::ROJO,
        1000.0,
    );

    assert_eq!(0.0, auto.percepciones_por_ano());

    auto.ano = 1999;
    assert_eq!(-50.0, auto.percepciones_por_ano());
}

#[test]
fn test_auto_calcular_precio() {
    let mut auto = Auto::new(
        "BMW".to_string(),
        "Z1".to_string(),
        2001,
        Color::ROJO,
        1000.0,
    );

    assert_eq!(1400.0, auto.calcular_precio());

    auto.marca = "Ford".to_string();
    auto.modelo = "Fiesta".to_string();
    auto.ano = 1995;
    auto.color = Color::NEGRO;
    auto.precio_bruto = 500.0;

    assert_eq!(500.0 - 50.0 - 25.0, auto.calcular_precio());
}

#[test]
fn test_concesionaria_agregar_auto() {
    let auto = Auto::new(
        "BMW".to_string(),
        "Z1".to_string(),
        2001,
        Color::ROJO,
        1000.0,
    );

    let auto2: Auto = Auto::new(
        "Renault".to_string(),
        "Clio".to_string(),
        2016,
        Color::AMARILLO,
        300.0,
    );

    let mut concesionaria = Concesionaria::new(
        "Simone".to_string(),
        "Abasto al fondo".to_string(),
        2,
        vec![auto],
    );
    concesionaria.agregar_auto(auto2.clone());

    assert_eq!(
        auto2.to_string(),
        concesionaria.autos_en_stock[1].to_string()
    );

    let auto3: Auto = Auto::new(
        "Ford".to_string(),
        "Fiesta".to_string(),
        2016,
        Color::AMARILLO,
        300.0,
    );

    let resultado: bool = concesionaria.agregar_auto(auto3.clone());
    println!("{}", concesionaria.autos_en_stock.len());

    assert_eq!(false, resultado);
}

#[test]
fn test_concesionaria_obtener_posicion_auto() {
    let auto1 = Auto::new(
        "BMW".to_string(),
        "Z1".to_string(),
        2001,
        Color::ROJO,
        1000.0,
    );

    let auto2: Auto = Auto::new(
        "Renault".to_string(),
        "Clio".to_string(),
        2016,
        Color::AMARILLO,
        300.0,
    );

    let auto3: Auto = Auto::new(
        "Ford".to_string(),
        "Fiesta".to_string(),
        2016,
        Color::AMARILLO,
        300.0,
    );

    let mut concesionaria = Concesionaria::new(
        "Simone".to_string(),
        "Abasto al fondo".to_string(),
        3,
        vec![],
    );

    concesionaria.agregar_auto(auto1.clone());
    concesionaria.agregar_auto(auto2.clone());
    concesionaria.agregar_auto(auto3.clone());

    let posicion: Option<usize> = concesionaria.obtener_posicion_auto(
        "Renault".to_string(),
        "Clio".to_string(),
        2016,
        Color::AMARILLO,
    );

    assert_eq!(1, posicion.unwrap());

    let posicion: Option<usize> = concesionaria.obtener_posicion_auto(
        "Renault".to_string(),
        "Clio".to_string(),
        2010,
        Color::AMARILLO,
    );

    assert_eq!(None, posicion)
}
#[test]
fn test_concesionaria_buscar_auto() {
    let auto1 = Auto::new(
        "BMW".to_string(),
        "Z1".to_string(),
        2001,
        Color::ROJO,
        1000.0,
    );

    let auto2: Auto = Auto::new(
        "Renault".to_string(),
        "Clio".to_string(),
        2016,
        Color::AMARILLO,
        300.0,
    );

    let auto3: Auto = Auto::new(
        "Ford".to_string(),
        "Fiesta".to_string(),
        2016,
        Color::AMARILLO,
        300.0,
    );

    let mut concesionaria = Concesionaria::new(
        "Simone".to_string(),
        "Abasto al fondo".to_string(),
        3,
        vec![],
    );
    let busqueda0 =
        concesionaria.buscar_auto("Ford".to_string(), "Ka".to_string(), 2016, Color::AMARILLO);

    assert_eq!("No hay autos en stock".to_string(), busqueda0);
    concesionaria.agregar_auto(auto1.clone());
    concesionaria.agregar_auto(auto2.clone());
    concesionaria.agregar_auto(auto3.clone());

    let busqueda = concesionaria.buscar_auto(
        "Ford".to_string(),
        "Fiesta".to_string(),
        2016,
        Color::AMARILLO,
    );

    let busqueda2 =
        concesionaria.buscar_auto("Ford".to_string(), "Ka".to_string(), 2016, Color::AMARILLO);

    assert_eq!(auto3.to_string(), busqueda);
    assert_eq!(
        "No se encontro auto con esas caracteristicas".to_string(),
        busqueda2
    )
}

#[test]
fn test_concesionaria_eliminar_auto() {
    let auto1 = Auto::new(
        "BMW".to_string(),
        "Z1".to_string(),
        2001,
        Color::ROJO,
        1000.0,
    );

    let auto2: Auto = Auto::new(
        "Renault".to_string(),
        "Clio".to_string(),
        2016,
        Color::AMARILLO,
        300.0,
    );

    let auto3: Auto = Auto::new(
        "Ford".to_string(),
        "Fiesta".to_string(),
        2016,
        Color::AMARILLO,
        300.0,
    );

    let mut concesionaria = Concesionaria::new(
        "Simone".to_string(),
        "Abasto al fondo".to_string(),
        3,
        vec![],
    );

    concesionaria.agregar_auto(auto1.clone());
    concesionaria.agregar_auto(auto2.clone());
    concesionaria.agregar_auto(auto3.clone());

    let busqueda = concesionaria.buscar_auto(
        "Renault".to_string(),
        "Clio".to_string(),
        2016,
        Color::AMARILLO,
    );

    assert_eq!(auto2.to_string(), busqueda);

    let resultado = concesionaria.eliminar_auto(
        "Renault".to_string(),
        "Clio".to_string(),
        2016,
        Color::AMARILLO,
    );

    let busqueda = concesionaria.buscar_auto(
        "Renault".to_string(),
        "Clio".to_string(),
        2016,
        Color::AMARILLO,
    );

    assert_ne!(false, resultado);
    assert_eq!(
        "No se encontro auto con esas caracteristicas".to_string(),
        busqueda
    );
}
