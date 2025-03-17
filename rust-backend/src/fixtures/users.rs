use diesel::PgConnection;
use uuid::Uuid;
use bcrypt::{hash, DEFAULT_COST};
use crate::models::user::{User, NewUser};
use diesel::prelude::*;
use chrono::Utc;

pub fn load(connection: &mut PgConnection) -> Result<(), Box<dyn std::error::Error>> {
    // Create admin user
    let admin_id = Uuid::new_v4();
    let admin_user = NewUser {
        id: admin_id,
        name: "Admin User".to_string(),
        email: "admin@example.com".to_string(),
        password_hash: hash("admin123", DEFAULT_COST)?,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    
    // Create regular users
    let user_id = Uuid::new_v4();
    let regular_user = NewUser {
        id: user_id,
        name: "Regular User".to_string(),
        email: "user@example.com".to_string(),
        password_hash: hash("user123", DEFAULT_COST)?,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    
    // Insert users into database
    use crate::schema::users;
    diesel::insert_into(users::table)
        .values(&vec![admin_user, regular_user])
        .on_conflict_do_nothing()
        .execute(connection)?;
    
    println!("Inserted 2 users");
    Ok(())
} 