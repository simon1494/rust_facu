fn main() {
    const VALOR: i32 = 8;
    let mut arreglo: [i32; 6] = [5, 9, 8, 3, 45, -9];

    arreglo[0] *= VALOR;
    arreglo[1] *= VALOR;
    arreglo[2] *= VALOR;
    arreglo[3] *= VALOR;
    arreglo[4] *= VALOR;
    arreglo[5] *= VALOR;

    println!(
        "{}, {}, {}, {}, {}, {}",
        arreglo[0], arreglo[1], arreglo[2], arreglo[3], arreglo[4], arreglo[5]
    );
}
