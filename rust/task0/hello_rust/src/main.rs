use std::io;

fn main() {
    println!("Как вас зовут?");

    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Не удалось прочитать строку");

    let name = name.trim(); // удаляем лишний перевод строки
    println!("Привет, {}! Добро пожаловать в Rust.", name);
}
