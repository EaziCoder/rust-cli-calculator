#![allow(unused_variables)]

use crate::calculator::{CalcError, Operation};

pub fn parse_command(input: &str) -> Result<(Operation, f64, f64), CalcError> {
    let parts = input.split_whitespace().collect::<Vec<&str>>();
    // turns "add 1 2" -> vec!["add" "1.0", "2.0"]

    if parts.len() == 2 {
        let operation = match parts[0].to_lowercase().as_str() {
            "sqrt" => Ok(Operation::Sqrt),
            "abs" => Ok(Operation::Abs),
            "neg" => Ok(Operation::Neg),
            _ => Err(CalcError::UnknownOperation),
        }?;

        let a = parts[1]
            .parse::<f64>()
            .map_err(|_| CalcError::ParseError(format!("Invalid number '{}'", parts[1])))?;

        return Ok((operation, a, 0.0));

        if parts.len() != 3 {
            return Err(CalcError::ParseError("Expected 2 or 3 input".into()));
        }
        // .to_string() always produces a String but .into() produces whatever type is required in that spot. In this case is a string
    }

    let operation = match parts[0].to_lowercase().as_str() {
        "add" => Ok(Operation::Add),
        "sub" => Ok(Operation::Sub),
        "mul" => Ok(Operation::Mul),
        "div" => Ok(Operation::Div),
        "pow" => Ok(Operation::Pow),
        "mod" => Ok(Operation::Mod),
        "log" => Ok(Operation::Log),
        _ => Err(CalcError::UnknownOperation),
    }?;
    // .to_lowercase().as_str() ensures case-insensitive commands.

    let a = parts[1]
        .parse::<f64>()
        .map_err(|_| CalcError::ParseError(format!("Invalid number '{}'", parts[1])))?;

    let b = parts[2]
        .parse::<f64>()
        .map_err(|_| CalcError::ParseError(format!("Invalid number '{}'", parts[2])))?;

    Ok((operation, a, b))
}
