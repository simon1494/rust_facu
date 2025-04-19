fn incrementar(numero: &mut f64, veces: &u128) {
    for i in 0..*veces {
        *numero += 1 as f64;
        //println!("{}", *numero);
    }
}

#[test]
fn test_incrementar() {
    let veces: u128 = 5000000;
    let mut numero: f64 = 0.0;
    incrementar(&mut numero, &veces);
    assert_eq!(numero, 5000000.0);
}
