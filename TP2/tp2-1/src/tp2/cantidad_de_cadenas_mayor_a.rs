pub fn cantidad_de_cadenas_mayor_a(arreglo: &[String], limite: usize) -> u32 {
    let mut cantidad: u32 = 0;
    for item in arreglo {
        if item.len() > limite {
            cantidad += 1;
        }
    }
    return cantidad;
}

#[test]
fn test_cantidad_0() {
    let arreglo: [String; 2] = ["hola".to_string(), "simon".to_string()];
    let resultado: u32 = cantidad_de_cadenas_mayor_a(&arreglo, 8);
    assert_eq!(resultado, 0);
}

#[test]
fn test_cantidad_1() {
    let arreglo: [String; 2] = ["hola".to_string(), "simon".to_string()];
    let resultado: u32 = cantidad_de_cadenas_mayor_a(&arreglo, 4);
    assert_eq!(resultado, 1);
}

#[test]
fn test_cantidad_2() {
    let arreglo: [String; 2] = ["hola".to_string(), "simon".to_string()];
    let resultado: u32 = cantidad_de_cadenas_mayor_a(&arreglo, 3);
    assert_eq!(resultado, 2);
}
