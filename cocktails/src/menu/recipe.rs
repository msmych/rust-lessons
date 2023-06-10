use std::collections::HashMap;
use uuid::Uuid;

use crate::menu::ingredients::IngredientAmount;

#[derive(Debug, Clone)]
pub struct Recipe {
    id: Uuid,
    name: String,
    description: String,
    items: HashMap<String, IngredientAmount>,
    instruction: String,
}

impl Recipe {
    pub fn new(
        name: &str,
        description: &str,
        items: HashMap<String, IngredientAmount>,
        instruction: &str,
    ) -> Self {
        Recipe {
            id: Uuid::new_v4(),
            name: name.to_string(),
            description: description.to_string(),
            items: items,
            instruction: instruction.to_string(),
        }
    }
}
