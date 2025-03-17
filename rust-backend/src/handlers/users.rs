use crate::db::DbPool;
use crate::error::AppError;
use crate::models::user::{UpdateUserDto, UserResponse};
use actix_web::{web, HttpResponse};
use chrono::Utc;
use uuid::Uuid;

// Get current user profile
pub async fn get_profile(
    _pool: web::Data<DbPool>,
    // In a real app, you would extract user_id from request
) -> Result<HttpResponse, AppError> {
    // Mock user data (in a real app, this would come from the database)
    let user = UserResponse {
        id: Uuid::new_v4(),
        email: "user@example.com".to_string(),
        name: "Test User".to_string(),
        created_at: Utc::now(),
    };
    
    Ok(HttpResponse::Ok().json(user))
}

// Update user profile
pub async fn update_profile(
    _pool: web::Data<DbPool>,
    user_data: web::Json<UpdateUserDto>,
    // In a real app, you would extract user_id from request
) -> Result<HttpResponse, AppError> {
    // Mock user data with updates from request
    let user = UserResponse {
        id: Uuid::new_v4(),
        email: user_data.email.clone().unwrap_or_else(|| "user@example.com".to_string()),
        name: user_data.name.clone().unwrap_or_else(|| "Test User".to_string()),
        created_at: Utc::now(),
    };
    
    Ok(HttpResponse::Ok().json(user))
} 