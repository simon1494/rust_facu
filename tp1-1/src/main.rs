use std::io::stdin;

fn main() {
    const NUM: f64 = 5.3;
    let mut input: String = String::new();

    println!("Ingresa un numero para operar sobre 5.3: ");
    stdin().read_line(&mut input).expect("No seas trooolo");

    let numero: f64 = input.trim().parse().expect("Eso no es un numero");

    println!("{}", NUM + numero);
    println!("{}", NUM - numero);
    println!("{}", NUM / numero);
    println!("{}", NUM * numero);

    stdin().read_line(&mut input).expect("No seas trooolo");
}
