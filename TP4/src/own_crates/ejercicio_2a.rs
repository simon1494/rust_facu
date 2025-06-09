#[derive(Clone, Copy)]
#[allow(dead_code)]
pub struct Persona<'a> {
    nombre: &'a str,
    apellido: &'a str,
    direccion: &'a str,
    ciudad: &'a str,
    salario: f64,
    edad: u8,
}

impl<'a> Persona<'a> {
    fn salario_mayor(&self, referencia: f64) -> bool {
        return self.salario > referencia;
    }
}

#[allow(dead_code)]
pub fn get_personas_con_salarios_superiores(
    vector: Vec<Persona>,
    referencia: f64,
) -> Option<Vec<String>> {
    let mut listado: Vec<String> = vec![];
    let reducido: Vec<Persona> = vector
        .iter()
        .filter(|x| x.salario_mayor(referencia))
        .cloned()
        .collect();
    if reducido.is_empty() {
        return None;
    }
    for persona in reducido {
        listado.push(persona.nombre.to_string())
    }
    return Some(listado);
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn test_salario_es_menor() {
        let persona: Persona<'_> = Persona {
            nombre: "Simon",
            apellido: "Bierozko",
            direccion: "asdf",
            ciudad: "adf",
            salario: 100.0,
            edad: 31,
        };

        assert_eq!(false, persona.salario_mayor(200.0));
    }

    #[test]
    fn test_salario_es_mayor() {
        let persona: Persona<'_> = Persona {
            nombre: "Simon",
            apellido: "Bierozko",
            direccion: "asdf",
            ciudad: "adf",
            salario: 300.0,
            edad: 31,
        };

        assert_eq!(true, persona.salario_mayor(200.0));
    }

    #[test]
    fn test_salario_es_igual() {
        let persona: Persona<'_> = Persona {
            nombre: "Simon",
            apellido: "Bierozko",
            direccion: "asdf",
            ciudad: "adf",
            salario: 200.0,
            edad: 31,
        };

        assert_eq!(false, persona.salario_mayor(200.0));
    }

    #[test]
    fn test_vector_vacio() {
        let personas: Vec<Persona> = vec![];

        let listado: Vec<String> =
            get_personas_con_salarios_superiores(personas, 0.0).unwrap_or(Vec::new());

        assert!(listado.is_empty());
    }

    #[test]
    fn test_ningun_salario() {
        let persona = Persona {
            nombre: "Simon",
            apellido: "Bierozko",
            direccion: "asdf",
            ciudad: "adf",
            salario: 200.0,
            edad: 31,
        };

        let personas: Vec<Persona> = vec![persona];

        let listado: Vec<String> =
            get_personas_con_salarios_superiores(personas, 300.0).unwrap_or(Vec::new());

        assert!(listado.is_empty());
    }

    #[test]
    fn test_un_salario() {
        let persona = Persona {
            nombre: "Simon",
            apellido: "Bierozko",
            direccion: "asdf",
            ciudad: "adf",
            salario: 200.0,
            edad: 31,
        };

        let personas: Vec<Persona> = vec![persona];

        let listado: Vec<String> =
            get_personas_con_salarios_superiores(personas, 100.0).unwrap_or(Vec::new());

        let esperado = vec!["Simon"];

        assert_eq!(esperado, listado);
    }

    #[test]
    fn test_varios_salario() {
        let persona1 = Persona {
            nombre: "Simon",
            apellido: "Bierozko",
            direccion: "asdf",
            ciudad: "adf",
            salario: 200.0,
            edad: 31,
        };

        let persona2 = Persona {
            nombre: "Carlos",
            apellido: "Panqueque",
            direccion: "asdf",
            ciudad: "adf",
            salario: 300.0,
            edad: 31,
        };

        let persona3 = Persona {
            nombre: "Pedro",
            apellido: "Panqueque",
            direccion: "asdf",
            ciudad: "adf",
            salario: 100.0,
            edad: 31,
        };

        let personas: Vec<Persona> = vec![persona1, persona2, persona3];

        let listado: Vec<String> =
            get_personas_con_salarios_superiores(personas, 150.0).unwrap_or(Vec::new());

        let esperado = vec!["Simon", "Carlos"];

        assert_eq!(esperado, listado);
    }
}
