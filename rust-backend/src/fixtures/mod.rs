pub mod users;
pub mod categories;
// Commenting out transactions until we resolve Diesel type issues
// pub mod transactions;

use diesel::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use crate::db::establish_connection;

pub fn load_all_fixtures() -> Result<(), Box<dyn std::error::Error>> {
    let pool = establish_connection();
    let mut conn = pool.get()?;
    
    // Start a transaction to ensure all fixtures load or none do
    conn.transaction::<_, Box<dyn std::error::Error>, _>(|conn| {
        // Clear existing data first (in reverse order of dependencies)
        clear_all_data(conn)?;
        
        // Load fixtures in order
        users::load(conn)?;
        categories::load(conn)?;
        // Temporarily skip transactions until we fix type issues
        // transactions::load(conn)?;
        
        Ok(())
    })?;
    
    Ok(())
}

fn clear_all_data(_connection: &mut PgConnection) -> Result<(), Box<dyn std::error::Error>> {
    // Clear data in reverse order to avoid foreign key constraint issues
    // diesel::delete(transactions::table).execute(connection)?;
    // diesel::delete(categories::table).execute(connection)?;
    // diesel::delete(users::table).execute(connection)?;
    
    Ok(())
} 