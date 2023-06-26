use std::collections::HashMap;

use uuid::Uuid;

pub mod ingredient;
pub mod recipe;

#[derive(Debug, Clone)]
pub struct Menu {
    id: Uuid,
    account_id: Uuid,
    name: String,
}

impl Menu {
    pub fn new(account_id: Uuid, name: &str) -> Self {
        Menu {
            id: Uuid::new_v4(),
            account_id,
            name: name.to_string(),
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
}

pub struct MenuService {
    menus: HashMap<Uuid, Menu>,
}

impl MenuService {
    pub fn create() -> Self {
        MenuService {
            menus: HashMap::new(),
        }
    }

    pub fn add(&mut self, menu: Menu) {
        self.menus.insert(menu.id, menu);
    }

    pub fn get(&self, id: Uuid) -> &Menu {
        self.menus
            .get(&id)
            .expect(&format!("Not found menu by id = {}", id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_menu_service_with_empty_menus() {
        let menus = MenuService::create();
        assert!(menus.menus.is_empty());
    }

    #[test]
    fn should_add_and_get_menu() {
        let mut menus = MenuService::create();
        let menu1 = Menu::new(Uuid::new_v4(), "Menu 1");
        let menu2 = Menu::new(Uuid::new_v4(), "Menu 2");

        menus.add(menu1.clone());
        menus.add(menu2.clone());

        assert_eq!(menus.menus.len(), 2);
        assert_eq!(menus.get(menu1.id).id, menu1.id);
        assert_eq!(menus.get(menu2.id).id, menu2.id);
    }
}
