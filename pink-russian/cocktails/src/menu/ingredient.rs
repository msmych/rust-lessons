use std::collections::HashMap;

use uuid::Uuid;

#[derive(Debug, Clone)]
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
    Oz(u8),
    Cl(u8),
    Some,
}

pub struct IngredientService {
    ingredients: HashMap<Uuid, Ingredient>,
}

impl IngredientService {
    pub fn create() -> Self {
        IngredientService {
            ingredients: HashMap::new(),
        }
    }

    pub fn add(&mut self, ingredient: Ingredient) {
        self.ingredients.insert(ingredient.id.clone(), ingredient);
    }

    pub fn get(&self, id: Uuid) -> &Ingredient {
        self.ingredients
            .get(&id)
            .expect(format!("Not found ingredient by id {:?}", id).as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::{Ingredient, IngredientService};

    #[test]
    fn should_create_ingredients_service_with_empty_ingredients() {
        let ingrs = IngredientService::create();
        assert!(ingrs.ingredients.is_empty());
    }

    #[test]
    fn should_add_and_get_ingredient() {
        let mut ingrs = IngredientService::create();

        let gin = Ingredient::common("Gin");
        let campari = Ingredient::common("Campari");

        ingrs.add(gin.clone());
        ingrs.add(campari.clone());

        assert_eq!(ingrs.ingredients.len(), 2);
        assert_eq!(ingrs.get(gin.id).id, gin.id);
        assert_eq!(ingrs.get(campari.id).id, campari.id);
    }
}
