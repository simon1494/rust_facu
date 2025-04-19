use crate::tp2::es_par::es_par;

pub fn reemplazar_pares(arreglo: &mut [i32]) {
    for numero in arreglo {
        if es_par(*numero) {
            *numero = -1;
        }
    }
}

#[test]
fn test_reemplazar_0() {
    let mut arreglo: [i32; 5] = [1, 3, 5, 7, 9];
    let esperado: [i32; 5] = [1, 3, 5, 7, 9];
    reemplazar_pares(&mut arreglo);
    assert_eq!(esperado, arreglo)
}

#[test]
fn test_reemplazar_1() {
    let mut arreglo: [i32; 5] = [1, 2, 5, 7, 9];
    let esperado: [i32; 5] = [1, -1, 5, 7, 9];
    reemplazar_pares(&mut arreglo);
    assert_eq!(esperado, arreglo)
}

#[test]
fn test_reemplazar_5() {
    let mut arreglo: [i32; 5] = [0, 2, 4, 6, 8];
    let esperado: [i32; 5] = [-1, -1, -1, -1, -1];
    reemplazar_pares(&mut arreglo);
    assert_eq!(esperado, arreglo)
}
