use uuid::Uuid;

#[derive(Debug)]
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
    Some,
    Cl(u8),
}
