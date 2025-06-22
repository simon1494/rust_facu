use crate::own_crates::ejercicio_2::Persona;

#[allow(dead_code)]
pub fn al_menos_una_persona_vive_en_ciudad(vector: Vec<Persona>, ciudad: String) -> bool {
    vector.iter().any(|x| x.vive_en_ciudad(&ciudad))
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
        let resultado = al_menos_una_persona_vive_en_ciudad(personas, "La Plata".to_string());
        assert!(!resultado);
    }

    #[test]
    fn test_ninguna_en_ciudad() {
        let personas = vec![persona("Ana", "Rosario"), persona("Luis", "Córdoba")];
        let resultado = al_menos_una_persona_vive_en_ciudad(personas, "La Plata".to_string());
        assert!(!resultado);
    }

    #[test]
    fn test_una_en_ciudad() {
        let personas = vec![persona("Ana", "La Plata"), persona("Luis", "Córdoba")];
        let resultado = al_menos_una_persona_vive_en_ciudad(personas, "La Plata".to_string());
        assert!(resultado);
    }

    #[test]
    fn test_varias_en_ciudad() {
        let personas = vec![persona("Ana", "La Plata"), persona("Luis", "La Plata")];
        let resultado = al_menos_una_persona_vive_en_ciudad(personas, "La Plata".to_string());
        assert!(resultado);
    }
}
