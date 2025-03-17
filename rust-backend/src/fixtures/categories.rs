use diesel::PgConnection;
use uuid::Uuid;
use crate::models::category::{Category, NewCategory};
use diesel::prelude::*;
use chrono::Utc;

pub fn load(connection: &mut PgConnection) -> Result<(), Box<dyn std::error::Error>> {
    // Get admin user
    use crate::schema::users;
    let admin_id: Uuid = users::table
        .select(users::id)
        .filter(users::email.eq("admin@example.com"))
        .first(connection)?;
    
    // Create a system user ID for system categories
    let system_user_id = admin_id; // Using admin as fallback for system categories
    
    // Default categories
    let categories = vec![
        NewCategory {
            id: Uuid::new_v4(),
            name: "Groceries".to_string(),
            description: Some("Food and household items".to_string()),
            color: Some("#4CAF50".to_string()),
            icon: Some("shopping_cart".to_string()),
            user_id: system_user_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        },
        NewCategory {
            id: Uuid::new_v4(),
            name: "Dining".to_string(),
            description: Some("Restaurants and take-out".to_string()),
            color: Some("#FF9800".to_string()),
            icon: Some("restaurant".to_string()),
            user_id: system_user_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        },
        NewCategory {
            id: Uuid::new_v4(),
            name: "Transportation".to_string(),
            description: Some("Public transport, gas, etc.".to_string()),
            color: Some("#2196F3".to_string()),
            icon: Some("directions_car".to_string()),
            user_id: system_user_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        },
        NewCategory {
            id: Uuid::new_v4(),
            name: "Entertainment".to_string(),
            description: Some("Movies, concerts, events".to_string()),
            color: Some("#9C27B0".to_string()),
            icon: Some("local_movies".to_string()),
            user_id: system_user_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        },
        NewCategory {
            id: Uuid::new_v4(),
            name: "Utilities".to_string(),
            description: Some("Water, electricity, internet".to_string()),
            color: Some("#607D8B".to_string()),
            icon: Some("power".to_string()),
            user_id: system_user_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        },
        NewCategory {
            id: Uuid::new_v4(),
            name: "Healthcare".to_string(),
            description: Some("Medical expenses".to_string()),
            color: Some("#F44336".to_string()),
            icon: Some("local_hospital".to_string()),
            user_id: system_user_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        },
        NewCategory {
            id: Uuid::new_v4(),
            name: "Shopping".to_string(),
            description: Some("Clothing, electronics, etc.".to_string()),
            color: Some("#E91E63".to_string()),
            icon: Some("shopping_bag".to_string()),
            user_id: system_user_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        },
        NewCategory {
            id: Uuid::new_v4(),
            name: "Other".to_string(),
            description: Some("Miscellaneous expenses".to_string()),
            color: Some("#9E9E9E".to_string()),
            icon: Some("more_horiz".to_string()),
            user_id: system_user_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        },
    ];
    
    // Insert categories
    use crate::schema::categories;
    diesel::insert_into(categories::table)
        .values(&categories)
        .on_conflict_do_nothing()
        .execute(connection)?;
    
    println!("Inserted {} categories", categories.len());
    Ok(())
} 