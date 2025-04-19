pub fn longitud_de_cadenas(strings: &[String]) -> Vec<usize> {
    strings.iter().map(|s| s.len()).collect()
}

#[test]
fn test_longitudes() {
    let cadenas: [String; 5] = [
        "1".to_string(),
        "22".to_string(),
        "333".to_string(),
        "4444".to_string(),
        "55555".to_string(),
    ];
    let esperado: Vec<usize> = vec![1, 2, 3, 4, 5];
    let resultado: Vec<usize> = longitud_de_cadenas(&cadenas);
    assert_eq!(esperado, resultado);
}

#[test]
fn test_0_longitudes() {
    let cadenas: [String; 0] = [];
    let esperado: Vec<usize> = vec![];
    let resultado: Vec<usize> = longitud_de_cadenas(&cadenas);
    assert_eq!(esperado, resultado);
}
