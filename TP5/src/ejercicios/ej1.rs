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
}

#[allow(dead_code)]
impl Concesionaria {
    pub fn new(
        nombre: String,
        direccion: String,
        capacidad_max: usize,
        autos_en_stock: Vec<Auto>,
    ) -> Self {
        Concesionaria {
            nombre,
            direccion,
            capacidad_max,
            autos_en_stock,
        }
    }

    pub fn agregar_auto(&mut self, nuevo_auto: Auto) -> bool {
        if self.autos_en_stock.len() >= self.capacidad_max {
            return false;
        }
        self.autos_en_stock.push(nuevo_auto);
        true
    }

    pub fn eliminar_auto(&mut self, marca: String, modelo: String, ano: u16, color: Color) -> bool {
        if self.autos_en_stock.is_empty() {
            return false;
        }
        if let Some(indice) = self.obtener_posicion_auto(&marca, &modelo, ano, &color) {
            self.autos_en_stock.remove(indice);
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

    pub fn agregar_auto_y_guardar(&mut self, nuevo_auto: Auto) -> Result<(), String> {
        if self.autos_en_stock.len() >= self.capacidad_max {
            return Err(format!(
                "No se puede agregar auto: capacidad maxima de {} alcanzada",
                self.capacidad_max
            ));
        }
        self.agregar_auto(nuevo_auto.clone());
        let mut autos_archivo = self.leer_autos_de_archivo();
        autos_archivo.push(nuevo_auto);
        self.escribir_autos_en_archivo(&autos_archivo);
        Ok(())
    }

    pub fn eliminar_auto_y_guardar(
        &mut self,
        marca: String,
        modelo: String,
        ano: u16,
        color: Color,
    ) -> bool {
        let ok = self.eliminar_auto(marca.clone(), modelo.clone(), ano, color.clone());
        if ok {
            let autos_filtrados: Vec<Auto> = self
                .leer_autos_de_archivo()
                .into_iter()
                .filter(|a| {
                    !(a.marca == marca && a.modelo == modelo && a.ano == ano && a.color == color)
                })
                .collect();
            self.escribir_autos_en_archivo(&autos_filtrados);
        }
        ok
    }

    pub fn leer_autos_de_archivo(&self) -> Vec<Auto> {
        const AUTOS: &str = "autos.json";
        if let Ok(file) = File::open(AUTOS) {
            let reader = BufReader::new(file);
            serde_json::from_reader(reader).unwrap_or_default()
        } else {
            vec![]
        }
    }

    pub fn escribir_autos_en_archivo(&self, autos: &[Auto]) {
        const AUTOS: &str = "autos.json";
        if let Ok(mut file) = File::create(AUTOS) {
            let _ = file.write_all(serde_json::to_string_pretty(autos).unwrap().as_bytes());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn auto_bmw() -> Auto {
        Auto::new(
            "BMW".to_string(),
            "Z1".to_string(),
            2001,
            Color::ROJO,
            1000.0,
        )
    }

    fn auto_renault() -> Auto {
        Auto::new(
            "Renault".to_string(),
            "Clio".to_string(),
            2016,
            Color::AMARILLO,
            300.0,
        )
    }

    fn auto_ford() -> Auto {
        Auto::new(
            "Ford".to_string(),
            "Fiesta".to_string(),
            1995,
            Color::NEGRO,
            500.0,
        )
    }

    fn clean_file() {
        let _ = fs::remove_file("autos.json");
    }

    fn concesionaria_test() -> Concesionaria {
        clean_file();
        Concesionaria::new("prueba".to_string(), "la plata".to_string(), 3, vec![])
    }

    #[test]
    fn test_agregar_auto_y_guardar() {
        clean_file();
        let mut conce = concesionaria_test();
        let auto = auto_bmw();

        let res = conce.agregar_auto_y_guardar(auto.clone());
        assert!(res.is_ok());

        let contenido = conce.leer_autos_de_archivo();
        assert_eq!(contenido.len(), 1);
        assert_eq!(contenido[0], auto);
        clean_file();
    }

    #[test]
    fn test_agregar_auto_supera_limite() {
        clean_file();
        let mut conce = Concesionaria::new(
            "lleno".to_string(),
            "la plata".to_string(),
            1,
            vec![auto_bmw()],
        );
        let auto = auto_renault();
        let res = conce.agregar_auto_y_guardar(auto);

        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err(),
            "No se puede agregar auto: capacidad maxima de 1 alcanzada"
        );
        clean_file();
    }

    #[test]
    fn test_eliminar_auto_y_guardar() {
        clean_file();
        let mut conce = concesionaria_test();
        let auto1 = auto_bmw();
        let auto2 = auto_renault();
        let auto3 = auto_ford();

        conce.agregar_auto_y_guardar(auto1.clone()).unwrap();
        conce.agregar_auto_y_guardar(auto2.clone()).unwrap();
        conce.agregar_auto_y_guardar(auto3.clone()).unwrap();

        let eliminado = conce.eliminar_auto_y_guardar(
            auto1.marca.clone(),
            auto1.modelo.clone(),
            auto1.ano,
            auto1.color.clone(),
        );

        assert!(eliminado);
        let cargados = conce.leer_autos_de_archivo();
        assert_eq!(cargados.len(), 2);
        assert_eq!(cargados[0], auto2);
        assert_eq!(cargados[1], auto3);
        clean_file();
    }

    #[test]
    fn test_escribir_y_leer_autos_directamente() {
        clean_file();
        let conce = Concesionaria::new("solo".to_string(), "la plata".to_string(), 2, vec![]);
        let autos = vec![auto_ford(), auto_renault()];
        conce.escribir_autos_en_archivo(&autos);

        let leidos = conce.leer_autos_de_archivo();

        assert_eq!(leidos.len(), 2);
        assert_eq!(leidos[0].marca, "Ford");
        assert_eq!(leidos[1].marca, "Renault");
        clean_file();
    }
}
