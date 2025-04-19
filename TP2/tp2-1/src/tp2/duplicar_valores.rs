pub fn duplicar_valores(arreglo: &mut [f64]) {
    for numero in arreglo {
        *numero *= 2.0;
    }
}

#[test]
fn test_duplico_todo() {
    let mut arreglo: [f64; 5] = [1.5, 2.5, 3.5, 4.5, 5.5];
    let esperado: [f64; 5] = [3.0, 5.0, 7.0, 9.0, 11.0];
    duplicar_valores(&mut arreglo);
    assert_eq!(arreglo, esperado);
}

#[test]
fn test_duplico_nada() {
    let mut arreglo: [f64; 0] = [];
    let esperado: [f64; 0] = [];
    duplicar_valores(&mut arreglo);
    assert_eq!(arreglo, esperado);
}
