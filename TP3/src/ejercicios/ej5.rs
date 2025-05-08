#[derive(Debug)]
pub struct Producto {
    id: u32,
    nombre: String,
    precio_bruto: f64,
}

#[allow(dead_code)]
impl Producto {
    fn new(id: u32, nombre: String, precio_bruto: f64) -> Producto {
        Producto {
            id: id,
            nombre: nombre,
            precio_bruto: precio_bruto,
        }
    }

    fn calcular_impuestos(&self, porcentaje_imp: Option<f64>) -> f64 {
        if let Some(imp) = porcentaje_imp {
            return self.precio_bruto * imp / 100.0;
        }
        return 0.0;
    }

    fn aplicar_descuento(&self, porcentaje_desc: Option<f64>) -> f64 {
        if let Some(desc) = porcentaje_desc {
            return self.precio_bruto * desc / 100.0;
        }
        return 0.0;
    }

    fn calcular_precio_total(
        &self,
        porcentaje_imp: Option<f64>,
        porcentaje_desc: Option<f64>,
    ) -> f64 {
        return self.precio_bruto + self.calcular_impuestos(porcentaje_imp)
            - self.aplicar_descuento(porcentaje_desc);
    }

    fn to_string(&self) -> String {
        format!(
            "ID: {}, NOMBRE: {}, PRECIO BRUTO:{}",
            self.id, self.nombre, self.precio_bruto
        )
    }
}

#[test]
fn test_producto_aplicar_impuesto() {
    let producto: Producto = Producto::new(1, "Teclado".to_string(), 500.0);
    let mut impuesto: Option<f64> = Some(50.0);
    assert_eq!(250.0, producto.calcular_impuestos(impuesto));

    impuesto = None;
    assert_eq!(0.0, producto.calcular_impuestos(impuesto));
}

#[test]
fn test_producto_aplicar_descuento() {
    let producto: Producto = Producto::new(1, "Teclado".to_string(), 500.0);
    let mut descuento: Option<f64> = Some(50.0);
    assert_eq!(250.0, producto.aplicar_descuento(descuento));

    descuento = None;
    assert_eq!(0.0, producto.aplicar_descuento(descuento));
}

#[test]
fn test_producto_calcular_precio_total() {
    let producto: Producto = Producto::new(1, "Campera".to_string(), 100.0);
    let mut descuento: Option<f64> = Some(25.0);
    let mut impuesto: Option<f64> = Some(10.0);

    assert_eq!(85.0, producto.calcular_precio_total(impuesto, descuento));

    descuento = None;
    impuesto = None;
    assert_eq!(100.0, producto.calcular_precio_total(impuesto, descuento));

    descuento = Some(40.0);
    impuesto = None;
    assert_eq!(60.0, producto.calcular_precio_total(impuesto, descuento));

    descuento = None;
    impuesto = Some(20.0);
    assert_eq!(120.0, producto.calcular_precio_total(impuesto, descuento));
}
