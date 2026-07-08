impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = Vec::new();
        for c in &tokens {
            match c.as_str() {
                "+" => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(b + a);
                }
                "-" => {
                    let a: i32 = stack.pop().unwrap();
                    let b: i32 = stack.pop().unwrap();
                    stack.push(b - a);
                }
                "*" => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(b * a);
                }
                "/" => {
                    let a: i32 = stack.pop().unwrap();
                    let b: i32 = stack.pop().unwrap();
                    stack.push(b / a);
                }
                _ => {
                    stack.push(c.parse::<i32>().unwrap());
                }
            }
        }
        stack[0]
    }
}