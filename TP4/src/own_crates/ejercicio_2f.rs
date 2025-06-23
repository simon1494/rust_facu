use std::usize;

use crate::own_crates::ejercicio_2::Persona;

#[allow(dead_code)]
pub fn edades_en_arreglo<const T: usize>(arreglo: &[Persona; T]) -> [u8; T] {
    let vec_edades: Vec<u8> = arreglo.iter().map(|u| u.edad).collect();
    let edades: [u8; T] = vec_edades.try_into().unwrap();
    return edades;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::own_crates::ejercicio_2::Persona;

    fn persona(nombre: &'static str, edad: u8) -> Persona<'static> {
        Persona {
            nombre,
            apellido: "",
            direccion: "",
            ciudad: "",
            salario: 0.0,
            edad,
        }
    }

    #[test]
    fn test_edades() {
        let arreglo = [
            persona("Ana", 5),
            persona("Luis", 15),
            persona("Sara", 25),
            persona("Juli", 35),
        ];

        let resultado = edades_en_arreglo(&arreglo);

        assert_eq!(resultado, [5, 15, 25, 35]);
    }
}
