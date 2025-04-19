pub fn ordenar_nombres(arreglo: &mut [String]) {
    arreglo.sort();
}

#[test]
fn testear_nombres() {
    let mut nombres: [String; 5] = [
        "simon".to_string(),
        "tadeo".to_string(),
        "ale".to_string(),
        "ruben".to_string(),
        "maylen".to_string(),
    ];
    ordenar_nombres(&mut nombres);
    assert_eq!(nombres, ["ale", "maylen", "ruben", "simon", "tadeo"])
}
