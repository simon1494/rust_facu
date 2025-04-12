use std::io::stdin;

fn main() {
    const NUM: i32 = 5;
    let mut input: String = String::new();

    println!("Ingrese un numero entero para operar:");
    stdin().read_line(&mut input).expect("Error");

    let resultado: i32 = input.trim().parse().expect("Error");

    println!("{}", (NUM + resultado).pow(2));
}
