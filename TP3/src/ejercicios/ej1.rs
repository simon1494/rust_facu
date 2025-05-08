#[derive(Debug)]
pub struct Persona {
    nombre: String,
    edad: i32,
    direccion: String,
}

#[allow(dead_code)]
impl Persona {
    fn new(nombre: String, edad: i32, direccion: String) -> Persona {
        Persona {
            nombre,
            edad,
            direccion,
        }
    }

    fn get_edad(&self) -> i32 {
        return self.edad;
    }

    fn set_direccion(&mut self, direccion: String) {
        self.direccion = direccion;
    }

    fn to_string(&self) -> String {
        format!(
            "Buenas, soy {}, tengo {} y vivo en {}",
            self.nombre.to_string(),
            self.edad,
            self.direccion.to_string(),
        )
    }
}

#[test]
fn test_persona_constructor() {
    let nombre: String = String::from("Simon");
    let edad: i32 = 31;
    let direccion: String = String::from("2 894");

    let nueva: Persona = Persona::new(nombre.clone(), edad.clone(), direccion.clone());

    assert_eq!(nombre, nueva.nombre);
    assert_eq!(edad, nueva.edad);
    assert_eq!(direccion, nueva.direccion);
}

#[test]
fn test_persona_to_string() {
    let nombre: String = String::from("Simon");
    let edad: i32 = 31;
    let direccion: String = String::from("2 894");

    let nueva: Persona = Persona::new(nombre.clone(), edad.clone(), direccion.clone());

    let esperado: String = String::from("Buenas, soy Simon, tengo 31 y vivo en 2 894");

    assert_eq!(esperado, nueva.to_string());
}

#[test]
fn test_set_direccion() {
    let nombre: String = String::from("Simon");
    let edad: i32 = 31;
    let direccion: String = String::from("2 894");
    let mut nueva: Persona = Persona::new(nombre.clone(), edad.clone(), direccion.clone());

    nueva.set_direccion("78 bis 928".to_string());
    assert_eq!("78 bis 928".to_string(), nueva.direccion);
}

#[test]
fn test_get_edad() {
    let nombre: String = String::from("Simon");
    let edad: i32 = 31;
    let direccion: String = String::from("2 894");
    let nueva: Persona = Persona::new(nombre.clone(), edad.clone(), direccion.clone());

    assert_eq!(31, nueva.get_edad());
}
