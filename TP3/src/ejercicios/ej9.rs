use crate::ejercicios::ej3::Fecha;
use std::collections::VecDeque;

#[allow(dead_code)]
pub enum Animal {
    PERRO,
    GATO,
    CABALLO,
    OTROS,
}

#[allow(dead_code)]
pub struct Tutor {
    nombre: String,
    direccion: String,
    telefono: u32,
}

#[allow(dead_code)]
pub struct Mascota {
    nombre: String,
    tipo_animal: Animal,
    edad: u8,
    tutor: Tutor,
}

#[allow(dead_code)]
pub struct Atencion {
    mascota: Mascota,
    diagnostico: String,
    tratamiento: String,
    proxima_visita: Fecha,
}
#[allow(dead_code)]
pub struct Veterinaria {
    id: u32,
    nombre: String,
    direccion: String,
    sala_de_espera: VecDeque<Mascota>,
    atenciones_realizadas: VecDeque<Atencion>,
}

#[allow(dead_code)]
impl Veterinaria {
    pub fn new(
        id: u32,
        nombre: String,
        direccion: String,
        sala_de_espera: VecDeque<Mascota>,
        atenciones_realizadas: VecDeque<Atencion>,
    ) -> Veterinaria {
        Veterinaria {
            id,
            nombre,
            direccion,
            sala_de_espera,
            atenciones_realizadas,
        }
    }

    fn poner_en_espera(&self, mascota: Mascota) {}

    fn poner_en_espera_con_prioridad(&self, mascota: Mascota) {}

    fn quitar_de_espera(&self, nombre_mascota: String, nombre_tutor: String, telefono_tutor: u32) {}

    fn atender_mascota(&self) {}

    fn registrar_atencion() {}

    fn eliminar_atencion() {}

    fn buscar_atencion(
        &self,
        nombre_mascota: String,
        nombre_tutor: String,
        telefono_tutor: u32,
    ) -> String {
        return "una atencion".to_string();
    }

    fn modificar_diagnostico(
        &self,
        nombre_mascota: String,
        nombre_tutor: String,
        telefono_tutor: u32,
    ) -> bool {
        return true;
    }

    fn modificar_proximo_turno(
        &self,
        nombre_mascota: String,
        nombre_tutor: String,
        telefono_tutor: u32,
    ) -> bool {
        return true;
    }
}
