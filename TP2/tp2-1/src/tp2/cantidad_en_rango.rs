pub fn cantidad_en_rango(arreglo: &[i32], lim_inf: i32, lim_sup: i32) -> u32 {
    let mut cantidad: u32 = 0;
    for &numero in arreglo {
        if numero > lim_inf && numero <= lim_sup {
            cantidad += 1;
        };
    }
    return cantidad;
}

#[test]
fn test_cantidad_0() {
    let inf: i32 = -1;
    let sup: i32 = 0;
    let arreglo: [i32; 5] = [1, 2, 3, 4, 5];
    let cantidad: u32 = cantidad_en_rango(&arreglo, inf, sup);
    assert_eq!(cantidad, 0);
}

#[test]
fn test_cantidad_1() {
    let inf: i32 = -1;
    let sup: i32 = 1;
    let arreglo: [i32; 5] = [1, 2, 3, 4, 5];
    let cantidad: u32 = cantidad_en_rango(&arreglo, inf, sup);
    assert_eq!(cantidad, 1);
}

#[test]
fn test_cantidad_2() {
    let inf: i32 = -1;
    let sup: i32 = 10;
    let arreglo: [i32; 5] = [1, 2, 3, 4, 5];
    let cantidad: u32 = cantidad_en_rango(&arreglo, inf, sup);
    assert_eq!(cantidad, 5);
}
