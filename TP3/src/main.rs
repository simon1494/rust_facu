mod ejercicios;

fn main() {
    //borrowing y ownership

    fn multiplicar_algo(algo: i32) -> i32 {
        return algo * 2;
    }

    let variable: i32 = 4;

    multiplicar_algo(variable);

    println!("{}", variable);
}
