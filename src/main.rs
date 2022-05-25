// https://doc.rust-lang.ru/book/ch03-05-control-flow.html
// конвертер температур из единиц Фаренгейта в единицы Цельсия
// C = F - 32

use std::io;
fn main() {
// Получаем данные от пользователя
    let mut input:String = String::new();
    io::stdin()
		.read_line(&mut input)
        .expect("Failed to conversion type.");
// Отрезаем от String символы \n
    let x = input.len() - 1;
    let slice = &input[..x];
// Конвертируем slice в i32
    let fahrenheit: i32 = slice.parse::<i32>().expect("Not a number");
// Создаём переменную для храния результата
// конвертации из Ф в Ц
    let celsius: i32 = fahrenheit - 32;
	println!("Celsius temperature is: {}", celsius);
}
