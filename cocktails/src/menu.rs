use recipe::Recipe;
use uuid::Uuid;

pub mod ingredients;
pub mod recipe;

#[derive(Debug)]
pub struct Menu {
    id: Uuid,
    name: String,
    recipes: Vec<Recipe>,
}

impl Menu {
    pub fn new(name: &str) -> Self {
        Menu {
            id: Uuid::new_v4(),
            name: name.to_string(),
            recipes: vec![],
        }
    }

    pub fn add_recipe(self, recipe: Recipe) -> Self {
        let mut recipes = self.recipes.clone();
        recipes.push(recipe);
        Menu {
            recipes,
            id: self.id,
            name: self.name,
        }
    }
}
