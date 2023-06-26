use cocktails::menu::ingredient::{Amount, Ingredient, IngredientService};
use cocktails::menu::recipe::{Recipe, RecipeService};
use cocktails::menu::{Menu, MenuService};
use cocktails::{Account, AccountService};
use std::collections::HashMap;

fn main() {
    let mut account_service = AccountService::create();
    let account = Account::new();
    account_service.add(account.clone());

    println!("Account: {:?}", account_service.get(account.id()));

    let mut menu_service = MenuService::create();
    let menu = Menu::new(account.id(), "Summer menu");
    menu_service.add(menu.clone());

    println!("Menu: {:?}", menu_service.get(menu.id()));

    let mut ingredient_service = IngredientService::create();
    let campari = Ingredient::common("Campari");
    ingredient_service.add(campari.clone());
    let red_vermouth = Ingredient::common("Red Vermouth");
    ingredient_service.add(red_vermouth.clone());
    let soda_water = Ingredient::common("Soda water");
    ingredient_service.add(soda_water.clone());

    println!(
        "Campari: {:?}, Red Vermouth: {:?}, Soda water: {:?}",
        campari, red_vermouth, soda_water
    );

    let mut recipe_service = RecipeService::create();
    let americano = Recipe::new(
        "Americano",
        HashMap::from([
            (campari.id(), Amount::Oz(1)),
            (red_vermouth.id(), Amount::Oz(1)),
            (soda_water.id(), Amount::Some),
        ]),
        "Mix the ingredients directly in an old-fashioned glass \
    filled with ice cubes,
    add a splash of soda water and garnish with half orange slice",
    );
    recipe_service.add(americano.clone());

    println!("Americano: {:?}", americano);
}
