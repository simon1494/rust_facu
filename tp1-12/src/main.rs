fn main() {
    let arreglo: [i32; 2] = [1, 2];
    let tupla: (&str, [i32; 2]) = ("Suma: ", arreglo);
    let suma: i32 = arreglo.iter().sum();

    println!("{}", tupla.0);
    println!("{}", suma);
}
