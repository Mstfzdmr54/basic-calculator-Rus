fn main() {
    let num1 = 10;
    let num2 = 5;

    


    let operators = vec![
        Operator::Add,
        Operator::Subtract,
        Operator::Multiply,
        Operator::Divide,
    ];

    for equal in operators {


    if let Operator::Add = equal{
        let result = calculate(Operator::Add, num1, num2);
        println!("{} + {} = {}", num1, num2, result);
    }else if let Operator::Subtract = equal{
        let result = calculate(Operator::Subtract, num1, num2);
        println!("{} - {} = {}", num1, num2, result);
    }else if let Operator::Multiply = equal{
        let result = calculate(Operator::Multiply, num1, num2);
        println!("{} * {} = {}", num1, num2, result);
    }else if let Operator::Divide = equal{
        let result = calculate(Operator::Divide, num1, num2);
        println!("{} / {} = {}", num1, num2, result);
    }

    }
}

enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

fn calculate(operator: Operator, num1: i32, num2: i32) -> i32 {
    match operator {
        Operator::Add => num1 + num2,
        Operator::Subtract => num1 - num2,
        Operator::Multiply => num1 * num2,
        Operator::Divide => num1 / num2,
    }
}


