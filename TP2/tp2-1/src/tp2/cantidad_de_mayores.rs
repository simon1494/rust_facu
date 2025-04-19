pub fn cantidad_de_mayores(arreglo: &[i32], limite: i32) -> i32 {
    let mut cantidad: i32 = 0;
    for &numero in arreglo {
        if numero > limite {
            cantidad += 1;
        }
    }
    return cantidad;
}

#[test]
fn test_0_mayores() {
    let arreglo: [i32; 5] = [1, 2, 3, 4, 5];
    let cantidad: i32 = cantidad_de_mayores(&arreglo, 6);
    assert_eq!(cantidad, 0);
}

#[test]
fn test_1_mayores() {
    let arreglo: [i32; 5] = [1, 2, 3, 4, 5];
    let cantidad: i32 = cantidad_de_mayores(&arreglo, 4);
    assert_eq!(cantidad, 1);
}

#[test]
fn test_5_mayores() {
    let arreglo: [i32; 5] = [1, 2, 3, 4, 5];
    let cantidad: i32 = cantidad_de_mayores(&arreglo, -1);
    assert_eq!(cantidad, 5);
}
