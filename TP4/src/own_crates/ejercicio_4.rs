use crate::own_crates::fecha::Fecha;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Categoria {
    Alimentos,
    Limpieza,
    Tecnologia,
    Indumentaria,
    Otros,
}

#[derive(Debug, Clone)]
struct Producto {
    nombre: String,
    categoria: Categoria,
    precio_base: f32,
}

#[derive(Debug, Clone)]
struct Cliente {
    nombre: String,
    apellido: String,
    direccion: String,
    dni: String,
    suscripto_newsletter: bool,
    correo: Option<String>,
}

#[derive(Debug, Clone)]
struct Vendedor {
    nombre: String,
    apellido: String,
    direccion: String,
    dni: String,
    legajo: u32,
    antiguedad: u32,
    salario: f32,
}

#[derive(Debug, Clone)]
enum MedioPago {
    Credito,
    Debito,
    Transferencia,
    Efectivo,
}

#[derive(Debug, Clone)]
struct Venta {
    fecha: Fecha,
    cliente: Cliente,
    vendedor: Vendedor,
    medio_pago: MedioPago,
    productos: Vec<(Producto, u32)>, // producto y cantidad
}

impl Venta {
    pub fn nueva(
        fecha: Fecha,
        cliente: Cliente,
        vendedor: Vendedor,
        medio_pago: MedioPago,
        productos: Vec<(Producto, u32)>,
    ) -> Self {
        Venta {
            fecha,
            cliente,
            vendedor,
            medio_pago,
            productos,
        }
    }

    pub fn calcular_precio_final(
        &self,
        descuentos_categoria: &HashMap<Categoria, f32>,
        descuento_newsletter: f32,
    ) -> f32 {
        let mut total = 0.0;

        for (producto, cantidad) in &self.productos {
            let mut precio_unitario = producto.precio_base;

            if let Some(desc) = descuentos_categoria.get(&producto.categoria) {
                precio_unitario *= 1.0 - (*desc / 100.0);
            }

            total += precio_unitario * (*cantidad as f32);
        }

        if self.cliente.suscripto_newsletter {
            total *= 1.0 - (descuento_newsletter / 100.0);
        }

        total
    }
}

#[derive(Default)]
struct SistemaVentas {
    ventas: Vec<Venta>,
}

impl SistemaVentas {
    pub fn nueva() -> Self {
        SistemaVentas { ventas: vec![] }
    }

    pub fn registrar_venta(&mut self, venta: Venta) {
        self.ventas.push(venta);
    }

    pub fn reporte_por_categoria(&self) -> HashMap<Categoria, f32> {
        let mut total_por_categoria: HashMap<Categoria, f32> = HashMap::new();

        for venta in &self.ventas {
            for (producto, cantidad) in &venta.productos {
                let subtotal = producto.precio_base * (*cantidad as f32);
                *total_por_categoria
                    .entry(producto.categoria.clone())
                    .or_insert(0.0) += subtotal;
            }
        }

        total_por_categoria
    }

    pub fn reporte_por_vendedor(&self) -> HashMap<u32, f32> {
        let mut total_por_vendedor: HashMap<u32, f32> = HashMap::new();

        for venta in &self.ventas {
            let subtotal: f32 = venta
                .productos
                .iter()
                .map(|(p, cant)| p.precio_base * (*cant as f32))
                .sum();

            *total_por_vendedor
                .entry(venta.vendedor.legajo)
                .or_insert(0.0) += subtotal;
        }

        total_por_vendedor
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::own_crates::fecha::Fecha;

    fn el_carlos() -> Cliente {
        Cliente {
            nombre: "carlito".to_string(),
            apellido: "gomez".to_string(),
            direccion: "calle 12 y 61".to_string(),
            dni: "11111111".to_string(),
            suscripto_newsletter: true,
            correo: Some("roberto@gmail.com".to_string()),
        }
    }

    fn vendedor_hdp() -> Vendedor {
        Vendedor {
            nombre: "Matute".to_string(),
            apellido: "Garca".to_string(),
            direccion: "diagonal 80 1234".to_string(),
            dni: "25123123".to_string(),
            legajo: 101,
            antiguedad: 5,
            salario: 200000.0,
        }
    }

    fn clavos() -> Producto {
        Producto {
            nombre: "clavos 2 pulgadas".to_string(),
            categoria: Categoria::Otros,
            precio_base: 500.0,
        }
    }

    fn martillo() -> Producto {
        Producto {
            nombre: "martillo carpintero".to_string(),
            categoria: Categoria::Tecnologia,
            precio_base: 3500.0,
        }
    }

    fn pintura() -> Producto {
        Producto {
            nombre: "pintura latex blanca".to_string(),
            categoria: Categoria::Limpieza,
            precio_base: 4200.0,
        }
    }

    #[test]
    fn test_crear_venta() {
        let venta = Venta::nueva(
            Fecha::new(1, 7, 2025),
            el_carlos(),
            vendedor_hdp(),
            MedioPago::Efectivo,
            vec![(clavos(), 2)],
        );

        assert_eq!(venta.productos.len(), 1);
        assert_eq!(venta.cliente.nombre, "carlito");
        assert_eq!(venta.vendedor.legajo, 101);
    }

    #[test]
    fn test_calcular_precio_final_con_descuentos() {
        let venta = Venta::nueva(
            Fecha::new(2, 7, 2025),
            el_carlos(),
            vendedor_hdp(),
            MedioPago::Debito,
            vec![(martillo(), 1), (clavos(), 2)],
        );

        let mut descuentos = HashMap::new();
        descuentos.insert(Categoria::Tecnologia, 10.0);
        descuentos.insert(Categoria::Otros, 5.0);

        let total = venta.calcular_precio_final(&descuentos, 5.0);

        assert_eq!(total.round() as i32, 3895);
    }

    #[test]
    fn test_registrar_venta_en_sistema() {
        let mut sistema = SistemaVentas::nueva();
        let venta = Venta::nueva(
            Fecha::new(3, 7, 2025),
            el_carlos(),
            vendedor_hdp(),
            MedioPago::Transferencia,
            vec![(pintura(), 1)],
        );
        sistema.registrar_venta(venta);
        assert_eq!(sistema.ventas.len(), 1);
    }

    #[test]
    fn test_reporte_por_categoria() {
        let mut sistema = SistemaVentas::nueva();
        let venta = Venta::nueva(
            Fecha::new(4, 7, 2025),
            el_carlos(),
            vendedor_hdp(),
            MedioPago::Credito,
            vec![(martillo(), 1), (pintura(), 1)],
        );
        sistema.registrar_venta(venta);
        let reporte = sistema.reporte_por_categoria();
        assert_eq!(reporte.get(&Categoria::Tecnologia).unwrap(), &3500.0);
        assert_eq!(reporte.get(&Categoria::Limpieza).unwrap(), &4200.0);
    }

    #[test]
    fn test_reporte_por_vendedor() {
        let mut sistema = SistemaVentas::nueva();
        let venta1 = Venta::nueva(
            Fecha::new(5, 7, 2025),
            el_carlos(),
            vendedor_hdp(),
            MedioPago::Efectivo,
            vec![(clavos(), 3)],
        );
        let venta2 = Venta::nueva(
            Fecha::new(6, 7, 2025),
            el_carlos(),
            vendedor_hdp(),
            MedioPago::Debito,
            vec![(martillo(), 1)],
        );
        sistema.registrar_venta(venta1);
        sistema.registrar_venta(venta2);
        let reporte = sistema.reporte_por_vendedor();
        assert_eq!(reporte.get(&101).unwrap(), &(1500.0 + 3500.0));
    }
}
