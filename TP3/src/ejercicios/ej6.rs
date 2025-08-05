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

#[derive(Debug)]
#[allow(dead_code)]
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

    fn obtener_promedio(&self) -> Option<f64> {
        if self.calificaciones.is_empty() {
            return None;
        }
        let mut total: f64 = 0.0;
        let mut contador: usize = 0;
        for examen in &self.calificaciones {
            total += examen.nota;
            contador += 1;
        }
        Some(total / contador as f64)
    }

    fn obtener_calificacion_mas_alta(&self) -> Option<f64> {
        if self.calificaciones.is_empty() {
            return None;
        }
        let mut mayor = self.calificaciones[0].nota;
        for examen in &self.calificaciones {
            if examen.nota > mayor {
                mayor = examen.nota;
            }
        }
        Some(mayor)
    }

    fn obtener_calificacion_mas_baja(&self) -> Option<f64> {
        if self.calificaciones.is_empty() {
            return None;
        }
        let mut menor = self.calificaciones[0].nota;
        for examen in &self.calificaciones {
            if examen.nota < menor {
                menor = examen.nota;
            }
        }
        Some(menor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn crear_examen(materia: &str, nota: f64) -> Examen {
        Examen::new(materia.to_string(), nota)
    }

    #[test]
    fn test_promedio_varios_examenes() {
        let examenes = vec![
            crear_examen("CADP", 7.0),
            crear_examen("Taller", 5.0),
            crear_examen("OC", 9.0),
        ];
        let estudiante = Estudiante::new("Simon".to_string(), 193253, examenes);
        assert_eq!(estudiante.obtener_promedio(), Some(7.0));
    }

    #[test]
    fn test_promedio_un_solo_examen() {
        let examenes = vec![crear_examen("CADP", 10.0)];
        let estudiante = Estudiante::new("Simon".to_string(), 193253, examenes);
        assert_eq!(estudiante.obtener_promedio(), Some(10.0));
    }

    #[test]
    fn test_promedio_sin_examenes() {
        let estudiante = Estudiante::new("Simon".to_string(), 193253, vec![]);
        assert_eq!(estudiante.obtener_promedio(), None);
    }

    #[test]
    fn test_nota_mas_alta_varios_examenes() {
        let examenes = vec![
            crear_examen("CADP", 7.0),
            crear_examen("Taller", 10.0),
            crear_examen("OC", 5.0),
        ];
        let estudiante = Estudiante::new("Simon".to_string(), 193253, examenes);
        assert_eq!(estudiante.obtener_calificacion_mas_alta(), Some(10.0));
    }

    #[test]
    fn test_nota_mas_alta_notas_iguales() {
        let examenes = vec![crear_examen("CADP", 8.0), crear_examen("OC", 8.0)];
        let estudiante = Estudiante::new("Simon".to_string(), 193253, examenes);
        assert_eq!(estudiante.obtener_calificacion_mas_alta(), Some(8.0));
    }

    #[test]
    fn test_nota_mas_alta_un_solo_examen() {
        let examenes = vec![crear_examen("Taller", 6.0)];
        let estudiante = Estudiante::new("Simon".to_string(), 193253, examenes);
        assert_eq!(estudiante.obtener_calificacion_mas_alta(), Some(6.0));
    }

    #[test]
    fn test_nota_mas_alta_sin_examenes() {
        let estudiante = Estudiante::new("Simon".to_string(), 193253, vec![]);
        assert_eq!(estudiante.obtener_calificacion_mas_alta(), None);
    }

    #[test]
    fn test_nota_mas_baja_varios_examenes() {
        let examenes = vec![
            crear_examen("CADP", 3.0),
            crear_examen("Taller", 5.0),
            crear_examen("OC", 9.0),
        ];
        let estudiante = Estudiante::new("Simon".to_string(), 193253, examenes);
        assert_eq!(estudiante.obtener_calificacion_mas_baja(), Some(3.0));
    }

    #[test]
    fn test_nota_mas_baja_notas_iguales() {
        let examenes = vec![crear_examen("CADP", 7.0), crear_examen("OC", 7.0)];
        let estudiante = Estudiante::new("Simon".to_string(), 193253, examenes);
        assert_eq!(estudiante.obtener_calificacion_mas_baja(), Some(7.0));
    }

    #[test]
    fn test_nota_mas_baja_un_solo_examen() {
        let examenes = vec![crear_examen("Taller", 4.0)];
        let estudiante = Estudiante::new("Simon".to_string(), 193253, examenes);
        assert_eq!(estudiante.obtener_calificacion_mas_baja(), Some(4.0));
    }

    #[test]
    fn test_nota_mas_baja_sin_examenes() {
        let estudiante = Estudiante::new("Simon".to_string(), 193253, vec![]);
        assert_eq!(estudiante.obtener_calificacion_mas_baja(), None);
    }
}
