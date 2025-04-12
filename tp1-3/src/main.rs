use std::io::stdin;

fn main() {
    let valor: bool = true;
    let mut input: String = String::new();

    println!("Ingrese un valor booleano para operar sobre 'true':");

    stdin().read_line(&mut input).expect("Error");

    let cast: bool = input.trim().parse().expect("No corresponde a booleano");

    println!("{}", valor & cast);
    println!("{}", valor | cast);
}
