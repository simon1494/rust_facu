use crate::own_crates::ejercicio_2::Persona;
use ordered_float::OrderedFloat;

#[allow(dead_code)]
pub fn get_personas_con_mayor_y_menor_salario(arreglo: [Persona; 4]) -> [Persona; 2] {
    let mut retorno: [Persona<'_>; 2] = [arreglo[0], arreglo[0]];

    let mayor = arreglo
        .iter()
        .max_by(|a, b| OrderedFloat(a.salario).cmp(&OrderedFloat(b.salario)))
        .cloned();
    let menor = arreglo
        .iter()
        .min_by(|a, b| OrderedFloat(a.salario).cmp(&OrderedFloat(b.salario)))
        .cloned();

    let mut vector_mayor = arreglo
        .iter()
        .filter(|x| x.salario == mayor.unwrap().salario)
        .cloned()
        .collect::<Vec<_>>();
    let mut vector_menor = arreglo
        .iter()
        .filter(|x| x.salario == menor.unwrap().salario)
        .cloned()
        .collect::<Vec<_>>();

    if vector_mayor.len() > 1 || vector_menor.len() > 1 {
        if vector_mayor.len() > 1 && !vector_menor.len() > 1 {
            retorno[0] = vector_menor.pop().unwrap();
            retorno[1] = vector_mayor
                .iter()
                .max_by(|x, y| x.edad.cmp(&y.edad))
                .cloned()
                .unwrap();
            return retorno;
        } else if !vector_mayor.len() > 1 && vector_menor.len() > 1 {
            retorno[1] = vector_mayor.pop().unwrap();
            retorno[0] = vector_menor
                .iter()
                .max_by(|x, y| x.edad.cmp(&y.edad))
                .cloned()
                .unwrap();
            return retorno;
        } else {
            retorno[0] = vector_menor
                .iter()
                .max_by(|x, y| x.edad.cmp(&y.edad))
                .cloned()
                .unwrap();
            retorno[1] = vector_mayor
                .iter()
                .max_by(|x, y| x.edad.cmp(&y.edad))
                .cloned()
                .unwrap();
            return retorno;
        }
    }
    retorno[0] = vector_menor.pop().unwrap();
    retorno[1] = vector_mayor.pop().unwrap();

    return retorno;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::own_crates::ejercicio_2::Persona;

    fn persona(nombre: &'static str, edad: u8, salario: f64) -> Persona<'static> {
        Persona {
            nombre,
            apellido: "",
            direccion: "",
            ciudad: "",
            salario,
            edad,
        }
    }

    #[test]
    fn test_sin_empates() {
        let personas = [
            persona("A", 25, 1000.0),
            persona("B", 35, 2000.0),
            persona("C", 45, 3000.0),
            persona("D", 55, 4000.0),
        ];

        let resultado = get_personas_con_mayor_y_menor_salario(personas);
        assert_eq!(resultado[0].nombre, "A"); // menor salario
        assert_eq!(resultado[1].nombre, "D"); // mayor salario
    }

    #[test]
    fn test_empate_en_mayor_salario() {
        let personas = [
            persona("A", 25, 1000.0),
            persona("B", 35, 4000.0),
            persona("C", 55, 4000.0), // empate en mayor, gana por edad
            persona("D", 45, 1500.0),
        ];

        let resultado = get_personas_con_mayor_y_menor_salario(personas);
        assert_eq!(resultado[0].nombre, "A"); // menor salario
        assert_eq!(resultado[1].nombre, "C"); // mayor salario por edad
    }

    #[test]
    fn test_empate_en_menor_salario() {
        let personas = [
            persona("A", 25, 1000.0),
            persona("B", 60, 1000.0), // empate en menor, gana por edad
            persona("C", 45, 2000.0),
            persona("D", 55, 3000.0),
        ];

        let resultado = get_personas_con_mayor_y_menor_salario(personas);
        assert_eq!(resultado[0].nombre, "B"); // menor salario por edad
        assert_eq!(resultado[1].nombre, "D"); // mayor salario
    }

    #[test]
    fn test_empate_en_ambos_extremos() {
        let personas = [
            persona("A", 25, 1000.0),
            persona("B", 55, 1000.0), // menor empate
            persona("C", 45, 4000.0),
            persona("D", 65, 4000.0), // mayor empate
        ];

        let resultado = get_personas_con_mayor_y_menor_salario(personas);
        assert_eq!(resultado[0].nombre, "B"); // menor salario por edad
        assert_eq!(resultado[1].nombre, "D"); // mayor salario por edad
    }

    #[test]
    fn test_todos_igual_salario() {
        let personas = [
            persona("A", 20, 3000.0),
            persona("B", 30, 3000.0),
            persona("C", 40, 3000.0),
            persona("D", 50, 3000.0),
        ];

        let resultado = get_personas_con_mayor_y_menor_salario(personas);
        // empate en ambos extremos, se eligen los de mayor edad
        assert_eq!(resultado[0].nombre, "D"); // menor por edad
        assert_eq!(resultado[1].nombre, "D"); // mayor por edad (igual)
    }
}
