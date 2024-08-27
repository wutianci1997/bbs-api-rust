use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Category {
    pub id: Option<u32>,
    pub name: String,
    pub description: Option<String>,
}

impl Category {
    pub fn create(name: &str, description: Option<&str>) -> Category {
        let mut _description: Option<String> = None;
        if let Some(v) = description {
            _description = Some(v.to_string());
        }
        Category {
            id: None,
            name: name.to_string(),
            description: _description,
        }
    }
}
