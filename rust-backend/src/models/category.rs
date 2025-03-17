use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use crate::schema::categories;

// Add Queryable trait for database operations
#[derive(Queryable, Identifiable, Serialize, Deserialize, Debug)]
#[diesel(table_name = categories)]
pub struct DbCategory {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub user_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCategoryDto {
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCategoryDto {
    pub name: Option<String>,
    pub description: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = categories)]
pub struct NewCategory {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Default categories
pub fn default_categories() -> Vec<Category> {
    vec![
        Category {
            id: Uuid::new_v4(),
            name: "Groceries".to_string(),
            description: Some("Food and household items".to_string()),
            color: Some("#4CAF50".to_string()),
            icon: Some("shopping_cart".to_string()),
            user_id: None,
        },
        Category {
            id: Uuid::new_v4(),
            name: "Dining".to_string(),
            description: Some("Restaurants and take-out".to_string()),
            color: Some("#FF9800".to_string()),
            icon: Some("restaurant".to_string()),
            user_id: None,
        },
        Category {
            id: Uuid::new_v4(),
            name: "Transportation".to_string(),
            description: Some("Public transport, gas, etc.".to_string()),
            color: Some("#2196F3".to_string()),
            icon: Some("directions_car".to_string()),
            user_id: None,
        },
        Category {
            id: Uuid::new_v4(),
            name: "Entertainment".to_string(),
            description: Some("Movies, concerts, events".to_string()),
            color: Some("#9C27B0".to_string()),
            icon: Some("local_movies".to_string()),
            user_id: None,
        },
        Category {
            id: Uuid::new_v4(),
            name: "Utilities".to_string(),
            description: Some("Water, electricity, internet".to_string()),
            color: Some("#607D8B".to_string()),
            icon: Some("power".to_string()),
            user_id: None,
        },
        Category {
            id: Uuid::new_v4(),
            name: "Healthcare".to_string(),
            description: Some("Medical expenses".to_string()),
            color: Some("#F44336".to_string()),
            icon: Some("local_hospital".to_string()),
            user_id: None,
        },
        Category {
            id: Uuid::new_v4(),
            name: "Shopping".to_string(),
            description: Some("Clothing, electronics, etc.".to_string()),
            color: Some("#E91E63".to_string()),
            icon: Some("shopping_bag".to_string()),
            user_id: None,
        },
        Category {
            id: Uuid::new_v4(),
            name: "Other".to_string(),
            description: Some("Miscellaneous expenses".to_string()),
            color: Some("#9E9E9E".to_string()),
            icon: Some("more_horiz".to_string()),
            user_id: None,
        },
    ]
} 