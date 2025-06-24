use crate::ejercicios::prueba::Usuario;
use serde_json;
use std::fs::File;
use std::io::{Read, Write};
mod ejercicios;

fn main() {
    let usuario1 = Usuario::new(1, "Simon", "Bierozko");
    let usuario2 = Usuario::new(1, "Carlo", "Rodriguez");
    let usuario3 = Usuario::new(1, "Analia", "Peterson");

    let vector: Vec<Usuario> = vec![usuario1, usuario2, usuario3];

    let vector_s = serde_json::to_string_pretty(&vector).unwrap();
    let mut f = File::create("src/usuarios.json").unwrap();
    let _ = f.write_all(vector_s.as_bytes());
}
