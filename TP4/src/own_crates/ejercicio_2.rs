#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(dead_code)]
pub struct Persona<'a> {
    pub nombre: &'a str,
    pub apellido: &'a str,
    pub direccion: &'a str,
    pub ciudad: &'a str,
    pub salario: f64,
    pub edad: u8,
}

#[allow(dead_code)]
impl<'a> Persona<'a> {
    pub fn salario_mayor(&self, referencia: f64) -> bool {
        return self.salario > referencia;
    }

    pub fn vive_en_ciudad(&self, ciudad: &str) -> bool {
        return self.ciudad.to_string() == ciudad;
    }

    pub fn edad_mayor_que(&self, edad: u8) -> bool {
        return self.edad > edad;
    }
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
    fn test_vive_en_ciudad() {
        let persona: Persona<'_> = Persona {
            nombre: "Simon",
            apellido: "Bierozko",
            direccion: "asdf",
            ciudad: "La Plata",
            salario: 200.0,
            edad: 31,
        };

        assert_eq!(true, persona.vive_en_ciudad("La Plata"));
    }

    #[test]
    fn test_no_vive_en_ciudad() {
        let persona: Persona<'_> = Persona {
            nombre: "Simon",
            apellido: "Bierozko",
            direccion: "asdf",
            ciudad: "Berisso",
            salario: 200.0,
            edad: 31,
        };

        assert_eq!(false, persona.vive_en_ciudad("La Plata"));
    }

    #[test]
    fn test_edad31_mayor_que_edad40() {
        let persona: Persona<'_> = Persona {
            nombre: "Simon",
            apellido: "Bierozko",
            direccion: "asdf",
            ciudad: "Berisso",
            salario: 200.0,
            edad: 31,
        };

        assert_eq!(false, persona.edad_mayor_que(40));
    }

    #[test]
    fn test_edad31_mayor_que_edad20() {
        let persona: Persona<'_> = Persona {
            nombre: "Simon",
            apellido: "Bierozko",
            direccion: "asdf",
            ciudad: "Berisso",
            salario: 200.0,
            edad: 31,
        };

        assert_eq!(true, persona.edad_mayor_que(20));
    }

    #[test]
    fn test_edad31_mayor_que_edad31() {
        let persona: Persona<'_> = Persona {
            nombre: "Simon",
            apellido: "Bierozko",
            direccion: "asdf",
            ciudad: "Berisso",
            salario: 200.0,
            edad: 31,
        };

        assert_eq!(false, persona.edad_mayor_que(31));
    }
}
