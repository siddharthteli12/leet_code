pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut operand_stack: Vec<i32> = vec![];

    for token in tokens {
        if let Ok(digit) = token.parse::<i32>() {
            operand_stack.push(digit);
        } else {
            let result = calculate(
                operand_stack.pop().unwrap(),
                operand_stack.pop().unwrap(),
                token.parse::<char>().unwrap(),
            );
            operand_stack.push(result);
        }
    }

    operand_stack.pop().unwrap()
}
pub fn calculate(operand1: i32, operand2: i32, operator: char) -> i32 {
    match operator {
        '+' => operand2 + operand1,
        '-' => operand2 - operand1,
        '*' => operand2 * operand1,
        '/' => operand2 / operand1,
        _ => unreachable!(),
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let tokens = vec!["2", "1", "+", "3", "*"];
        assert_eq!(
            eval_rpn(
                tokens
                    .iter()
                    .map(|slice| slice.to_string())
                    .collect::<Vec<String>>()
            ),
            9
        );
    }
}
