use diesel::PgConnection;
use uuid::Uuid;
use crate::models::transaction::{NewTransaction, TransactionItem};
use crate::models::category::DbCategory;
use diesel::prelude::*;
use chrono::{Utc, Duration};
use serde_json::json;
use rand::Rng;

pub fn load(connection: &mut PgConnection) -> Result<(), Box<dyn std::error::Error>> {
    // Get users
    use crate::schema::users;
    let users: Vec<(Uuid, String)> = users::table
        .select((users::id, users::email))
        .load(connection)?;
    
    // Get categories
    use crate::schema::categories;
    let all_categories: Vec<DbCategory> = categories::table.load(connection)?;
    
    let mut transactions = Vec::new();
    let mut rng = rand::thread_rng();
    
    // Create 30 random transactions over the last 3 months
    for user in &users {
        // Generate between 5-15 transactions per user
        let num_transactions = rng.gen_range(5..=15);
        
        for _i in 0..num_transactions {
            // Generate a random date within the last 90 days
            let days_ago = rng.gen_range(0..90);
            let now = Utc::now();
            let transaction_date = now - Duration::days(days_ago);
            
            // Select a random category
            let category_index = rng.gen_range(0..all_categories.len());
            let category = &all_categories[category_index];
            
            // Generate a random transaction amount between $5 and $200
            let amount = (rng.gen_range(500..20000) as f64) / 100.0;
            let amount_string = amount.to_string(); // Convert to string
            
            // Generate 1-4 items for the transaction
            let num_items = rng.gen_range(1..=4);
            let mut items = Vec::new();
            
            for _j in 0..num_items {
                let item_price = amount / (num_items as f64) * (0.8 + rng.gen_range(0.0..0.4));
                let item_quantity = rng.gen_range(1..=3);
                
                items.push(TransactionItem {
                    name: format!("Item {}", _j + 1),
                    price: Some(item_price),
                    quantity: Some(item_quantity),
                });
            }
            
            // List of merchants
            let merchants = vec![
                "Grocery Store", "Coffee Shop", "Restaurant", "Gas Station", 
                "Department Store", "Online Shop", "Pharmacy", "Hardware Store"
            ];
            let merchant_index = rng.gen_range(0..merchants.len());
            
            // Create transaction
            let transaction = NewTransaction {
                id: Uuid::new_v4(),
                amount: amount_string,
                date: transaction_date,
                merchant: merchants[merchant_index].to_string(),
                category_id: Some(category.id),
                notes: Some(format!("Sample transaction {}", _i + 1)),
                items: Some(json!(items)),
                image_path: None,
                user_id: user.0,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };
            
            transactions.push(transaction);
        }
    }
    
    // Insert transactions
    use crate::schema::transactions;
    diesel::insert_into(transactions::table)
        .values(&transactions)
        .on_conflict_do_nothing()
        .execute(connection)?;
    
    println!("Inserted {} transactions", transactions.len());
    Ok(())
} 