pub fn sumar_arreglos(arreglo0: &[i32], arreglo1: &[i32]) -> Vec<i32> {
    if arreglo0.len() != arreglo1.len() {
        let retorno: Vec<i32> = Vec::new();
        return retorno;
    }
    arreglo0
        .iter()
        .zip(arreglo1.iter())
        .map(|(a, b)| a + b)
        .collect()
}

#[test]
fn test_sumar_arreglos() {
    let arreglo0: [i32; 3] = [1, 2, 3];
    let arreglo1: [i32; 3] = [4, 5, 6];
    let resultado: Vec<i32> = sumar_arreglos(&arreglo0, &arreglo1);
    let esperado: Vec<i32> = vec![5, 7, 9];
    assert_eq!(resultado, esperado);
}

#[test]
fn test_sumar_nada() {
    let arreglo0: [i32; 0] = [];
    let arreglo1: [i32; 0] = [];
    let resultado: Vec<i32> = sumar_arreglos(&arreglo0, &arreglo1);
    let esperado: Vec<i32> = vec![];
    assert_eq!(resultado, esperado);
}

#[test]
fn test_sumar_diferentes() {
    let arreglo0: [i32; 0] = [];
    let arreglo1: [i32; 1] = [1];
    let resultado: Vec<i32> = sumar_arreglos(&arreglo0, &arreglo1);
    let esperado: Vec<i32> = vec![];
    assert_eq!(resultado, esperado);
}
