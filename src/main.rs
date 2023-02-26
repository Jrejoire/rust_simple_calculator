use std::io;

fn find_next_operation_index(split_equation: &Vec<String>) -> Result<usize, io::Error> {
    let divide_index = split_equation.iter().position(|x| x == "/").unwrap_or(0);
    if divide_index > 0 {
        return Ok(divide_index);
    }

    let multiply_index = split_equation.iter().position(|x| x == "*").unwrap_or(0);
    if multiply_index > 0 {
        return Ok(multiply_index);
    }

    let sum_index = split_equation.iter().position(|x| x == "+").unwrap_or(0);
    if sum_index > 0 {
        return Ok(sum_index);
    }

    let difference_index = split_equation.iter().position(|x| x == "-").unwrap_or(0);
    if difference_index > 0 {
        return Ok(difference_index);
    }

    return Err(io::Error::new(io::ErrorKind::NotFound, "Not found"));
}

fn calculate_equation(slice_equation: &[String]) -> Result<String, io::Error> {
    let result: String;
    let operator: &str = &slice_equation[1][..];
    let left_value: f64 = slice_equation[0].trim().parse::<f64>().unwrap();
    let right_value: f64 = slice_equation[2].trim().parse::<f64>().unwrap();

    match operator {
        "+" => result = (left_value + right_value).to_string(),
        "-" => result = (left_value - right_value).to_string(),
        "*" => result = (left_value * right_value).to_string(),
        "/" => {
            if slice_equation[2] == "0" {
                return Err(io::Error::new(io::ErrorKind::InvalidInput, "division by 0"));
            } else {
                result = (left_value / right_value).to_string();
            }
        }
        _ => unreachable!(),
    }
    return Ok(result);
}

fn main() -> Result<(), io::Error> {
    println!("Enter the expression to calculate (separate with spaces, no parenthesis):");
    let mut string_expression = String::new();

    io::stdin()
        .read_line(&mut string_expression)
        .expect("Failed to read line");

    let borrowed_split_expression = string_expression.trim().split(" ").collect::<Vec<&str>>();
    let mut split_expression = Vec::new();

    for borrowed in borrowed_split_expression {
        split_expression.push(String::from(borrowed))
    }

    while split_expression.len() > 1 {
        let next_equation_index = find_next_operation_index(&split_expression).unwrap();
        let slice_equation = &split_expression[next_equation_index - 1..next_equation_index + 2];

        let string_result = calculate_equation(slice_equation).unwrap();

        split_expression.remove(next_equation_index + 1);
        split_expression.remove(next_equation_index);
        split_expression[next_equation_index - 1] = string_result;
    }

    println!("result: {}", split_expression[0]);

    Ok(())
}
