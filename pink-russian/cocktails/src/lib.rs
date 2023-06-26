use std::collections::HashMap;

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

#[derive(Clone)]
pub struct AccountService {
    accounts: HashMap<Uuid, Account>,
}

impl AccountService {
    pub fn create() -> Self {
        AccountService {
            accounts: HashMap::new(),
        }
    }

    pub fn add(&mut self, account: Account) {
        self.accounts.insert(account.id, account);
    }

    pub fn get(&self, id: Uuid) -> &Account {
        self.accounts
            .get(&id)
            .expect(&format!("Not found account by id = {}", id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_account_service_with_empty_accounts() {
        let accs = AccountService::create();
        assert!(accs.accounts.is_empty());
    }

    #[test]
    fn should_add_and_get_account() {
        let mut accs = AccountService::create();
        let acc1 = Account::new();
        let acc2 = Account::new();

        accs.add(acc1.clone());
        accs.add(acc2.clone());

        assert_eq!(accs.accounts.len(), 2);
        assert_eq!(accs.get(acc1.id).id, acc1.id);
        assert_eq!(accs.get(acc2.id).id, acc2.id);
    }
}
