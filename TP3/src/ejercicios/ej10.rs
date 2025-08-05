use crate::ejercicios::ej3::Fecha;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(dead_code)]
pub enum Errores {
    UsuarioSinHistorial,
    SinHistorialParaEseEstado,
}

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

#[derive(Debug, Clone, PartialEq)]
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
        if self.correo_existente(cliente) {
            return false;
        }

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
            if &p.libro == libro
                && p.cliente.correo == cliente.correo
                && p.cliente.nombre == cliente.nombre
            {
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

    fn correo_existente(&self, cliente: &Cliente) -> bool {
        self.prestamos
            .iter()
            .any(|p| p.cliente.correo == cliente.correo && p.cliente.nombre != cliente.nombre)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        // TEST 29-02-2024 > 28-02-2024 ✅
        let fecha: Fecha = Fecha::new(29, 02, 2024);
        let otra_fecha: Fecha = Fecha::new(28, 02, 2024);
        assert_eq!(true, fecha.es_mayor(otra_fecha));

        // TEST 01-02-2024 > 28-02-2024 ❌
        let fecha: Fecha = Fecha::new(1, 02, 2024);
        let otra_fecha: Fecha = Fecha::new(28, 02, 2024);
        assert_eq!(false, fecha.es_mayor(otra_fecha));

        // TEST 01-02-2024 > 28-02-1985 ✅
        let fecha: Fecha = Fecha::new(1, 02, 2024);
        let otra_fecha: Fecha = Fecha::new(28, 02, 1985);
        assert_eq!(true, fecha.es_mayor(otra_fecha));

        // TEST 01-02-2024 > 28-01-2024 ✅
        let fecha: Fecha = Fecha::new(1, 02, 2024);
        let otra_fecha: Fecha = Fecha::new(28, 01, 2024);
        assert_eq!(true, fecha.es_mayor(otra_fecha));
    }

    #[test]
    fn test_fecha_es_fecha_valida() {
        // fecha -> 1/1/2025 ✅ - TEST LIMITES NORMALES
        let fecha: Fecha = Fecha::new(1, 1, 2025);
        assert_eq!(true, fecha.es_fecha_valida());

        // fecha -> 30/4/1985 ✅ - TEST LIMITE SUPERIOR PARA MES CORTO
        let fecha: Fecha = Fecha::new(30, 4, 1985);
        assert_eq!(true, fecha.es_fecha_valida());

        // fecha -> 31/12/2001 ✅ - TEST LIMITE SUPERIOR PARA MES LARGO
        let fecha: Fecha = Fecha::new(31, 12, 2001);
        assert_eq!(true, fecha.es_fecha_valida());

        // fecha -> 29/2/2024 ✅ - TEST LIMITE SUPERIOR PARA FEBRERO BISIESTO
        let fecha: Fecha = Fecha::new(29, 2, 2024);
        assert_eq!(true, fecha.es_fecha_valida());

        // fecha -> 29/2/1900 ❌ - TEST LIMITE SUPERIOR PARA FEBRERO NO BISIESTO
        let fecha: Fecha = Fecha::new(29, 02, 1900);
        assert_eq!(false, fecha.es_fecha_valida());

        // fecha -> 0/1/2025 ❌ - TEST DIA 0
        let fecha: Fecha = Fecha::new(0, 1, 2025);
        assert_eq!(false, fecha.es_fecha_valida());

        // fecha -> 31/02/2024 ❌ - TEST FEBRERO CON 31 DIAS
        let fecha: Fecha = Fecha::new(31, 2, 2025);
        assert_eq!(false, fecha.es_fecha_valida());

        // fecha -> 31/4/1690 ❌ - TEST MES CORTO CON 31 DIAS
        let fecha: Fecha = Fecha::new(31, 4, 2025);
        assert_eq!(false, fecha.es_fecha_valida());

        // fecha -> 1/1/2026 ✅ - TEST AÑO MAYOR AL ACTUAL
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

    #[test]
    fn test_realizar_prestamo_con_igual_correo() {
        let libro = crear_libro();
        let cliente_original = Cliente {
            nombre: "Mandinga".to_string(),
            telefono: "1234651346".to_string(),
            correo: "mandingueitor@mail.com".to_string(),
        };
        let cliente_distinto = Cliente {
            nombre: "Otro Mandinga".to_string(),
            telefono: "34623462".to_string(),
            correo: "mandingueitor@mail.com".to_string(), // mismo correo
        };

        let mut biblioteca = Biblioteca::new("BN", "LP");
        biblioteca.incrementar_copias(&libro);
        biblioteca.realizar_prestamo(&libro, &cliente_original, 3);

        // intento de duplicado
        biblioteca.incrementar_copias(&libro);
        let resultado = biblioteca.realizar_prestamo(&libro, &cliente_distinto, 3);

        assert!(
            !resultado,
            "Ya hay otro cliente con el mismo correo pillin. Toca de acaaaa"
        );
    }

    #[test]
    fn test_mismo_cliente_puede_tener_varios_prestamos() {
        let libro1 = crear_libro();
        let libro2 = Libro {
            isbn: "999".to_string(),
            titulo: "Libro 2".to_string(),
            autor: "Autor X".to_string(),
            paginas: 100,
            genero: Genero::Otros,
        };
        let cliente = crear_cliente();
        let mut biblioteca = Biblioteca::new("BN", "LP");

        biblioteca.incrementar_copias(&libro1);
        biblioteca.incrementar_copias(&libro2);

        let r1 = biblioteca.realizar_prestamo(&libro1, &cliente, 5);
        let r2 = biblioteca.realizar_prestamo(&libro2, &cliente, 5);

        assert!(
            r1 && r2,
            "El mismo cliente debería poder tener múltiples préstamos activos."
        );
    }

    #[test]
    fn test_buscar_prestamo_no_retorna_prestamo_de_otro_cliente() {
        let libro = crear_libro();
        let cliente_original = crear_cliente();
        let cliente_distinto = Cliente {
            nombre: "Otro".to_string(),
            telefono: "000".to_string(),
            correo: cliente_original.correo.clone(), // mismo correo
        };

        let mut biblioteca = Biblioteca::new("BN", "LP");
        biblioteca.incrementar_copias(&libro);
        biblioteca.realizar_prestamo(&libro, &cliente_original, 5);

        let encontrado = biblioteca.buscar_prestamo(&libro, &cliente_distinto);
        assert!(
            encontrado.is_none(),
            "No debe devolver préstamo de otro cliente con el mismo correo"
        );
    }

    #[test]
    fn test_no_se_puede_devolver_libro_de_otro_cliente() {
        let libro = crear_libro();
        let cliente_original = crear_cliente();
        let cliente_incorrecto = Cliente {
            nombre: "Invasor".to_string(),
            telefono: "0000".to_string(),
            correo: "otro@mail.com".to_string(),
        };

        let mut biblioteca = Biblioteca::new("BN", "LP");
        biblioteca.incrementar_copias(&libro);
        biblioteca.realizar_prestamo(&libro, &cliente_original, 3);

        // Intento devolver el libro con un cliente que no tiene el préstamo
        let resultado = biblioteca.devolver_libro(&libro, &cliente_incorrecto);
        assert!(
            !resultado,
            "No debe permitir devolver un libro que no pertenece a este cliente"
        );
        assert_eq!(
            biblioteca.obtener_copias(&libro),
            0,
            "Las copias no deben incrementarse"
        );
    }

    #[test]
    fn test_no_se_puede_devolver_libro_dos_veces() {
        let libro = crear_libro();
        let cliente = crear_cliente();

        let mut biblioteca = Biblioteca::new("BN", "LP");
        biblioteca.incrementar_copias(&libro);
        biblioteca.realizar_prestamo(&libro, &cliente, 3);

        // Primera devolución: válida
        assert!(biblioteca.devolver_libro(&libro, &cliente));
        assert_eq!(
            biblioteca.obtener_copias(&libro),
            1,
            "Las copias deben incrementarse en la primera devolución"
        );

        // Segunda devolución: debe fallar y no alterar copias
        assert!(
            !biblioteca.devolver_libro(&libro, &cliente),
            "No debe permitir devolver dos veces el mismo préstamo"
        );
        assert_eq!(
            biblioteca.obtener_copias(&libro),
            1,
            "Las copias no deben incrementarse en una devolución repetida"
        );
    }
}
