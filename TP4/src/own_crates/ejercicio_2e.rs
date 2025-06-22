use crate::own_crates::ejercicio_2::Persona;

#[allow(dead_code)]
pub fn persona_existe_en_arreglo(arreglo: &[Persona], objetivo: Persona) -> bool {
    arreglo.iter().any(|x| x == &objetivo)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::own_crates::ejercicio_2::Persona;

    #[test]
    fn test_persona_existe_en_arreglo() {
        let p1 = Persona {
            nombre: "Simón",
            apellido: "Bierozko",
            direccion: "78 bis",
            ciudad: "La Plata",
            salario: 300.0,
            edad: 31,
        };

        let p2 = Persona {
            nombre: "Ana",
            apellido: "López",
            direccion: "Otra calle",
            ciudad: "Rosario",
            salario: 500.0,
            edad: 28,
        };

        let personas = vec![p1, p2];

        let objetivo = Persona {
            nombre: "Ana",
            apellido: "López",
            direccion: "Otra calle",
            ciudad: "Rosario",
            salario: 500.0,
            edad: 28,
        };

        assert!(persona_existe_en_arreglo(&personas, objetivo));
    }

    #[test]
    fn test_persona_no_existe_en_arreglo() {
        let personas = vec![Persona {
            nombre: "Juan",
            apellido: "Pérez",
            direccion: "123 calle",
            ciudad: "Córdoba",
            salario: 1000.0,
            edad: 45,
        }];

        let objetivo = Persona {
            nombre: "María",
            apellido: "Gómez",
            direccion: "78 bis",
            ciudad: "Buenos Aires",
            salario: 1200.0,
            edad: 38,
        };

        assert!(!persona_existe_en_arreglo(&personas, objetivo));
    }

    #[test]
    fn test_arreglo_vacio() {
        let personas: Vec<Persona> = vec![];

        let objetivo = Persona {
            nombre: "Simón",
            apellido: "Bierozko",
            direccion: "Calle Falsa 123",
            ciudad: "La Plata",
            salario: 300.0,
            edad: 31,
        };

        assert!(!persona_existe_en_arreglo(&personas, objetivo));
    }
}
