use std::collections::HashMap;

use uuid::Uuid;

#[derive(Debug)]
pub struct Ingredient {
    id: Uuid,
    name: String,
    owner: Option<Uuid>,
}

impl Ingredient {
    pub fn common(name: &str) -> Self {
        Ingredient {
            id: Uuid::new_v4(),
            name: name.to_string(),
            owner: None,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
}

#[derive(Debug, Clone)]
pub enum Amount {
    Some,
    Cl(u8),
}

pub struct IngredientService {
    ingredients: HashMap<String, Ingredient>,
}

impl IngredientService {
    pub fn create() -> Self {
        IngredientService {
            ingredients: HashMap::new(),
        }
    }

    pub fn add(&mut self, ingredient: Ingredient) {
        self.ingredients.insert(ingredient.name.clone(), ingredient);
    }

    pub fn get_by_name(&self, name: &str) -> &Ingredient {
        self.ingredients
            .get(&name.to_string())
            .expect(format!("Not found ingredient with name {}", name).as_str())
    }
}
