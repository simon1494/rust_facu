use crate::tp2::es_par::es_par;

pub fn suma_pares(arreglo: &[i32]) -> i32 {
    let mut suma: i32 = 0;
    for &numero in arreglo {
        if es_par(numero) {
            suma += &numero;
        }
    }
    return suma;
}

#[test]
fn suma_ningun_par() {
    let arreglo: [i32; 5] = [-1, 3, 5, 7, -9];
    let suma: i32 = suma_pares(&arreglo);
    assert_eq!(suma, 0);
}
#[test]
fn suma_un_par() {
    let arreglo: [i32; 5] = [1, 4, 5, -7, 9];
    let suma: i32 = suma_pares(&arreglo);
    assert_eq!(suma, 4);
}
#[test]
fn suma_muchos_pares() {
    let arreglo: [i32; 5] = [2, 4, -6, 8, 10];
    let suma: i32 = suma_pares(&arreglo);
    assert_eq!(suma, 18);
}
