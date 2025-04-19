pub fn es_primo(numero: i32) -> bool {
    if numero <= 1 {
        return false;
    };
    if numero == 2 {
        return true;
    };
    if numero % 2 == 0 {
        return false;
    }
    let limite: i32 = (numero as f64).sqrt() as i32;
    for i in (3..=limite).step_by(2) {
        if numero % i == 0 {
            return false;
        }
    }
    return true;
}

#[test]
fn test_si_es_1() {
    let numero: i32 = 1;
    let resultado: bool = es_primo(numero);
    assert_eq!(resultado, false);
}

#[test]
fn test_si_es_2() {
    let numero: i32 = 2;
    let resultado: bool = es_primo(numero);
    assert_eq!(resultado, true);
}

#[test]
fn test_si_es_primo() {
    let numero: i32 = 29;
    let resultado: bool = es_primo(numero);
    assert_eq!(resultado, true);
}

#[test]
fn test_no_es_primo() {
    let numero: i32 = 4;
    let resultado: bool = es_primo(numero);
    assert_eq!(resultado, false);
}
