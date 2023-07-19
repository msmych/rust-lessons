use std::{collections::HashMap, sync::Mutex};

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Ingredient {
    id: Uuid,
    name: String,
    owner: Option<Uuid>,
}

impl Ingredient {
    pub fn new(name: &str, owner_id: Option<Uuid>) -> Self {
        Ingredient {
            id: Uuid::new_v4(),
            name: name.to_string(),
            owner: owner_id,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
}

#[derive(Debug, Clone)]
pub enum Amount {
    Oz(u8),
    Cl(u8),
    Some,
}

pub struct IngredientService {
    ingredients: Mutex<HashMap<Uuid, Ingredient>>,
}

impl IngredientService {
    pub fn create() -> Self {
        IngredientService {
            ingredients: Mutex::new(HashMap::new()),
        }
    }

    pub fn add(&self, ingredient: Ingredient) {
        self.ingredients
            .lock()
            .unwrap()
            .insert(ingredient.id.clone(), ingredient);
    }

    pub fn get(&self, id: Uuid) -> Ingredient {
        self.ingredients
            .lock()
            .unwrap()
            .get(&id)
            .expect(format!("Not found ingredient by id {:?}", id).as_str())
            .to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_ingredients_service_with_empty_ingredients() {
        let ingrs = IngredientService::create();
        assert!(ingrs.ingredients.lock().unwrap().is_empty());
    }

    #[test]
    fn should_add_and_get_ingredient() {
        let ingrs = IngredientService::create();

        let gin = Ingredient::new("Gin", None);
        let campari = Ingredient::new("Campari", None);

        ingrs.add(gin.clone());
        ingrs.add(campari.clone());

        assert_eq!(ingrs.ingredients.lock().unwrap().len(), 2);
        assert_eq!(ingrs.get(gin.id).id, gin.id);
        assert_eq!(ingrs.get(campari.id).id, campari.id);
    }
}
