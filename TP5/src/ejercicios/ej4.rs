use crate::ejercicios::fecha::Fecha;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize)]
#[allow(dead_code)]
struct Biblio {
    copias: HashMap<String, u32>,
    prestamos: Vec<Prestamo>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[allow(dead_code)]
enum Genero {
    Novela,
    Infantil,
    Tecnico,
    Otros,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[allow(dead_code)]
struct Libro {
    isbn: String,
    titulo: String,
    autor: String,
    paginas: u32,
    genero: Genero,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
struct Cliente {
    nombre: String,
    telefono: String,
    correo: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(dead_code)]
enum EstadoPrestamo {
    EnPrestamo,
    Devuelto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
struct Prestamo {
    libro: Libro,
    cliente: Cliente,
    fecha_vencimiento: Fecha,
    fecha_devolucion: Option<Fecha>,
    estado: EstadoPrestamo,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
struct Biblioteca {
    nombre: String,
    direccion: String,
    copias: HashMap<String, u32>,
    prestamos: Vec<Prestamo>,
    ruta_archivo: String,
}

#[allow(dead_code)]
impl Biblioteca {
    pub fn new(nombre: &str, direccion: &str, ruta_archivo: &str) -> Self {
        Biblioteca {
            nombre: nombre.to_string(),
            direccion: direccion.to_string(),
            copias: HashMap::new(),
            prestamos: Vec::new(),
            ruta_archivo: ruta_archivo.to_string(),
        }
    }

    pub fn obtener_copias(&self, libro: &Libro) -> u32 {
        *self.copias.get(&libro.isbn).unwrap_or(&0)
    }

    pub fn restar_copias(&mut self, libro: &Libro) {
        if let Some(cantidad) = self.copias.get_mut(&libro.isbn) {
            if *cantidad > 0 {
                *cantidad -= 1;
            }
        }
        self.guardar_archivo();
    }

    pub fn sumar_copias(&mut self, libro: &Libro) {
        *self.copias.entry(libro.isbn.clone()).or_insert(0) += 1;
        self.guardar_archivo();
    }

    pub fn contar_prestamos_cliente(&self, cliente: &Cliente) -> u32 {
        self.prestamos
            .iter()
            .filter(|p| {
                p.cliente.correo == cliente.correo && p.estado == EstadoPrestamo::EnPrestamo
            })
            .count() as u32
    }

    pub fn realizar_prestamo(&mut self, libro: &Libro, cliente: &Cliente, dias: u32) -> bool {
        if self.contar_prestamos_cliente(cliente) >= 5 {
            return false;
        }

        if self.obtener_copias(libro) == 0 {
            return false;
        }

        let mut fecha_vencimiento = Fecha::hoy();
        fecha_vencimiento.sumar_dias(dias);

        let prestamo = Prestamo {
            libro: libro.clone(),
            cliente: cliente.clone(),
            fecha_vencimiento,
            fecha_devolucion: None,
            estado: EstadoPrestamo::EnPrestamo,
        };

        self.restar_copias(libro);
        self.prestamos.push(prestamo);
        self.guardar_archivo();
        true
    }

    pub fn prestamos_a_vencer(&self, dias: u32) -> Vec<&Prestamo> {
        let mut fecha_limite = Fecha::hoy();
        fecha_limite.sumar_dias(dias);
        let hoy = Fecha::hoy();

        self.prestamos
            .iter()
            .filter(|p| {
                p.estado == EstadoPrestamo::EnPrestamo
                    && !p.fecha_vencimiento.es_mayor(fecha_limite.clone())
                    && !hoy.es_mayor(p.fecha_vencimiento.clone())
            })
            .collect()
    }

    pub fn prestamos_vencidos(&self) -> Vec<&Prestamo> {
        let hoy = Fecha::hoy();
        self.prestamos
            .iter()
            .filter(|p| {
                p.estado == EstadoPrestamo::EnPrestamo && hoy.es_mayor(p.fecha_vencimiento.clone())
            })
            .collect()
    }

    pub fn buscar_prestamo(&self, libro: &Libro, cliente: &Cliente) -> Option<&Prestamo> {
        self.prestamos
            .iter()
            .find(|p| p.libro.isbn == libro.isbn && p.cliente.correo == cliente.correo)
    }

