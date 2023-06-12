use cocktails::menu::ingredients::{Amount, Ingredient};
use cocktails::menu::{recipe::Recipe, Menu};
use std::collections::HashMap;

fn main() {
    let ingredients = common_ingredients();
    let campari = ingredients.get("Campari").expect("msg").id();
    let red_vermouth = ingredients.get("Red Vermouth").expect("msg").id();
    let soda_water = ingredients.get("Soda water").expect("msg").id();
    let americano_ingredients = HashMap::from([
        (campari, Amount::Cl(3)),
        (red_vermouth, Amount::Cl(3)),
        (soda_water, Amount::Some),
    ]);
    let americano = Recipe::new(
        "Americano",
        "N/A",
        americano_ingredients,
        "Mix the ingredients directly in an old-fashioned glass \
    filled with ice cubes, 
    add a splash of soda water and garnish with half orange slice",
    );
    let menu = Menu::new("My menu");
    let menu = menu.add_recipe(americano);
    println!("{:?}", menu)
}

fn common_ingredients() -> HashMap<String, Ingredient> {
    let mut ingredients = HashMap::new();
    ingredients.insert(String::from("Campari"), Ingredient::common("Campari"));
    ingredients.insert(
        String::from("Red Vermouth"),
        Ingredient::common("Red Vermouth"),
    );
    ingredients.insert(String::from("Soda water"), Ingredient::common("Soda water"));
    ingredients
}
