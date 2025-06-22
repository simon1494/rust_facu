use crate::own_crates::ejercicio_2::Persona;

#[allow(dead_code)]
pub fn get_personas_con_edad_y_en_ciudad(
    vector: Vec<Persona>,
    edad: u8,
    ciudad: String,
) -> Option<Vec<Persona>> {
    let reducido: Vec<Persona> = vector
        .iter()
        .filter(|x| x.edad_mayor_que(edad) && x.vive_en_ciudad(&ciudad))
        .cloned()
        .collect();
    if reducido.is_empty() {
        return None;
    }
    return Some(reducido);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::own_crates::ejercicio_2::Persona;

    #[test]
    fn test_lista_vacia() {
        let personas: Vec<Persona> = vec![];

        let resultado = get_personas_con_edad_y_en_ciudad(personas, 30, "La Plata".to_string())
            .unwrap_or(vec![]);

        assert!(resultado.is_empty());
    }

    #[test]
    fn test_cumplen_edad_no_ciudad() {
        let persona = Persona {
            nombre: "Luis",
            apellido: "Gomez",
            direccion: "Calle Falsa 123",
            ciudad: "Rosario",
            salario: 1000.0,
            edad: 40,
        };

        let personas: Vec<Persona> = vec![persona];

        let resultado = get_personas_con_edad_y_en_ciudad(personas, 30, "La Plata".to_string())
            .unwrap_or(vec![]);

        assert!(resultado.is_empty());
    }

    #[test]
    fn test_cumplen_ciudad_no_edad() {
        let persona = Persona {
            nombre: "Sofía",
            apellido: "López",
            direccion: "Calle Real 456",
            ciudad: "La Plata",
            salario: 1200.0,
            edad: 25,
        };

        let personas: Vec<Persona> = vec![persona];

        let resultado = get_personas_con_edad_y_en_ciudad(personas, 30, "La Plata".to_string())
            .unwrap_or(vec![]);

        assert!(resultado.is_empty());
    }

    #[test]
    fn test_una_persona_cumple() {
        let persona = Persona {
            nombre: "Marta",
            apellido: "Fernandez",
            direccion: "Av. Siempre Viva",
            ciudad: "La Plata",
            salario: 2000.0,
            edad: 45,
        };

        let personas: Vec<Persona> = vec![persona];

        let resultado =
            get_personas_con_edad_y_en_ciudad(personas.clone(), 30, "La Plata".to_string())
                .unwrap();

        assert_eq!(resultado[0], persona);
    }

    #[test]
    fn test_varias_personas_cumplen() {
        let personas: Vec<Persona> = vec![
            Persona {
                nombre: "Luis",
                apellido: "Martinez",
                direccion: "Calle 1",
                ciudad: "La Plata",
                salario: 1500.0,
                edad: 35,
            },
            Persona {
                nombre: "Ana",
                apellido: "Torres",
                direccion: "Calle 2",
                ciudad: "La Plata",
                salario: 1600.0,
                edad: 45,
            },
            Persona {
                nombre: "Pedro",
                apellido: "Juarez",
                direccion: "Calle 3",
                ciudad: "Rosario",
                salario: 1700.0,
                edad: 50,
            },
        ];

        let resultado =
            get_personas_con_edad_y_en_ciudad(personas.clone(), 30, "La Plata".to_string())
                .unwrap();

        assert_eq!(resultado[0], personas[0]);
        assert_eq!(resultado[1], personas[1]);
    }
}
