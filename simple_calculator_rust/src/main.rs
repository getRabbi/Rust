
use std::io;

fn main() {
   loop {
        let mut number1 = String::new();
    let mut number2 = String::new();
    let mut operator = String::new();

    println!("input the number: {}", number1);
    
    io::stdin()
        .read_line(&mut number1)
        .expect("fail to input number");
    
    println!("input the number: {}", number2);

    io::stdin()
        .read_line(&mut number2)
        .expect("fail to input number");

    println!("input operator you like [+,-,*,/,%]: {}", operator);

    io::stdin()
        .read_line(&mut operator)
        .expect("Fail to input operator");
    
    let number1:f32 = number1.trim().parse().expect("Not an nubmer");
    let number2:f32 = number2.trim().parse().expect("Not an number");
    let operator:char = operator.trim().chars().next().expect("Not an operator");

    let result = match operator {
        '+' => number1 + number2,
        '-' => number1 - number2,
        '*' => number1 * number2,
        '/' => number1 / number2,
        '%' => number1 % number2,
        _ => panic!("something wrong")
    };

    println!("the resluts of {} {} {} = {}", number1, operator, number2, result);
   }
}
