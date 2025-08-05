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

#[derive(Debug, Clone, PartialEq)]
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

    pub fn calcular_precio(&self) -> f64 {
        let mut precio = self.precio_bruto;

        // recargo/descuento por color
        if self.color == Color::ROJO || self.color == Color::AZUL || self.color == Color::AMARILLO {
            precio += precio * 0.25;
        } else {
            precio -= precio * 0.10;
        }

        // recargo por marca BMW
        if self.marca == "BMW" {
            precio += precio * 0.15;
        }

        // descuento por año menor a 2000
        if self.ano < 2000 {
            precio -= precio * 0.05;
        }

        precio
    }
}

#[allow(dead_code)]
pub struct ConcesionarioAuto {
    pub nombre: String,
    pub direccion: String,
    pub capacidad_max: usize,
    pub autos_en_stock: Vec<Auto>,
}

#[allow(dead_code)]
impl ConcesionarioAuto {
    pub fn new(nombre: String, direccion: String, capacidad_max: usize) -> ConcesionarioAuto {
        ConcesionarioAuto {
            nombre,
            direccion,
            capacidad_max,
            autos_en_stock: Vec::new(),
        }
    }

    pub fn agregar_auto(&mut self, nuevo_auto: Auto) -> bool {
        if self.autos_en_stock.len() >= self.capacidad_max {
            return false;
        }
        self.autos_en_stock.push(nuevo_auto);
        true
    }

    pub fn buscar_auto(&self, auto_buscado: &Auto) -> Option<&Auto> {
        let mut i = 0;
        while i < self.autos_en_stock.len() {
            let a = &self.autos_en_stock[i];
            if a.marca == auto_buscado.marca
                && a.modelo == auto_buscado.modelo
                && a.ano == auto_buscado.ano
                && a.color == auto_buscado.color
            {
                return Some(a);
            }
            i += 1;
        }
        None
    }

