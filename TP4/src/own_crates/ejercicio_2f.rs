use crate::own_crates::ejercicio_2::Persona;

#[allow(dead_code)]
pub fn edades_en_arreglo(arreglo: [Persona; 4], _objetivo: Persona) -> [u8; 4] {
    let mut edades = [0u8; 4];

    for (i, persona) in arreglo.iter().enumerate() {
        edades[i] = persona.edad;
    }

    edades
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

        let objetivo = persona("Luis", 15);

        let resultado = edades_en_arreglo(arreglo, objetivo);

        assert_eq!(resultado, [5, 15, 25, 35]);
    }
}
