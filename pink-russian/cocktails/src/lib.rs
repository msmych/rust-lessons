use std::{collections::HashMap, sync::Mutex};

use uuid::Uuid;

pub mod menu;

#[derive(Debug, Clone)]
pub struct Account {
    id: Uuid,
}

impl Account {
    pub fn new() -> Self {
        Account { id: Uuid::new_v4() }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
}

pub struct AccountService {
    accounts: Mutex<HashMap<Uuid, Account>>,
}

impl AccountService {
    pub fn create() -> Self {
        AccountService {
            accounts: Mutex::new(HashMap::new()),
        }
    }

    pub fn add(&self, account: Account) {
        self.accounts.lock().unwrap().insert(account.id, account);
    }

    pub fn get(&self, id: Uuid) -> Account {
        self.accounts
            .lock()
            .unwrap()
            .get(&id)
            .expect(&format!("Not found account by id = {}", id))
            .to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_account_service_with_empty_accounts() {
        let accs = AccountService::create();
        assert!(accs.accounts.lock().unwrap().is_empty());
    }

    #[test]
    fn should_add_and_get_account() {
        let accs = AccountService::create();
        let acc1 = Account::new();
        let acc2 = Account::new();

        accs.add(acc1.clone());
        accs.add(acc2.clone());

        assert_eq!(accs.accounts.lock().unwrap().len(), 2);
        assert_eq!(accs.get(acc1.id).id, acc1.id);
        assert_eq!(accs.get(acc2.id).id, acc2.id);
    }
}
