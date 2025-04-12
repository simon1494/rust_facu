use std::io::stdin;
fn main() {
    let mut testigo: bool = false;
    let mut input: String = String::new();
    let frutas: [&str; 10] = [
        "banana", "manzana", "kiwi", "naranja", "melon", "anana", "pera", "ciruela", "tomate",
        "pomelo",
    ];

    stdin().read_line(&mut input).expect("Error");

    for fruta in frutas.iter() {
        if fruta == &input.trim() {
            testigo = true;
        }
    }
    if testigo {
        println!("Si hay");
    } else {
        println!("No hay");
    }
}
