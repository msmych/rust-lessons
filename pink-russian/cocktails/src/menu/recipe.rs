use crate::menu::ingredient::Amount;
use std::{collections::HashMap, sync::Mutex};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Recipe {
    id: Uuid,
    name: String,
    ingredients: HashMap<Uuid, Amount>,
    instruction: String,
}

impl Recipe {
    pub fn new(name: &str, ingredients: HashMap<Uuid, Amount>, instruction: &str) -> Self {
        Recipe {
            id: Uuid::new_v4(),
            name: name.to_string(),
            ingredients,
            instruction: instruction.to_string(),
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
}

pub struct RecipeService {
    recipes: Mutex<HashMap<Uuid, Recipe>>,
}

impl RecipeService {
    pub fn create() -> Self {
        RecipeService {
            recipes: Mutex::new(HashMap::new()),
        }
    }

    pub fn add(&self, recipe: Recipe) {
        self.recipes.lock().unwrap().insert(recipe.id, recipe);
    }

    pub fn get(&self, id: Uuid) -> Recipe {
        self.recipes
            .lock()
            .unwrap()
            .get(&id)
            .expect(format!("Not found recipe by id {:?}", id).as_str())
            .to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_recipe_service_with_empty_recipes() {
        let recipes = RecipeService::create();
        assert!(recipes.recipes.lock().unwrap().is_empty());
    }

    #[test]
    fn should_add_and_get_recipe() {
        let recipes = RecipeService::create();
        let gin_id = Uuid::new_v4();
        let sweet_vermouth_id = Uuid::new_v4();
        let campari_id = Uuid::new_v4();
        let tonic_water_id = Uuid::new_v4();
        let americano_ingredients = HashMap::from([
            (sweet_vermouth_id, Amount::Oz(1)),
            (campari_id, Amount::Oz(1)),
            (tonic_water_id, Amount::Some),
        ]);
        let americano = Recipe::new(
            "Americano",
            americano_ingredients,
            "Mix the ingredients directly in an old-fashioned glass \
        filled with ice cubes, \
        add a splash of soda water and garnish with half orange slice",
        );
        let negroni_ingredients = HashMap::from([
            (gin_id, Amount::Oz(1)),
            (sweet_vermouth_id, Amount::Oz(1)),
            (campari_id, Amount::Oz(1)),
            (tonic_water_id, Amount::Some),
        ]);
        let negroni = Recipe::new(
            "Negroni",
            negroni_ingredients,
            "Stir all the ingredients in a mixing glass with ice. \
        Strain into a rocks glass with a single huge ice cube. \
        Garnish with an orange peel",
        );

        recipes.add(americano.clone());
        recipes.add(negroni.clone());

        assert_eq!(recipes.recipes.lock().unwrap().len(), 2);
        assert_eq!(recipes.get(americano.id).id, americano.id);
        assert_eq!(recipes.get(negroni.id).id, negroni.id);
    }
}
