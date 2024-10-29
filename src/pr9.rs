use std::io::{self, Write};


#[test]
fn main() {
    println!("Введіть число для перевірки:");

    // Отримуємо введення з командного рядка
    let mut input = String::new();
    io::stdout().flush().unwrap(); // Очищаємо буфер, щоб текст відобразився одразу
    io::stdin().read_line(&mut input).expect("Не вдалося зчитати рядок");

    // Перетворюємо введене значення на число
    let number: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Помилка: введіть дійсне число.");
            return;
        }
    };

    // Викликаємо функцію is_prime та виводимо результат
    if is_prime(&number) {
        println!("Число {} є простим.", number);
    } else {
        println!("Число {} не є простим.", number);
    }
}
fn is_prime(n: &u32) -> bool {
    if *n <= 1 {
        return false;
    }
    if *n <= 3 {
        return true;
    }
    if *n % 2 == 0 || *n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= *n {
        if *n % i == 0 || *n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}