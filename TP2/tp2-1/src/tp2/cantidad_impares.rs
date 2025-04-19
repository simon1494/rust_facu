use crate::tp2::es_par::es_par;

pub fn cantidad_impares(arreglo: &[i32]) -> u32 {
    let mut suma: u32 = 0;
    for &numero in arreglo {
        if !es_par(numero) {
            suma += 1;
        }
    }
    return suma;
}

#[test]
fn test_cantidad_impares_0() {
    let arreglo: [i32; 5] = [-2, 2, -6, 4, -222];
    let cantidad: u32 = cantidad_impares(&arreglo);
    assert_eq!(cantidad, 0)
}

#[test]
fn test_cantidad_impares_1() {
    let arreglo: [i32; 5] = [-1, 2, 6, 4, -50];
    let cantidad: u32 = cantidad_impares(&arreglo);
    assert_eq!(cantidad, 1)
}

#[test]
fn test_cantidad_impares_4() {
    let arreglo: [i32; 5] = [1, -21, 31, 4, -51];
    let cantidad: u32 = cantidad_impares(&arreglo);
    assert_eq!(cantidad, 4)
}
