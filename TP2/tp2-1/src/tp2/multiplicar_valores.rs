pub fn multiplicar_valores(arreglo: &mut [i32], factor: i32) {
    for numero in arreglo {
        *numero *= factor;
    }
}

#[test]
fn test_duplico_todo() {
    let mut arreglo: [i32; 5] = [1, 2, 3, 4, 5];
    let esperado: [i32; 5] = [3, 6, 9, 12, 15];
    multiplicar_valores(&mut arreglo, 3);
    assert_eq!(arreglo, esperado);
}

#[test]
fn test_duplico_nada() {
    let mut arreglo: [i32; 0] = [];
    let esperado: [i32; 0] = [];
    multiplicar_valores(&mut arreglo, 3);
    assert_eq!(arreglo, esperado);
}
