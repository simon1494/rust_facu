use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Usuario<'a> {
    id: u128,
    nombre: &'a str,
    apellido: &'a str,
}

#[allow(dead_code)]
impl<'a> Usuario<'a> {
    pub fn new(id: u128, nombre: &'a str, apellido: &'a str) -> Self {
        Usuario {
            id,
            nombre,
            apellido,
        }
    }
}
