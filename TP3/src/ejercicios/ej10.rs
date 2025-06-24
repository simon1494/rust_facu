use crate::ejercicios::ej3::Fecha;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)]
enum Genero {
    Novela,
    Infantil,
    Tecnico,
    Otros,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)]
struct Libro {
    isbn: String,
    titulo: String,
    autor: String,
    paginas: u32,
    genero: Genero,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Cliente {
    nombre: String,
    telefono: String,
    correo: String,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
enum EstadoPrestamo {
    EnPrestamo,
    Devuelto,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Prestamo {
    libro: Libro,
    cliente: Cliente,
    fecha_vencimiento: Fecha,
    fecha_devolucion: Option<Fecha>,
    estado: EstadoPrestamo,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Biblioteca {
    nombre: String,
    direccion: String,
    copias: HashMap<Libro, u32>,
    prestamos: Vec<Prestamo>,
}

#[allow(dead_code)]
impl Biblioteca {
    fn new(nombre: &str, direccion: &str) -> Self {
        Biblioteca {
            nombre: nombre.to_string(),
            direccion: direccion.to_string(),
            copias: HashMap::new(),
            prestamos: Vec::new(),
        }
    }

    pub fn obtener_copias(&self, libro: &Libro) -> u32 {
        *self.copias.get(libro).unwrap_or(&0)
    }

    pub fn decrementar_copias(&mut self, libro: &Libro) {
        if let Some(cantidad) = self.copias.get_mut(libro) {
            if *cantidad > 0 {
                *cantidad -= 1;
            }
        }
    }

    pub fn incrementar_copias(&mut self, libro: &Libro) {
        *self.copias.entry(libro.clone()).or_insert(0) += 1;
    }

    pub fn contar_prestamos_cliente(&self, cliente: &Cliente) -> u32 {
        let mut contador = 0;
        for prestamo in &self.prestamos {
            if prestamo.cliente.correo == cliente.correo
                && prestamo.estado == EstadoPrestamo::EnPrestamo
            {
                contador += 1;
            }
        }
        contador
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

        self.decrementar_copias(libro);
        self.prestamos.push(prestamo);
        true
    }

    pub fn prestamos_a_vencer(&self, dias: u32) -> Vec<&Prestamo> {
        let mut fecha_limite = Fecha::hoy();
        fecha_limite.sumar_dias(dias);
        let hoy = Fecha::hoy();

        let mut resultado = Vec::new();
        for p in &self.prestamos {
            if p.estado == EstadoPrestamo::EnPrestamo
                && !p.fecha_vencimiento.es_mayor(fecha_limite)
                && !hoy.es_mayor(p.fecha_vencimiento)
            {
                resultado.push(p);
            }
        }
        resultado
    }

    pub fn prestamos_vencidos(&self) -> Vec<&Prestamo> {
        let hoy = Fecha::hoy();
        let mut resultado = Vec::new();
        for p in &self.prestamos {
            if p.estado == EstadoPrestamo::EnPrestamo && hoy.es_mayor(p.fecha_vencimiento) {
                resultado.push(p);
            }
        }
        resultado
    }

    pub fn buscar_prestamo(&self, libro: &Libro, cliente: &Cliente) -> Option<&Prestamo> {
        for p in &self.prestamos {
            if &p.libro == libro && p.cliente.correo == cliente.correo {
                return Some(p);
            }
        }
        None
    }

    pub fn devolver_libro(&mut self, libro: &Libro, cliente: &Cliente) -> bool {
        for p in &mut self.prestamos {
            if &p.libro == libro
                && p.cliente.correo == cliente.correo
                && p.estado == EstadoPrestamo::EnPrestamo
            {
                p.estado = EstadoPrestamo::Devuelto;
                p.fecha_devolucion = Some(Fecha::hoy());
                self.incrementar_copias(libro);
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ejercicios::ej3::Fecha;

    fn crear_libro() -> Libro {
        Libro {
            isbn: "9789875668751".to_string(),
            titulo: "Rayuela".to_string(),
            autor: "Julio Cortazar".to_string(),
            paginas: 400,
            genero: Genero::Novela,
        }
    }

    fn crear_cliente() -> Cliente {
        Cliente {
            nombre: "Carlos Gonzalez".to_string(),
            telefono: "1134567890".to_string(),
            correo: "carlos@mail.com".to_string(),
        }
    }

    #[test]
    fn test_obtener_copias() {
        let libro = crear_libro();
        let mut biblioteca = Biblioteca::new("Biblioteca Nacional de La Plata", "La Plata");
        biblioteca.incrementar_copias(&libro);
        biblioteca.incrementar_copias(&libro);
        assert_eq!(biblioteca.obtener_copias(&libro), 2);
    }

    #[test]
    fn test_decrementar_copias() {
        let libro = crear_libro();
        let mut biblioteca = Biblioteca::new("BN", "LP");
        biblioteca.incrementar_copias(&libro);
        biblioteca.decrementar_copias(&libro);
        assert_eq!(biblioteca.obtener_copias(&libro), 0);
    }

    #[test]
    fn test_incrementar_copias() {
        let libro = crear_libro();
        let mut biblioteca = Biblioteca::new("BN", "LP");
        biblioteca.incrementar_copias(&libro);
        assert_eq!(biblioteca.obtener_copias(&libro), 1);
    }

    #[test]
    fn test_contar_prestamos_cliente() {
        let libro = crear_libro();
        let cliente = crear_cliente();
        let mut biblioteca = Biblioteca::new("BN", "LP");
        biblioteca.incrementar_copias(&libro);
        biblioteca.realizar_prestamo(&libro, &cliente, 5);
        assert_eq!(biblioteca.contar_prestamos_cliente(&cliente), 1);
    }

    #[test]
    fn test_realizar_prestamo_exitoso() {
        let libro = crear_libro();
        let cliente = crear_cliente();
        let mut biblioteca = Biblioteca::new("BN", "LP");
        biblioteca.incrementar_copias(&libro);
        let resultado = biblioteca.realizar_prestamo(&libro, &cliente, 7);
        assert!(resultado);
        assert_eq!(biblioteca.obtener_copias(&libro), 0);
    }

    #[test]
    fn test_realizar_prestamo_falla_por_copias() {
        let libro = crear_libro();
        let cliente = crear_cliente();
        let mut biblioteca = Biblioteca::new("BN", "LP");
        let resultado = biblioteca.realizar_prestamo(&libro, &cliente, 7);
        assert!(!resultado);
    }

    #[test]
    fn test_realizar_prestamo_falla_por_limite_cliente() {
        let libro = crear_libro();
        let cliente = crear_cliente();
        let mut biblioteca = Biblioteca::new("BN", "LP");

        biblioteca.copias.insert(libro.clone(), 10);

        for _ in 0..5 {
            biblioteca.realizar_prestamo(&libro, &cliente, 3);
        }

        // Sexto intento debe fallar
        let resultado = biblioteca.realizar_prestamo(&libro, &cliente, 3);
        assert!(!resultado);
    }

    #[test]
    fn test_prestamos_a_vencer() {
        let libro = crear_libro();
        let cliente = crear_cliente();
        let mut biblioteca = Biblioteca::new("BN", "LP");

        biblioteca.incrementar_copias(&libro);
        biblioteca.realizar_prestamo(&libro, &cliente, 3);

        let vencen = biblioteca.prestamos_a_vencer(5);
        assert_eq!(vencen.len(), 1);
    }

    #[test]
    fn test_prestamos_vencidos() {
        let libro = crear_libro();
        let cliente = crear_cliente();
        let mut biblioteca = Biblioteca::new("BN", "LP");

        biblioteca.incrementar_copias(&libro);
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
    }

    #[test]
    fn test_buscar_prestamo_existente() {
        let libro = crear_libro();
        let cliente = crear_cliente();
        let mut biblioteca = Biblioteca::new("BN", "LP");

        biblioteca.incrementar_copias(&libro);
        biblioteca.realizar_prestamo(&libro, &cliente, 3);

        let prestamo = biblioteca.buscar_prestamo(&libro, &cliente);
        assert!(prestamo.is_some());
    }

    #[test]
    fn test_devolver_libro() {
        let libro = crear_libro();
        let cliente = crear_cliente();
        let mut biblioteca = Biblioteca::new("BN", "LP");

        biblioteca.incrementar_copias(&libro);
        biblioteca.realizar_prestamo(&libro, &cliente, 3);

        let ok = biblioteca.devolver_libro(&libro, &cliente);
        assert!(ok);
        assert_eq!(biblioteca.obtener_copias(&libro), 1);
    }
}
