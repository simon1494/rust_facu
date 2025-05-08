#[derive(Debug)]
#[allow(dead_code)]
pub struct Examen {
    materia: String,
    nota: f64,
}
#[allow(dead_code)]
impl Examen {
    fn new(materia: String, nota: f64) -> Examen {
        Examen { materia, nota }
    }
}
#[allow(dead_code)]
#[derive(Debug)]
pub struct Estudiante {
    nombre: String,
    legajo: u32,
    calificaciones: Vec<Examen>,
}
#[allow(dead_code)]
impl Estudiante {
    fn new(nombre: String, legajo: u32, calificaciones: Vec<Examen>) -> Estudiante {
        Estudiante {
            nombre,
            legajo,
            calificaciones,
        }
    }
    fn obtener_promedio(&self) -> f64 {
        let mut contador: u8 = 0;
        let mut total: f64 = 0.0;
        for examen in &self.calificaciones {
            total += examen.nota;
            contador += 1;
        }
        return total / contador as f64;
    }

    fn obtener_calificacion_mas_alta(&self) -> f64 {
        let mut mayor_nota: f64 = -1.0;
        for examen in &self.calificaciones {
            if examen.nota >= mayor_nota {
                mayor_nota = examen.nota
            }
        }
        return mayor_nota;
    }

    fn obtener_calificacion_mas_baja(&self) -> f64 {
        let mut menor_nota: f64 = 11.0;
        for examen in &self.calificaciones {
            if examen.nota <= menor_nota {
                menor_nota = examen.nota
            }
        }
        return menor_nota;
    }
}

#[test]
fn test_estudiante_obtener_promedio() {
    let examen1: Examen = Examen {
        materia: "CADP".to_string(),
        nota: 7.0,
    };

    let examen2: Examen = Examen {
        materia: "Taller".to_string(),
        nota: 7.0,
    };

    let examen3: Examen = Examen {
        materia: "OC".to_string(),
        nota: 5.0,
    };

    let examen4: Examen = Examen {
        materia: "OC".to_string(),
        nota: 9.0,
    };

    let examen5: Examen = Examen {
        materia: "OC".to_string(),
        nota: 4.0,
    };

    let examen6: Examen = Examen {
        materia: "OC".to_string(),
        nota: 10.0,
    };

    let examenes: Vec<Examen> = vec![examen1, examen2, examen3, examen4, examen5, examen6];

    let simon: Estudiante = Estudiante {
        nombre: "Simon".to_string(),
        legajo: 193253,
        calificaciones: examenes,
    };

    assert_eq!(7.0, simon.obtener_promedio())
}

#[test]
fn test_estudiante_obtener_nota_mas_alta() {
    let examen1: Examen = Examen {
        materia: "CADP".to_string(),
        nota: 7.0,
    };

    let examen2: Examen = Examen {
        materia: "Taller".to_string(),
        nota: 7.0,
    };

    let examen3: Examen = Examen {
        materia: "OC".to_string(),
        nota: 5.0,
    };

    let examen4: Examen = Examen {
        materia: "OC".to_string(),
        nota: 9.0,
    };

    let examen5: Examen = Examen {
        materia: "OC".to_string(),
        nota: 4.0,
    };

    let examen6: Examen = Examen {
        materia: "OC".to_string(),
        nota: 10.0,
    };

    let examenes: Vec<Examen> = vec![examen1, examen2, examen3, examen4, examen5, examen6];

    let simon: Estudiante = Estudiante {
        nombre: "Simon".to_string(),
        legajo: 193253,
        calificaciones: examenes,
    };

    assert_eq!(10.0, simon.obtener_calificacion_mas_alta())
}

#[test]
fn test_estudiante_obtener_nota_mas_baja() {
    let examen1: Examen = Examen {
        materia: "CADP".to_string(),
        nota: 7.0,
    };

    let examen2: Examen = Examen {
        materia: "Taller".to_string(),
        nota: 7.0,
    };

    let examen3: Examen = Examen {
        materia: "OC".to_string(),
        nota: 5.0,
    };

    let examen4: Examen = Examen {
        materia: "OC".to_string(),
        nota: 9.0,
    };

    let examen5: Examen = Examen {
        materia: "OC".to_string(),
        nota: 4.0,
    };

    let examen6: Examen = Examen {
        materia: "OC".to_string(),
        nota: 10.0,
    };

    let examenes: Vec<Examen> = vec![examen1, examen2, examen3, examen4, examen5, examen6];

    let simon: Estudiante = Estudiante {
        nombre: "Simon".to_string(),
        legajo: 193253,
        calificaciones: examenes,
    };

    assert_eq!(4.0, simon.obtener_calificacion_mas_baja())
}