    pub fn devolver_libro(&mut self, libro: &Libro, cliente: &Cliente) -> bool {
        if let Some(p) = self.prestamos.iter_mut().find(|p| {
            p.libro.isbn == libro.isbn
                && p.cliente.correo == cliente.correo
                && p.estado == EstadoPrestamo::EnPrestamo
        }) {
            p.estado = EstadoPrestamo::Devuelto;
            p.fecha_devolucion = Some(Fecha::hoy());
            self.sumar_copias(libro);
            return true;
        }
        false
    }

    fn guardar_archivo(&self) {
        let estado = Biblio {
            copias: self.copias.clone(),
            prestamos: self.prestamos.clone(),
        };

        if let Ok(json) = serde_json::to_string_pretty(&estado) {
            let _ = std::fs::write(&self.ruta_archivo, json);
        }
    }

    fn leer_archivo(ruta: &str) -> (HashMap<String, u32>, Vec<Prestamo>) {
        if let Ok(file) = File::open(ruta) {
            if let Ok(estado) = serde_json::from_reader::<_, Biblio>(BufReader::new(file)) {
                return (estado.copias, estado.prestamos);
            }
        }
        (HashMap::new(), Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ejercicios::fecha::Fecha;

    fn un_librito() -> Libro {
        Libro {
            isbn: "9789875668751".to_string(),
            titulo: "Memoria de mis putas tristes".to_string(),
            autor: "GG marquez".to_string(),
            paginas: 400,
            genero: Genero::Novela,
        }
    }

    fn un_gil() -> Cliente {
        Cliente {
            nombre: "Tu vieja en tanga".to_string(),
            telefono: "1134567890".to_string(),
            correo: "vieja@mail.com".to_string(),
        }
    }

    #[test]
    fn test_obtener_copias() {
        let libro = un_librito();
        let ruta = "test_biblio1.json";
        let mut biblioteca = Biblioteca::new("Biblioteca Nacional de La Plata", "La Plata", ruta);
        biblioteca.sumar_copias(&libro);
        biblioteca.sumar_copias(&libro);
        assert_eq!(biblioteca.obtener_copias(&libro), 2);
        let _ = std::fs::remove_file(ruta);
    }

    #[test]
    fn test_restar_copias() {
        let libro = un_librito();
        let ruta = "test_biblio2.json";
        let mut biblioteca = Biblioteca::new("BN", "LP", ruta);
        biblioteca.sumar_copias(&libro);
        biblioteca.restar_copias(&libro);
        assert_eq!(biblioteca.obtener_copias(&libro), 0);
        let _ = std::fs::remove_file(ruta);
    }

    #[test]
    fn test_sumar_copias() {
        let libro = un_librito();
        let ruta = "test_biblio3.json";
        let mut biblioteca = Biblioteca::new("BN", "LP", ruta);
        biblioteca.sumar_copias(&libro);
        assert_eq!(biblioteca.obtener_copias(&libro), 1);
        let _ = std::fs::remove_file(ruta);
    }

    #[test]
    fn test_contar_prestamos_cliente() {
        let ruta = "test_biblio4.json";
        let libro = un_librito();
        let cliente = un_gil();
        let mut biblioteca = Biblioteca::new("BN", "LP", ruta);
        biblioteca.sumar_copias(&libro);
        biblioteca.realizar_prestamo(&libro, &cliente, 5);
        assert_eq!(biblioteca.contar_prestamos_cliente(&cliente), 1);
        let _ = std::fs::remove_file(ruta);
    }

    #[test]
    fn test_realizar_prestamo_exitoso() {
        let libro = un_librito();
        let ruta = "test_biblio3.json";
        let cliente = un_gil();
        let mut biblioteca = Biblioteca::new("BN", "LP", ruta);
        biblioteca.sumar_copias(&libro);
        let resultado = biblioteca.realizar_prestamo(&libro, &cliente, 7);
        assert!(resultado);
        assert_eq!(biblioteca.obtener_copias(&libro), 0);
        let _ = std::fs::remove_file(ruta);
    }

    #[test]
    fn test_realizar_prestamo_falla_por_copias() {
        let ruta = "test_biblio6.json";
        let libro = un_librito();
        let cliente = un_gil();
        let mut biblioteca = Biblioteca::new("BN", "LP", ruta);
        let resultado = biblioteca.realizar_prestamo(&libro, &cliente, 7);
        assert!(!resultado);
        let _ = std::fs::remove_file(ruta);
    }

    #[test]
    fn test_realizar_prestamo_falla_por_limite_cliente() {
        let libro = un_librito();
        let ruta = "test_biblio7.json";
        let cliente = un_gil();
        let mut biblioteca = Biblioteca::new("BN", "LP", ruta);
        let isbn = "12343456".to_string();

        biblioteca.copias.insert(isbn.clone(), 10);

        for _ in 0..5 {
            biblioteca.realizar_prestamo(&libro, &cliente, 3);
        }

        // Sexto intento debe fallar
        let resultado = biblioteca.realizar_prestamo(&libro, &cliente, 3);
        assert!(!resultado);
        let _ = std::fs::remove_file(ruta);
    }

    #[test]
    fn test_prestamos_a_vencer() {
        let libro = un_librito();
        let cliente = un_gil();
        let ruta = "test_biblio8.json";
        let mut biblioteca = Biblioteca::new("BN", "LP", ruta);

        biblioteca.sumar_copias(&libro);
        biblioteca.realizar_prestamo(&libro, &cliente, 3);

        let vencen = biblioteca.prestamos_a_vencer(5);
        assert_eq!(vencen.len(), 1);
        let _ = std::fs::remove_file(ruta);
    }

    #[test]
    fn test_prestamos_vencidos() {
        let libro = un_librito();
        let cliente = un_gil();
        let ruta = "test_biblio9.json";
        let mut biblioteca = Biblioteca::new("BN", "LP", ruta);

        biblioteca.sumar_copias(&libro);
        let mut fecha_vencida = Fecha::hoy();
        fecha_vencida.restar_dias(10);

        let prestamo = Prestamo {
            libro: libro.clone(),
            cliente: cliente.clone(),
            fecha_vencimiento: fecha_vencida,
            fecha_devolucion: None,
            estado: EstadoPrestamo::EnPrestamo,
        };

        biblioteca.prestamos.push(prestamo);
        let vencidos = biblioteca.prestamos_vencidos();
        assert_eq!(vencidos.len(), 1);
        let _ = std::fs::remove_file(ruta);
    }

    #[test]
    fn test_buscar_prestamo_existente() {
        let libro = un_librito();
        let cliente = un_gil();
        let ruta = "test_biblio10.json";
        let mut biblioteca = Biblioteca::new("BN", "LP", ruta);

        biblioteca.sumar_copias(&libro);
        biblioteca.realizar_prestamo(&libro, &cliente, 3);

        let prestamo = biblioteca.buscar_prestamo(&libro, &cliente);
        assert!(prestamo.is_some());
        let _ = std::fs::remove_file(ruta);
    }

    #[test]
    fn test_devolver_libro() {
        let libro = un_librito();
        let cliente = un_gil();
        let ruta = "test_biblio11.json";
        let mut biblioteca = Biblioteca::new("BN", "LP", ruta);

        biblioteca.sumar_copias(&libro);
        biblioteca.realizar_prestamo(&libro, &cliente, 3);

        let ok = biblioteca.devolver_libro(&libro, &cliente);
        assert!(ok);
        assert_eq!(biblioteca.obtener_copias(&libro), 1);
        let _ = std::fs::remove_file(ruta);
    }

    #[test]
    fn test_guardar_y_leer_archivo() {
        let ruta = "test_biblio12.json";
        let libro = un_librito();
        let cliente = un_gil();
        let mut biblioteca = Biblioteca::new("BN", "LP", ruta);

        biblioteca.sumar_copias(&libro);
        biblioteca.realizar_prestamo(&libro, &cliente, 7);
        biblioteca.guardar_archivo();

        let (copias, prestamos) = Biblioteca::leer_archivo(ruta);
        assert_eq!(copias.get(&libro.isbn), Some(&0));
        assert_eq!(prestamos.len(), 1);
        assert_eq!(prestamos[0].cliente.correo, cliente.correo);
        assert_eq!(prestamos[0].libro.titulo, libro.titulo);

        let _ = std::fs::remove_file(ruta);
    }

    #[test]
    fn test_leer_archivo_inexistente_devuelve_vacios() {
        let ruta = "test_biblio13.json";
        let (copias, prestamos) = Biblioteca::leer_archivo(ruta);
        assert!(copias.is_empty());
        assert!(prestamos.is_empty());
    }
}
