fn main() {
    let cantidad: usize = "Bueno a ver como carajo hacemos esto"
        .chars()
        .filter(|c: &char| *c == 'a')
        .count();

    println!("{}", cantidad);
}
