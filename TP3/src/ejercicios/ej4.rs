pub struct Triangle {
    side1: f64,
    side2: f64,
    side3: f64,
}

#[allow(dead_code)]
impl Triangle {
    fn new(side1: f64, side2: f64, side3: f64) -> Triangle {
        if side1 <= 0.0 || side2 <= 0.0 || side3 <= 0.0 {
            panic!("Los lados del triangulo deben ser positivos")
        }
        Triangle {
            side1,
            side2,
            side3,
        }
    }

    fn determinar_tipo(&self) -> &str {
        if self.side1 == self.side2 && self.side2 == self.side3 {
            return "Equilatero";
        }
        if self.side1 == self.side2 || self.side1 == self.side3 || self.side2 == self.side3 {
            return "Isoceles";
        }
        return "Escaleno";
    }

    fn calcular_area(&self) -> f64 {
        let s = (self.side1 + self.side2 + self.side3) / 2.0;
        let area = (s * (s - self.side1) * (s - self.side2) * (s - self.side3)).sqrt();
        area
    }

    fn calcular_perimetro(&self) -> f64 {
        return self.side1 + self.side2 + self.side3;
    }
}

#[test]
fn test_triangulo_determinar_tipo() {
    let triangulo: Triangle = Triangle::new(1.2, 4.2, 5.3);
    assert_eq!("Escaleno", triangulo.determinar_tipo());

    let triangulo: Triangle = Triangle::new(1.2, 4.2, 1.2);
    assert_eq!("Isoceles", triangulo.determinar_tipo());

    let triangulo: Triangle = Triangle::new(1.2, 1.2, 1.2);
    assert_eq!("Equilatero", triangulo.determinar_tipo());
}
