use cocktails::menu::{ingredients::IngredientAmount, recipe::Recipe, Menu};
use std::collections::HashMap;

fn main() {
    let menu = Menu::new("My menu");
    let americano_ingredients = HashMap::from([(
        String::from("Campari"),
        IngredientAmount {
            item_id: String::from("Campari"),
            amount: 3,
            unit: String::from("cl"),
        },
    )]);
    let americano = Recipe::new("Americano", "N/A", americano_ingredients, "Mix");
    let menu = menu.add_recipe(americano);
    println!("{:?}", menu)
}
