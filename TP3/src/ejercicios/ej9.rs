use crate::ejercicios::ej3::Fecha;
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
enum TipoAnimal {
    Perro,
    Gato,
    Caballo,
    Otros,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
struct Tutor {
    nombre: String,
    direccion: String,
    telefono: String,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
struct Mascota {
    nombre: String,
    edad: u32,
    tipo: TipoAnimal,
    tutor: Tutor,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Atencion {
    mascota: Mascota,
    diagnostico: String,
    tratamiento: String,
    proxima_visita: Option<Fecha>,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Veterinaria {
    id: u32,
    nombre: String,
    direccion: String,
    cola_atencion: VecDeque<Mascota>,
    historial_atenciones: Vec<Atencion>,
}

#[allow(dead_code)]
impl Veterinaria {
    pub fn nueva(id: u32, nombre: &str, direccion: &str) -> Self {
        Veterinaria {
            id,
            nombre: nombre.to_string(),
            direccion: direccion.to_string(),
            cola_atencion: VecDeque::new(),
            historial_atenciones: Vec::new(),
        }
    }

    fn existe_en_cola(&self, nombre: &str, telefono: &str) -> bool {
        for m in &self.cola_atencion {
            if m.nombre == nombre && m.tutor.telefono == telefono {
                return true;
            }
        }
        false
    }

    pub fn agregar_mascota(&mut self, mascota: Mascota) -> bool {
        if self.existe_en_cola(&mascota.nombre, &mascota.tutor.telefono) {
            return false; // no agregar duplicado
        }
        self.cola_atencion.push_back(mascota);
        true
    }

    pub fn agregar_mascota_prioritaria(&mut self, mascota: Mascota) -> bool {
        if self.existe_en_cola(&mascota.nombre, &mascota.tutor.telefono) {
            return false;
        }
        self.cola_atencion.push_front(mascota);
        true
    }

    pub fn atender_proxima(&mut self) -> Option<Mascota> {
        self.cola_atencion.pop_front()
    }

    pub fn eliminar_mascota_de_cola(&mut self, nombre: &str, telefono: &str) -> bool {
        let mut i = 0;
        while i < self.cola_atencion.len() {
            let m = &self.cola_atencion[i];
            if m.nombre == nombre && m.tutor.telefono == telefono {
                self.cola_atencion.remove(i);
                return true;
            }
            i += 1;
        }
        false
    }

    pub fn registrar_atencion(
        &mut self,
        mascota: Mascota,
        diagnostico: &str,
        tratamiento: &str,
        proxima_visita: Option<Fecha>,
    ) {
        // Antes de registrar, eliminar de la cola si estaba pendiente
        self.eliminar_mascota_de_cola(&mascota.nombre, &mascota.tutor.telefono);

        // Registrar la atención
        let atencion = Atencion {
            mascota,
            diagnostico: diagnostico.to_string(),
            tratamiento: tratamiento.to_string(),
            proxima_visita,
        };
        self.historial_atenciones.push(atencion);
    }

    pub fn buscar_atencion(
        &self,
        nombre_mascota: &str,
        nombre_tutor: &str,
        telefono: &str,
    ) -> Option<&Atencion> {
        for a in &self.historial_atenciones {
            if a.mascota.nombre == nombre_mascota
                && a.mascota.tutor.nombre == nombre_tutor
                && a.mascota.tutor.telefono == telefono
            {
                return Some(a);
            }
        }
        None
    }

    pub fn modificar_diagnostico(
        &mut self,
        nombre_mascota: &str,
        telefono: &str,
        nuevo_diag: &str,
    ) -> bool {
        for a in &mut self.historial_atenciones {
            if a.mascota.nombre == nombre_mascota && a.mascota.tutor.telefono == telefono {
                a.diagnostico = nuevo_diag.to_string();
                return true;
            }
        }
        false
    }

    pub fn modificar_proxima_visita(
        &mut self,
        nombre_mascota: &str,
        telefono: &str,
        nueva_fecha: Fecha,
    ) -> bool {
        for a in &mut self.historial_atenciones {
            if a.mascota.nombre == nombre_mascota && a.mascota.tutor.telefono == telefono {
                a.proxima_visita = Some(nueva_fecha);
                return true;
            }
        }
        false
    }

    pub fn eliminar_atencion(&mut self, nombre_mascota: &str, telefono: &str) -> bool {
        let mut i = 0;
        while i < self.historial_atenciones.len() {
            let a = &self.historial_atenciones[i];
            if a.mascota.nombre == nombre_mascota && a.mascota.tutor.telefono == telefono {
                self.historial_atenciones.remove(i);
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
    use crate::ejercicios::ej3::Fecha;

    fn un_pelotudo() -> Tutor {
        Tutor {
            nombre: "Juan Carlos Pelotudo".to_string(),
            direccion: "rivadavia 2345".to_string(),
            telefono: "11-1234-5678".to_string(),
        }
    }

    fn nueva_mascota() -> Mascota {
        Mascota {
            nombre: "cachito".to_string(),
            edad: 5,
            tipo: TipoAnimal::Perro,
            tutor: un_pelotudo(),
        }
    }

    #[test]
    fn test_nueva_veterinaria() {
        let vet = Veterinaria::nueva(1, "veterinaria san roque", "calle 410");
        assert_eq!(vet.nombre, "veterinaria san roque");
        assert_eq!(vet.direccion, "calle 410");
        assert_eq!(vet.id, 1);
        assert_eq!(vet.cola_atencion.len(), 0);
        assert_eq!(vet.historial_atenciones.len(), 0);
    }

    #[test]
    fn test_agregar_y_atender_mascota() {
        let mut vet = Veterinaria::nueva(2, "zoonosis la plata", "calle 19");
        let m = nueva_mascota();
        vet.agregar_mascota(m.clone());
        assert_eq!(vet.cola_atencion.len(), 1);
        let atendida = vet.atender_proxima().unwrap();
        assert_eq!(atendida.nombre, "cachito");
        assert_eq!(vet.cola_atencion.len(), 0);
    }

    #[test]
    fn test_mascota_prioritaria_primero() {
        let mut vet = Veterinaria::nueva(3, "animalia", "diag 74");
        let m1 = Mascota {
            nombre: "lola".to_string(),
            edad: 3,
            tipo: TipoAnimal::Gato,
            tutor: un_pelotudo(),
        };
        let m2 = nueva_mascota();
        vet.agregar_mascota(m1.clone());
        vet.agregar_mascota_prioritaria(m2.clone());
        let atendida = vet.atender_proxima().unwrap();
        assert_eq!(atendida.nombre, "cachito");
    }

    #[test]
    fn test_eliminar_mascota_de_cola() {
        let mut vet = Veterinaria::nueva(4, "peteto", "dia 80");
        let m = nueva_mascota();
        vet.agregar_mascota(m.clone());
        let exito = vet.eliminar_mascota_de_cola("cachito", "11-1234-5678");
        assert!(exito);
        assert_eq!(vet.cola_atencion.len(), 0);
    }

    #[test]
    fn test_registrar_atencion() {
        let mut vet = Veterinaria::nueva(5, "los bichitos", "uruaguay 12");
        let m = nueva_mascota();
        let fecha_visita = Fecha::new(1, 7, 2025);
        vet.registrar_atencion(
            m.clone(),
            "dolor de panza",
            "que no coma mas cucarachas",
            Some(fecha_visita),
        );
        assert_eq!(vet.historial_atenciones.len(), 1);
    }

    #[test]
    fn test_modificar_diagnostico() {
        let mut vet = Veterinaria::nueva(6, "bichos club", "calle 30");
        let m = nueva_mascota();
        vet.registrar_atencion(m.clone(), "resfrio", "jarabe", None);
        let ok = vet.modificar_diagnostico("cachito", "11-1234-5678", "gripe");
        assert!(ok);
    }

    #[test]
    fn test_modificar_proxima_visita() {
        let mut vet = Veterinaria::nueva(7, "clinica animal", "pasaje 12");
        let m = nueva_mascota();
        vet.registrar_atencion(m.clone(), "vacuna", "aplicacion vacuna", None);
        let nueva_fecha = Fecha::new(15, 8, 2025);
        let ok = vet.modificar_proxima_visita("cachito", "11-1234-5678", nueva_fecha);
        assert!(ok);
    }

    #[test]
    fn test_eliminar_atencion() {
        let mut vet = Veterinaria::nueva(8, "mascotitas felices", "80 al fondo");
        let m = nueva_mascota();
        vet.registrar_atencion(m.clone(), "chequeo", "todo bien", None);
        let ok = vet.eliminar_atencion("cachito", "11-1234-5678");
        assert!(ok);
    }

    #[test]
    fn test_buscar_atencion() {
        let mut vet = Veterinaria::nueva(9, "el refugio", "los hornos");
        let m = nueva_mascota();
        vet.registrar_atencion(m.clone(), "infeccion", "antibiotico", None);
        let atencion = vet.buscar_atencion("cachito", "Juan Carlos Pelotudo", "11-1234-5678");
        assert!(atencion.is_some());
        let a = atencion.unwrap();
        assert_eq!(a.diagnostico, "infeccion");
    }

    #[test]
    fn test_no_permite_duplicados_en_cola() {
        let mut vet = Veterinaria::nueva(10, "duplicados", "dir 123");
        let m = nueva_mascota();
        let agregado1 = vet.agregar_mascota(m.clone());
        let agregado2 = vet.agregar_mascota(m.clone());
        assert!(agregado1);
        assert!(!agregado2);
        assert_eq!(vet.cola_atencion.len(), 1);
    }

    #[test]
    fn test_registrar_atencion_elimina_de_cola() {
        let mut vet = Veterinaria::nueva(11, "san martin", "dir");
        let m = nueva_mascota();
        vet.agregar_mascota(m.clone());
        vet.registrar_atencion(m.clone(), "diagnostico", "tratamiento", None);
        assert!(vet.cola_atencion.is_empty());
        assert_eq!(vet.historial_atenciones.len(), 1);
    }
}
