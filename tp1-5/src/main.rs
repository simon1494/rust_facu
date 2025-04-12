use std::io::stdin;

fn main() {
    let hola: &str = "hola";
    let mut input: String = String::new();

    println!("Ingresa tu nombre:");
    stdin().read_line(&mut input).expect("Error");

    input = hola.to_string() + ", " + input.trim_end();

    println!("{}", input);
}
