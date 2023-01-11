#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn update_stack(stack: &mut Vec<i32>) -> Result<(i32, i32), ()> {
    let (first_val, second_val): (i32, i32);

    match stack.pop() {
        Some(val) => second_val = val.clone(),
        None => return Err(()),
    }

    match stack.pop() {
        Some(val) => first_val = val.clone(),
        None => return Err(()),
    }

    Ok((first_val, second_val))
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = vec![];

    for input in inputs {
        match input {
            CalculatorInput::Add => {
                match update_stack(&mut stack) {
                    Ok(values) => stack.push(values.0 + values.1),
                    Err(_) => return None,
                };
            }
            CalculatorInput::Subtract => {
                match update_stack(&mut stack) {
                    Ok(values) => stack.push(values.0 - values.1),
                    Err(_) => return None,
                };
            }
            CalculatorInput::Multiply => {
                match update_stack(&mut stack) {
                    Ok(values) => stack.push(values.0 * values.1),
                    Err(_) => return None,
                };
            }
            CalculatorInput::Divide => {
                match update_stack(&mut stack) {
                    Ok(values) => stack.push(values.0 / values.1),
                    Err(_) => return None,
                };
            }
            CalculatorInput::Value(num) => stack.push(*num),
        }
    }

    if stack.len() > 1 {
        None
    } else {
        stack.pop()
    }
}
