use crate::core::functions;

use super::utils::{get_adjacent_expression_end_index, Direction};

pub fn operators_to_functions(string: &mut String) -> Result<(), &'static str> {
    loop {
        let (current_operator_index, current_operator_name) =
            match get_highest_priority_operator(&string) {
                Some(result) => result,
                None => break,
            };

        // Replace operator char with a comma
        string.replace_range(current_operator_index..current_operator_index + 1, ",");

        println!("comma: {}", string);

        let left_expression_end = match get_adjacent_expression_end_index(
            &string,
            current_operator_index,
            Direction::Left,
        ) {
            Ok(result) => result,
            Err(msg) => return Err(msg),
        };

        println!("right...");

        let right_expression_end = match get_adjacent_expression_end_index(
            string,
            current_operator_index,
            Direction::Right,
        ) {
            Ok(result) => result,
            Err(msg) => return Err(msg),
        };

        // Place closing bracket at the end of the right expression
        string.insert(right_expression_end, ')');
        // Place function and opening bracket at the beginning of the left expression
        string.insert_str(
            left_expression_end,
            &(current_operator_name.to_owned() + "("),
        );
    }
    remove_useless_brackets(string);
    println!("{}", string);
    Ok(())
}

// Returns the index and the function of the highest priority operator
fn get_highest_priority_operator(string: &String) -> Option<(usize, &'static str)> {
    let mut current_depth: u16 = 0;
    let mut highest_priority_operator_index: usize = 0;
    let mut highest_priority_operator_name: &str = "";
    let mut highest_priority: (u16, u8) = (0, 0);
    for (i, char) in string.chars().enumerate() {
        if char == '(' {
            current_depth += 1;
        } else if char == ')' {
            current_depth -= 1;
        }

        let (operator_name, operator_priority) =
            match functions::operators::operator_data_from_char(&char) {
                Some(value) => value,
                None => continue,
            };

        let this_priority = (current_depth, operator_priority);

        if this_priority > highest_priority {
            highest_priority = this_priority;
            highest_priority_operator_name = operator_name;
            highest_priority_operator_index = i;
            println!("high: {}", highest_priority_operator_index);
        }
    }

    println!("high: {}", highest_priority_operator_index);

    println!("{:?}", highest_priority);

    if highest_priority == (0, 0) {
        None
    } else {
        Some((
            highest_priority_operator_index,
            highest_priority_operator_name,
        ))
    }
}

// To be executed after operators have been converted, otherwise
// operations will be executed in the wrong order
fn remove_useless_brackets(string: &mut String) {
    let mut previous_is_letter = false;
    let mut index = 0;
    while index < string.len() {
        let char = string.chars().nth(index).unwrap();

        if index == string.len() - 2 {
            return;
        }

        if char == '(' {
            if !previous_is_letter {
                // Remove the corresponding bracket
                let mut inner_index = index + 1;
                let mut inner_depth: i16 = 1;
                while inner_index < string.len() {
                    let inner_char = string.chars().nth(inner_index).unwrap();
                    if inner_char == '(' {
                        inner_depth += 1;
                    } else if inner_char == ')' {
                        inner_depth -= 1;
                        if inner_depth == 0 {
                            // Remove the bracket
                            string.replace_range(inner_index..inner_index + 1, "");
                            break;
                        }
                    }
                    inner_index += 1;
                }
                // Remove the starting bracket
                string.replace_range(index..index + 1, "");
                index -= 1;
            }
        }
        previous_is_letter = char.is_alphabetic();
        index += 1;
    }
}