    pub fn eliminar_auto(&mut self, auto_buscado: &Auto) -> bool {
        let mut i = 0;
        while i < self.autos_en_stock.len() {
            let a = &self.autos_en_stock[i];
            if a.marca == auto_buscado.marca
                && a.modelo == auto_buscado.modelo
                && a.ano == auto_buscado.ano
                && a.color == auto_buscado.color
            {
                self.autos_en_stock.remove(i);
                return true;
            }
            i += 1;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn crear_auto(marca: &str, modelo: &str, ano: u16, color: Color, precio: f64) -> Auto {
        Auto::new(marca.to_string(), modelo.to_string(), ano, color, precio)
    }

    #[test]
    fn test_calcular_precio_color_primario() {
        let auto = crear_auto("Ford", "Fiesta", 2010, Color::ROJO, 1000.0);
        // 25% recargo
        assert_eq!(auto.calcular_precio(), 1250.0);
    }

    #[test]
    fn test_calcular_precio_color_no_primario() {
        let auto = crear_auto("Ford", "Fiesta", 2010, Color::NEGRO, 1000.0);
        // 10% descuento
        assert_eq!(auto.calcular_precio(), 900.0);
    }

    #[test]
    fn test_calcular_precio_marca_bmw() {
        let auto = crear_auto("BMW", "Z1", 2010, Color::NEGRO, 1000.0);
        // primero 10% descuento por color, luego 15% recargo sobre precio con descuento
        let esperado = 900.0 + (900.0 * 0.15);
        assert_eq!(auto.calcular_precio(), esperado);
    }

    #[test]
    fn test_calcular_precio_ano_menor_2000() {
        let auto = crear_auto("Ford", "Ka", 1999, Color::NEGRO, 1000.0);
        // 10% descuento por color → 900, luego 5% descuento → 855
        assert_eq!(auto.calcular_precio(), 855.0);
    }

    #[test]
    fn test_calcular_precio_todos_los_descuentos_y_recargos() {
        let auto = crear_auto("BMW", "Z3", 1995, Color::ROJO, 1000.0);
        // Color primario: +25% → 1250
        // Marca BMW: +15% → 1437.5
        // Año <2000: -5% → 1365.625
        let esperado = 1365.625;
        assert!((auto.calcular_precio() - esperado).abs() < 0.0001);
    }

    #[test]
    fn test_agregar_auto_hasta_capacidad() {
        let mut cons = ConcesionarioAuto::new("Simone".to_string(), "Abasto".to_string(), 2);
        let a1 = crear_auto("Ford", "Fiesta", 2010, Color::NEGRO, 1000.0);
        let a2 = crear_auto("Renault", "Clio", 2011, Color::ROJO, 2000.0);
        let a3 = crear_auto("BMW", "Z1", 2012, Color::AZUL, 3000.0);

        assert_eq!(cons.agregar_auto(a1.clone()), true);
        assert_eq!(cons.agregar_auto(a2.clone()), true);
        // supera capacidad
        assert_eq!(cons.agregar_auto(a3.clone()), false);
    }

    #[test]
    fn test_buscar_auto_en_concesionaria_vacia() {
        let cons = ConcesionarioAuto::new("vacia".to_string(), "Sin autos".to_string(), 5);
        let buscado = crear_auto("Ford", "Fiesta", 2010, Color::NEGRO, 1000.0);
        assert_eq!(cons.buscar_auto(&buscado), None);
    }

    #[test]
    fn test_buscar_auto_existente() {
        let mut cons = ConcesionarioAuto::new("Simone".to_string(), "Abasto".to_string(), 3);
        let a1 = crear_auto("Ford", "Fiesta", 2010, Color::NEGRO, 1000.0);
        let a2 = crear_auto("Renault", "Clio", 2011, Color::ROJO, 2000.0);
        cons.agregar_auto(a1.clone());
        cons.agregar_auto(a2.clone());

        let resultado = cons.buscar_auto(&a2);
        assert!(resultado.is_some());
        assert_eq!(resultado.unwrap().marca, "Renault");
    }

    #[test]
    fn test_buscar_auto_inexistente() {
        let mut cons = ConcesionarioAuto::new("Simone".to_string(), "Abasto".to_string(), 3);
        let a1 = crear_auto("Ford", "Fiesta", 2010, Color::NEGRO, 1000.0);
        cons.agregar_auto(a1.clone());
        let buscado = crear_auto("BMW", "Z1", 2001, Color::ROJO, 3000.0);
        assert_eq!(cons.buscar_auto(&buscado), None);
    }

    #[test]
    fn test_eliminar_auto_en_concesionaria_vacia() {
        let mut cons = ConcesionarioAuto::new("Vacia".to_string(), "Nada".to_string(), 3);
        let buscado = crear_auto("Ford", "Fiesta", 2010, Color::NEGRO, 1000.0);
        assert_eq!(cons.eliminar_auto(&buscado), false);
    }

    #[test]
    fn test_eliminar_auto_existente() {
        let mut cons = ConcesionarioAuto::new("Simone".to_string(), "Abasto".to_string(), 3);
        let a1 = crear_auto("Ford", "Fiesta", 2010, Color::NEGRO, 1000.0);
        let a2 = crear_auto("Renault", "Clio", 2011, Color::ROJO, 2000.0);
        cons.agregar_auto(a1.clone());
        cons.agregar_auto(a2.clone());

        assert_eq!(cons.eliminar_auto(&a1), true);
        // comprobar que ya no está
        assert_eq!(cons.buscar_auto(&a1), None);
    }

    #[test]
    fn test_eliminar_auto_inexistente() {
        let mut cons = ConcesionarioAuto::new("Simone".to_string(), "Abasto".to_string(), 3);
        let a1 = crear_auto("Ford", "Fiesta", 2010, Color::NEGRO, 1000.0);
        let a2 = crear_auto("Renault", "Clio", 2011, Color::ROJO, 2000.0);
        cons.agregar_auto(a1.clone());

        // intento eliminar uno que no está
        assert_eq!(cons.eliminar_auto(&a2), false);
    }
}
