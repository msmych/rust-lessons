use cocktails::menu::ingredients::{Amount, Ingredient, IngredientService};
use cocktails::menu::{recipe::Recipe, Menu};
use cocktails::{Account, AccountService};
use std::collections::HashMap;

fn main() {
    let mut account_service = AccountService::create();
    let account = Account::new();
    account_service.add(account.clone());
    let ingredient_service = init_ingredients_service();
    let campari = ingredient_service.get_by_name("Campari");
    let red_vermouth = ingredient_service.get_by_name("Red Vermouth");
    let soda_water = ingredient_service.get_by_name("Soda water");
    let americano_ingredients = HashMap::from([
        (campari.id(), Amount::Cl(3)),
        (red_vermouth.id(), Amount::Cl(3)),
        (soda_water.id(), Amount::Some),
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

fn init_ingredients_service() -> IngredientService {
    let mut ingredient_service = IngredientService::create();
    ingredient_service.add(Ingredient::common("Campari"));
    ingredient_service.add(Ingredient::common("Red Vermouth"));
    ingredient_service.add(Ingredient::common("Soda water"));
    ingredient_service
}
