use crate::own_crates::ejercicio_2::Persona;

#[allow(dead_code)]
pub fn get_todas_las_personas_viven_en_ciudad(vector: Vec<Persona>, ciudad: String) -> bool {
    vector.iter().all(|x| x.vive_en_ciudad(&ciudad))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::own_crates::ejercicio_2::Persona;

    fn persona(nombre: &'static str, ciudad: &'static str) -> Persona<'static> {
        Persona {
            nombre,
            apellido: "",
            direccion: "",
            ciudad,
            salario: 0.0,
            edad: 0,
        }
    }

    #[test]
    fn test_lista_vacia() {
        let personas: Vec<Persona> = vec![];
        let resultado = get_todas_las_personas_viven_en_ciudad(personas, "La Plata".to_string());
        assert!(resultado);
    }

    #[test]
    fn test_todas_en_ciudad() {
        let personas = vec![persona("Ana", "La Plata"), persona("Luis", "La Plata")];
        let resultado = get_todas_las_personas_viven_en_ciudad(personas, "La Plata".to_string());
        assert!(resultado);
    }

    #[test]
    fn test_algunas_en_ciudad() {
        let personas = vec![persona("Ana", "La Plata"), persona("Luis", "Rosario")];
        let resultado = get_todas_las_personas_viven_en_ciudad(personas, "La Plata".to_string());
        assert!(!resultado);
    }

    #[test]
    fn test_ninguna_en_ciudad() {
        let personas = vec![persona("Ana", "Rosario"), persona("Luis", "Córdoba")];
        let resultado = get_todas_las_personas_viven_en_ciudad(personas, "La Plata".to_string());
        assert!(!resultado);
    }
}
