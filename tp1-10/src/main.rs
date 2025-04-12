fn main() {
    let arreglo0 = [1, 2, 3, 4, 5];
    let arreglo1 = [6, 7, 8, 9, 10];
    let mut arreglo2: [i32; 5] = [0, 0, 0, 0, 0];

    arreglo2[0] = arreglo0[0] + arreglo1[0];
    arreglo2[1] = arreglo0[1] + arreglo1[1];
    arreglo2[2] = arreglo0[2] + arreglo1[2];
    arreglo2[3] = arreglo0[3] + arreglo1[3];
    arreglo2[4] = arreglo0[4] + arreglo1[4];

    println!("{}", arreglo2[0]);
    println!("{}", arreglo2[1]);
    println!("{}", arreglo2[2]);
    println!("{}", arreglo2[3]);
    println!("{}", arreglo2[4]);
}
