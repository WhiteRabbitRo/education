fn main() {

    // ЗАДАЧА 1

    // Копирование
    let a: i32 = 5;
    let b: i32 = a;

    // Перемещение
    let str1 = String::from("hello");
    let str2 = str1;

    println!("a = {}", a);
    println!("b = {}", b);
    // println!("str1 = {}", str1); // Ошибка! Значение было перемещено
    println!("str2 = {}", str2);

    // ЗАДАЧА 2

    let mut v = vec![1, 2, 3];
    swap_elements(&mut v, 0, 2);
    println!("v = {:?}", v);

    // ЗАДАЧА 3

    let s1 = String::from("hello");
    let s2 = String::from("world");
    let longest = longest_word(&s1, &s2);
    println!("The longest word is: {}", longest);

    let s1 = String::from("hello");
    let s2 = String::from("marketplace");
    let longest = longest_word(&s1, &s2);
    println!("The longest word is: {}", longest);

    // ЗАДАЧА 4

    // Ошибка возникала, потому что вектор data был перемещен в неизменяемую переменную,
    // после чего произошло изменение вектора. Следовательно мы либо должны использовать
    // неизменяемую переменную и вернуться к изменяемой, либо не использовать неизменяемую
    // переменную.
    let mut data = vec![1, 2, 3];
    let first = &data[0];
    println!("First elements: {}", first);
    data.push(4);

    // ЗАДАЧА 5

    let s = String::from("hello world");
    let word = first_word(&s);
    println!("The first word is: {}", word);

    let s = String::from("hello");
    let word = first_word(&s);
    println!("The first word is: {}", word);
}

fn swap_elements<T>(v: &mut Vec<T>, i: usize, j: usize) {
    if i >= v.len() || j >= v.len() {
        panic!("Index out of bounds");
    }

    if i == j {
        return;
    }

    v.swap(i, j);
}

fn longest_word<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() >= s2.len() {
        s1
    } else {
        s2
    }
}

fn first_word(s: &str) -> &str {
    let mut i = 0;
    while (s.chars().nth(i) != Some(' ')) && (i < s.len()) {
        i += 1;
    }
    &s[0..i]
}