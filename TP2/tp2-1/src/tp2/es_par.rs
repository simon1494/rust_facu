pub fn es_par(numero: i32) -> bool {
    if numero % 2 == 0 {
        return true;
    } else {
        return false;
    }
}

#[test]
fn test_no_es_par_si() {
    let impar: i32 = 1;
    let resultado: bool = es_par(impar);
    assert_eq!(resultado, false);
}

#[test]
fn test_no_es_par() {
    let par: i32 = 2;
    let resultado: bool = es_par(par);
    assert_eq!(resultado, true);
}
