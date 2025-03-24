use crate::operations;

pub fn calc_with_option(operator: &str, numbers: &Vec<i32>) -> Option<f32> {
    match operator {
        "+" => Some(operations::summation(numbers)),
        "-" => Some(operations::subtraction(numbers)),
        "/" => Some(operations::division(numbers)),
        "*" => Some(operations::multiplication(numbers)),
        _ => None,
    }
}

pub fn calc_no_option(operator: &str, numbers: &Vec<i32>) -> f32 {
    match operator {
        "+" => operations::summation(numbers),
        "-" => operations::subtraction(numbers),
        "/" => operations::division(numbers),
        "*" => operations::multiplication(numbers),
        _ => 0.0,
    }
}
