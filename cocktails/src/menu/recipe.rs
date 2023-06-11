use crate::menu::ingredients::Amount;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Recipe {
    id: Uuid,
    name: String,
    description: String,
    ingredients: HashMap<Uuid, Amount>,
    instruction: String,
}

impl Recipe {
    pub fn new(
        name: &str,
        description: &str,
        ingredients: HashMap<Uuid, Amount>,
        instruction: &str,
    ) -> Self {
        Recipe {
            id: Uuid::new_v4(),
            name: name.to_string(),
            description: description.to_string(),
            ingredients,
            instruction: instruction.to_string(),
        }
    }
}
