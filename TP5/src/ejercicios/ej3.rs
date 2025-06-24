use crate::ejercicios::fecha::Fecha;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufReader, Write};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(dead_code)]
enum TipoAnimal {
    Perro,
    Gato,
    Caballo,
    Otros,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(dead_code)]
struct Tutor {
    nombre: String,
    direccion: String,
    telefono: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(dead_code)]
struct Mascota {
    nombre: String,
    edad: u32,
    tipo: TipoAnimal,
    tutor: Tutor,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    ruta_archivo: String,
}

#[allow(dead_code)]
impl Veterinaria {
    pub fn new(id: u32, nombre: &str, direccion: &str, ruta_archivo: &str) -> Self {
        let mut vet = Veterinaria {
            id,
            nombre: nombre.to_string(),
            direccion: direccion.to_string(),
            cola_atencion: VecDeque::new(),
            historial_atenciones: vec![],
            ruta_archivo: ruta_archivo.to_string(),
        };
        vet.historial_atenciones = vet.leer_atenciones_de_archivo();
        vet
    }

    pub fn agregar_mascota(&mut self, mascota: Mascota) {
        self.cola_atencion.push_back(mascota);
    }

    pub fn agregar_mascota_prioritaria(&mut self, mascota: Mascota) {
        self.cola_atencion.push_front(mascota);
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
        let atencion = Atencion {
            mascota,
            diagnostico: diagnostico.to_string(),
            tratamiento: tratamiento.to_string(),
            proxima_visita,
        };
        self.historial_atenciones.push(atencion);
        self.escribir_atenciones_en_archivo();
    }

    pub fn buscar_atencion(
        &self,
        nombre_mascota: &str,
        nombre_tutor: &str,
        telefono: &str,
    ) -> Option<&Atencion> {
        self.historial_atenciones.iter().find(|a| {
            a.mascota.nombre == nombre_mascota
                && a.mascota.tutor.nombre == nombre_tutor
                && a.mascota.tutor.telefono == telefono
        })
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
                self.escribir_atenciones_en_archivo();
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
                self.escribir_atenciones_en_archivo();
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
                self.escribir_atenciones_en_archivo();
                return true;
            }
            i += 1;
        }
        false
    }

    fn leer_atenciones_de_archivo(&self) -> Vec<Atencion> {
        if let Ok(file) = File::open(&self.ruta_archivo) {
            let reader = BufReader::new(file);
            serde_json::from_reader(reader).unwrap_or_default()
        } else {
            vec![]
        }
    }

    fn escribir_atenciones_en_archivo(&self) {
        if let Ok(mut file) = File::create(&self.ruta_archivo) {
            let _ = file.write_all(
                serde_json::to_string_pretty(&self.historial_atenciones)
                    .unwrap()
                    .as_bytes(),
            );
        }
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
            telefono: "123456789".to_string(),
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
        let ruta = "probatoria.json";
        let vet = Veterinaria::new(1, "veterinaria san roque", "calle 410", ruta);
        assert_eq!(vet.nombre, "veterinaria san roque");
        assert_eq!(vet.direccion, "calle 410");
        assert_eq!(vet.id, 1);
        assert_eq!(vet.cola_atencion.len(), 0);
        assert_eq!(vet.historial_atenciones.len(), 0);
    }

    #[test]
    fn test_agregar_y_atender_mascota() {
        let ruta = "probatoria2.json";
        let mut vet = Veterinaria::new(2, "zoonosis la plata", "calle 19", ruta);
        let m = nueva_mascota();
        vet.agregar_mascota(m.clone());
        assert_eq!(vet.cola_atencion.len(), 1);
        let atendida = vet.atender_proxima().unwrap();
        assert_eq!(atendida.nombre, "cachito");
        assert_eq!(vet.cola_atencion.len(), 0);
    }

    #[test]
    fn test_mascota_prioritaria_primero() {
        let ruta = "probatoria3.json";
        let mut vet = Veterinaria::new(3, "animalia", "diag 74", ruta);
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
        let ruta = "probatoria4.json";
        let mut vet = Veterinaria::new(4, "peteto", "dia 80", ruta);
        let m = nueva_mascota();
        vet.agregar_mascota(m.clone());
        let exito = vet.eliminar_mascota_de_cola("cachito", "123456789");
        assert!(exito);
        assert_eq!(vet.cola_atencion.len(), 0);
    }

    #[test]
    fn test_registrar_atencion() {
        let ruta = "probatoria5.json";
        let mut vet = Veterinaria::new(5, "los bichitos", "uruaguay 12", ruta);
        let m = nueva_mascota();
        let fecha_visita = Fecha::new(1, 7, 2025);
        vet.registrar_atencion(
            m.clone(),
            "dolor de panza",
            "que no coma mas cucarachas",
            Some(fecha_visita),
        );
        assert_eq!(vet.historial_atenciones.len(), 1);
        let _ = std::fs::remove_file(ruta);
    }

    #[test]
    fn test_modificar_diagnostico() {
        let ruta = "probatoria6.json";
        let mut vet = Veterinaria::new(6, "bichos club", "calle 30", ruta);
        let m = nueva_mascota();
        vet.registrar_atencion(m.clone(), "resfrio", "jarabe", None);
        let ok = vet.modificar_diagnostico("cachito", "123456789", "gripe");
        assert!(ok);
        let _ = std::fs::remove_file(ruta);
    }

    #[test]
    fn test_modificar_proxima_visita() {
        let ruta = "probatoria7.json";
        let mut vet = Veterinaria::new(7, "clinica animal", "pasaje 12", ruta);
        let m = nueva_mascota();
        vet.registrar_atencion(m.clone(), "vacuna", "aplicacion vacuna", None);
        let nueva_fecha = Fecha::new(15, 8, 2025);
        let ok = vet.modificar_proxima_visita("cachito", "123456789", nueva_fecha);
        assert!(ok);
        let _ = std::fs::remove_file(ruta);
    }

    #[test]
    fn test_eliminar_atencion() {
        let ruta = "probatoria9.json";
        let mut vet = Veterinaria::new(8, "mascotitas felices", "80 al fondo", ruta);
        let m = nueva_mascota();
        vet.registrar_atencion(m.clone(), "chequeo", "todo bien", None);
        let ok = vet.eliminar_atencion("cachito", "123456789");
        assert!(ok);
        let _ = std::fs::remove_file(ruta);
    }

    #[test]
    fn test_buscar_atencion() {
        let ruta = "probatoria9.json";
        let mut vet = Veterinaria::new(9, "el refugio", "los hornos", ruta);
        let m = nueva_mascota();
        vet.registrar_atencion(m.clone(), "infeccion", "antibiotico", None);
        let atencion = vet.buscar_atencion("cachito", "Juan Carlos Pelotudo", "123456789");
        assert!(atencion.is_some());
        let a = atencion.unwrap();
        assert_eq!(a.diagnostico, "infeccion");
        let _ = std::fs::remove_file(ruta);
    }
}
