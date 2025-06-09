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
pub struct Informe {
    nombre_estudiante: String,
    cant_examenes_rendidos: u16,
    promedio_general: f64,
    mayor_nota: f64,
    menor_nota: f64,
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
    pub fn obtener_promedio(&self) -> f64 {
        let mut contador: u8 = 0;
        let mut total: f64 = 0.0;
        if self.calificaciones.is_empty() {
            return total;
        }
        for examen in &self.calificaciones {
            total += examen.nota;
            contador += 1;
        }
        return total / contador as f64;
    }

    pub fn obtener_calificacion_mas_alta(&self) -> f64 {
        let mut mayor_nota: f64 = -1.0;
        for examen in &self.calificaciones {
            if examen.nota >= mayor_nota {
                mayor_nota = examen.nota
            }
        }
        return mayor_nota;
    }

    pub fn obtener_calificacion_mas_baja(&self) -> f64 {
        let mut menor_nota: f64 = 11.0;
        for examen in &self.calificaciones {
            if examen.nota <= menor_nota {
                menor_nota = examen.nota
            }
        }
        return menor_nota;
    }

    pub fn generar_informe(&self) -> Option<Informe> {
        if self.obtener_promedio() > 0.0 {
            let informe_estudiantil: Informe = Informe {
                nombre_estudiante: self.nombre.clone(),
                cant_examenes_rendidos: self.calificaciones.len() as u16,
                promedio_general: self.obtener_promedio(),
                mayor_nota: self.obtener_calificacion_mas_alta(),
                menor_nota: self.obtener_calificacion_mas_baja(),
            };
            return Some(informe_estudiantil);
        }
        None
    }
}

// La estrategia a planear será implementar un struc Informe, con los datos
// pedidos, luego agregar el metodo pedido que, segun lo solicitado, deberá
// retornar un Option<Informe> y luego implementar dos casos de tes, para
// un alumno con examenes y para otro sin

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

    assert_eq!(7.0, simon.obtener_promedio());

    let simon: Estudiante = Estudiante {
        nombre: "Simon".to_string(),
        legajo: 193253,
        calificaciones: vec![],
    };

    assert_eq!(0.0, simon.obtener_promedio())
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

    assert_eq!(10.0, simon.obtener_calificacion_mas_alta());

    let simon: Estudiante = Estudiante {
        nombre: "Simon".to_string(),
        legajo: 193253,
        calificaciones: vec![],
    };

    assert_eq!(-1.0, simon.obtener_calificacion_mas_alta());
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

    assert_eq!(4.0, simon.obtener_calificacion_mas_baja());

    let simon: Estudiante = Estudiante {
        nombre: "Simon".to_string(),
        legajo: 193253,
        calificaciones: vec![],
    };

    assert_eq!(11.0, simon.obtener_calificacion_mas_baja());
}

#[test]
fn test_generar_informe_con_examenes() {
    let examen1: Examen = Examen {
        materia: "Rust 1".to_string(),
        nota: 4.0,
    };
    let examen2: Examen = Examen {
        materia: "Rust 2".to_string(),
        nota: 6.0,
    };

    let alumno: Estudiante = Estudiante {
        nombre: "Simon".to_string(),
        legajo: 1234,
        calificaciones: vec![examen1, examen2],
    };

    if let Some(informe) = alumno.generar_informe() {
        assert_eq!("Simon".to_string(), informe.nombre_estudiante);
        assert_eq!(2, informe.cant_examenes_rendidos);
        assert_eq!(5.0, informe.promedio_general);
        assert_eq!(6.0, informe.mayor_nota);
        assert_eq!(4.0, informe.menor_nota);
    } else {
        assert!(false)
    };
}

#[test]
fn test_generar_informe_sin_examenes() {
    let alumno: Estudiante = Estudiante {
        nombre: "Simon".to_string(),
        legajo: 1234,
        calificaciones: vec![],
    };
    if let Some(_informe) = alumno.generar_informe() {
        assert!(false)
    } else {
        assert!(true)
    };
}
