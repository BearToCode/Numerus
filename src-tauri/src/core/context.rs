use super::functions::Expression;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Context {
    pub definitions: HashMap<char, f64>, // Ex: x = 4
    pub user_functions: Vec<Expression>, // All custom functions provided by the user
}