impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = Vec::new();

        for token in tokens {
            match token.as_str() {
                "+" | "-" | "*" | "/" => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let result = match token.as_str() {
                        "+" => a + b,
                        "-" => a - b,
                        "*" => a * b,
                        "/" => a / b, // Rust integer division already truncates toward zero
                        _ => unreachable!(),
                    };
                    stack.push(result);
                }
                num => {
                    stack.push(num.parse::<i32>().unwrap());
                }
            }
        }

        stack.pop().unwrap()
    }
}