use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, Write};

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Color {
    ROJO,
    VERDE,
    AZUL,
    AMARILLO,
    BLANCO,
    NEGRO,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

    pub fn calcular_precio(&mut self) -> f64 {
        let mut precio_final: f64 = self.precio_bruto;

        precio_final += self.percepciones_por_color();
        precio_final += self.percepciones_por_marca();
        precio_final += self.percepciones_por_ano();

        precio_final
    }

    fn percepciones_por_color(&self) -> f64 {
        let primarios: [Color; 3] = [Color::ROJO, Color::AZUL, Color::AMARILLO];
        if primarios.contains(&self.color) {
            return self.precio_bruto * 0.25;
        }
        -1.0 * self.precio_bruto * 0.10
    }

    fn percepciones_por_marca(&self) -> f64 {
        if self.marca == "BMW" {
            return self.precio_bruto * 0.15;
        }
        0.0
    }

    fn percepciones_por_ano(&self) -> f64 {
        if self.ano < 2000 {
            return -1.0 * self.precio_bruto * 0.05;
        }
        0.0
    }

    pub fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Concesionaria {
    pub nombre: String,
    pub direccion: String,
    pub capacidad_max: usize,
    pub autos_en_stock: Vec<Auto>,
    pub ruta_archivo: String,
}

#[allow(dead_code)]
impl Concesionaria {
    pub fn new(
        nombre: String,
        direccion: String,
        capacidad_max: usize,
        autos_en_stock: Vec<Auto>,
        ruta_archivo: String,
    ) -> Self {
        let mut c = Concesionaria {
            nombre,
            direccion,
            capacidad_max,
            autos_en_stock,
            ruta_archivo,
        };
        c.autos_en_stock = c.leer_de_archivo();
        c
    }

    pub fn agregar_auto(&mut self, nuevo_auto: Auto) -> bool {
        if self.autos_en_stock.len() >= self.capacidad_max {
            return false;
        }
        self.autos_en_stock.push(nuevo_auto);
        self.guardar_en_archivo();
        true
    }

    pub fn eliminar_auto(&mut self, marca: String, modelo: String, ano: u16, color: Color) -> bool {
        if self.autos_en_stock.is_empty() {
            return false;
        }
        if let Some(indice) = self.obtener_posicion_auto(&marca, &modelo, ano, &color) {
            self.autos_en_stock.remove(indice);
            self.guardar_en_archivo();
            return true;
        }
        false
    }

    pub fn buscar_auto(&self, marca: String, modelo: String, ano: u16, color: Color) -> String {
        if self.autos_en_stock.is_empty() {
            return "No hay autos en stock".to_string();
        }
        if let Some(indice) = self.obtener_posicion_auto(&marca, &modelo, ano, &color) {
            return self.autos_en_stock[indice].to_string();
        }
        "No se encontro auto con esas caracteristicas".to_string()
    }

    fn obtener_posicion_auto(
        &self,
        marca: &str,
        modelo: &str,
        ano: u16,
        color: &Color,
    ) -> Option<usize> {
        self.autos_en_stock.iter().position(|a| {
            a.marca == marca && a.modelo == modelo && a.ano == ano && &a.color == color
        })
    }

    fn guardar_en_archivo(&self) {
        if let Ok(json) = serde_json::to_string_pretty(&self.autos_en_stock) {
            let _ = File::create(&self.ruta_archivo).and_then(|mut f| f.write_all(json.as_bytes()));
        }
    }

    pub fn leer_de_archivo(&self) -> Vec<Auto> {
        if let Ok(file) = File::open(&self.ruta_archivo) {
            let reader = BufReader::new(file);
            serde_json::from_reader(reader).unwrap_or_default()
        } else {
            vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;

    fn crear_auto(marca: &str, modelo: &str, ano: u16, color: Color, precio: f64) -> Auto {
        Auto::new(marca.into(), modelo.into(), ano, color, precio)
    }

    #[test]
    fn test_precio_color_primario() {
        let mut auto = Auto::new("Fiat".into(), "Uno".into(), 2020, Color::ROJO, 10000.0);
        let precio = auto.calcular_precio();
        assert_eq!(precio, 10000.0 + 2500.0); // +25% por color primario
    }

    #[test]
    fn test_precio_color_no_primario() {
        let mut auto = Auto::new("Fiat".into(), "Uno".into(), 2020, Color::VERDE, 10000.0);
        let precio = auto.calcular_precio();
        assert_eq!(precio, 10000.0 - 1000.0); // -10% por color no primario
    }

    #[test]
    fn test_precio_por_marca_bmw() {
        let mut auto = Auto::new("BMW".into(), "X5".into(), 2020, Color::ROJO, 10000.0);
        let precio = auto.calcular_precio();
        // +25% color primario +15% marca
        assert_eq!(precio, 10000.0 + 2500.0 + 1500.0);
    }

    #[test]
    fn test_precio_por_ano_antiguo() {
        let mut auto = Auto::new("Ford".into(), "F100".into(), 1990, Color::NEGRO, 10000.0);
        let precio = auto.calcular_precio();
        // -10% color no primario -5% por año
        assert_eq!(precio, 10000.0 - 1000.0 - 500.0);
    }

    #[test]
    fn test_archivo_inexistente() {
        let ruta = "test_archivo_inexistente.json";
        let _ = fs::remove_file(ruta);
        let c = Concesionaria::new("X".into(), "Y".into(), 5, vec![], ruta.into());
        assert!(c.autos_en_stock.is_empty());
    }

    #[test]
    fn test_lee_archivo_si_existe() {
        let ruta = "test_lee_archivo_si_existe.json";
        let autos = vec![crear_auto("Ford", "Focus", 2020, Color::NEGRO, 20000.0)];
        let json = serde_json::to_string(&autos).unwrap();
        let mut file = File::create(ruta).unwrap();
        file.write_all(json.as_bytes()).unwrap();
        let c = Concesionaria::new("A".into(), "B".into(), 5, vec![], ruta.into());
        assert_eq!(c.autos_en_stock.len(), 1);
        let _ = fs::remove_file(ruta);
    }

    #[test]
    fn test_agregar_auto() {
        let ruta = "test_agregar_auto.json";
        let mut c = Concesionaria::new("Test".into(), "asdf".into(), 2, vec![], ruta.into());
        let auto = crear_auto("Fiat", "Uno", 2010, Color::ROJO, 10000.0);
        let ok = c.agregar_auto(auto.clone());
        assert!(ok);
        assert_eq!(c.autos_en_stock.len(), 1);
        let _ = fs::remove_file(ruta);
    }

    #[test]
    fn test_agregar_auto_supera_capacidad() {
        let ruta = "test_agregar_auto_supera_capacidad.json";
        let mut c = Concesionaria::new("Test".into(), "asdf".into(), 1, vec![], ruta.into());
        c.agregar_auto(crear_auto("Fiat", "Uno", 2010, Color::ROJO, 10000.0));
        let fail = c.agregar_auto(crear_auto("Peugeot", "208", 2012, Color::VERDE, 13000.0));
        assert!(!fail);
        let _ = fs::remove_file(ruta);
    }

    #[test]
    fn test_agregar_auto_y_guardar() {
        let ruta = "test_agregar_auto_y_guardar.json";
        let mut c = Concesionaria::new("Test".into(), "asdf".into(), 5, vec![], ruta.into());
        let auto = crear_auto("VW", "Gol", 2018, Color::AZUL, 15000.0);
        c.agregar_auto(auto.clone());
        let contenido = fs::read_to_string(ruta).unwrap();
        assert!(contenido.contains("Gol"));
        let _ = fs::remove_file(ruta);
    }

    #[test]
    fn test_eliminar_auto_existente() {
        let ruta = "test_eliminar_auto_existente.json";
        let mut c = Concesionaria::new("Test".into(), "asdf".into(), 5, vec![], ruta.into());
        let auto = crear_auto("Ford", "fiesta", 2020, Color::BLANCO, 20000.0);
        c.agregar_auto(auto.clone());
        let eliminado = c.eliminar_auto("Ford".into(), "fiesta".into(), 2020, Color::BLANCO);
        assert!(eliminado);
        let contenido = fs::read_to_string(ruta).unwrap();
        assert!(!contenido.contains("Corolla"));
        let _ = fs::remove_file(ruta);
    }

    #[test]
    fn test_eliminar_auto_inexistente() {
        let ruta = "test_eliminar_auto_inexistente.json";
        let mut c = Concesionaria::new("Test".into(), "asdf".into(), 5, vec![], ruta.into());
        let eliminado = c.eliminar_auto("X".into(), "Y".into(), 2000, Color::ROJO);
        assert!(!eliminado);
        let _ = fs::remove_file(ruta);
    }

    #[test]
    fn test_buscar_auto_existente() {
        let ruta = "test_buscar_auto_existente.json";
        let mut c = Concesionaria::new(
            "Buscadora".into(),
            "Calle Falsa".into(),
            5,
            vec![],
            ruta.into(),
        );
        let auto = crear_auto("Ford", "Ka", 2018, Color::VERDE, 12000.0);
        c.agregar_auto(auto.clone());
        let res = c.buscar_auto("Ford".into(), "Ka".into(), 2018, Color::VERDE);
        assert!(res.contains("Ford"));
        let _ = fs::remove_file(ruta);
    }

    #[test]
    fn test_buscar_auto_inexistente() {
        let ruta = "test_buscar_auto_inexistente.json";
        let c = Concesionaria::new("Buscadora".into(), "Calle".into(), 5, vec![], ruta.into());
        let res = c.buscar_auto("X".into(), "Y".into(), 2000, Color::AZUL);
        assert_eq!(res, "No hay autos en stock");
    }

    #[test]
    fn test_leer_autos_de_archivo_valido() {
        let ruta = "test_leer_autos_de_archivo_valido.json";
        let autos = vec![crear_auto("Renault", "Clio", 2015, Color::NEGRO, 11000.0)];
        let mut file = File::create(ruta).unwrap();
        file.write_all(serde_json::to_string(&autos).unwrap().as_bytes())
            .unwrap();
        let c = Concesionaria::new("probar".into(), "asdf".into(), 5, vec![], ruta.into());
        assert_eq!(c.autos_en_stock.len(), 1);
        let _ = fs::remove_file(ruta);
    }

    #[test]
    fn test_leer_autos_de_archivo_inexistente() {
        let ruta = "test_leer_autos_de_archivo_inexistente.json";
        let _ = fs::remove_file(ruta);
        let c = Concesionaria::new("probar".into(), "asdf".into(), 5, vec![], ruta.into());
        assert!(c.autos_en_stock.is_empty());
    }

    #[test]
    fn test_guardar_en_archivo() {
        let ruta = "test_guardar_en_archivo.json";
        let autos = vec![crear_auto("Reno", "clio", 2021, Color::ROJO, 22000.0)];
        let c = Concesionaria {
            nombre: "Guardar".into(),
            direccion: "asdf".into(),
            capacidad_max: 10,
            autos_en_stock: autos.clone(),
            ruta_archivo: ruta.into(),
        };
        c.guardar_en_archivo();
        let contenido = fs::read_to_string(ruta).unwrap();
        assert!(contenido.contains("clio"));
        let _ = fs::remove_file(ruta);
    }
}
