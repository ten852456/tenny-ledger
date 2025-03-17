use crate::config::{Claims, Config};
use crate::db::DbPool;
use crate::error::AppError;
use crate::models::user::{AuthResponse, CreateUserDto, LoginDto, User, UserResponse};
use actix_web::{web, HttpResponse};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use uuid::Uuid;

pub async fn register(
    _pool: web::Data<DbPool>,
    user_data: web::Json<CreateUserDto>,
) -> Result<HttpResponse, AppError> {
    // In a real application, you would:
    // 1. Check if user with that email already exists
    // 2. Hash the password
    // 3. Store the user in the database
    
    // For now, we'll just create a mock user
    let user = User {
        id: Uuid::new_v4(),
        email: user_data.email.clone(),
        name: user_data.name.clone(),
        password_hash: "hashed_password".to_string(), // This should be properly hashed in production
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    
    // Generate JWT token
    let token = generate_token(&user.id)?;
    
    let response = AuthResponse {
        token,
        user: UserResponse {
            id: user.id,
            email: user.email,
            name: user.name,
            created_at: user.created_at,
        },
    };
    
    Ok(HttpResponse::Created().json(response))
}

pub async fn login(
    _pool: web::Data<DbPool>,
    login_data: web::Json<LoginDto>,
) -> Result<HttpResponse, AppError> {
    // In a real application, you would:
    // 1. Find the user by email
    // 2. Verify the password hash
    // 3. Generate and return a JWT token
    
    // For now, we'll just create a mock user and token
    let user = User {
        id: Uuid::new_v4(),
        email: login_data.email.clone(),
        name: "User Name".to_string(), // In a real app, this would come from the database
        password_hash: "hashed_password".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    
    // Generate JWT token
    let token = generate_token(&user.id)?;
    
    let response = AuthResponse {
        token,
        user: UserResponse {
            id: user.id,
            email: user.email,
            name: user.name,
            created_at: user.created_at,
        },
    };
    
    Ok(HttpResponse::Ok().json(response))
}

// JWT token generation
fn generate_token(user_id: &Uuid) -> Result<String, AppError> {
    let config = Config::from_env();
    let jwt_secret = config.jwt.secret;
    
    let expiration = Utc::now()
        .checked_add_signed(Duration::seconds(
            config.jwt.expiry.as_secs() as i64,
        ))
        .expect("Valid timestamp")
        .timestamp() as usize;
        
    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration,
        iat: Utc::now().timestamp() as usize,
    };
    
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .map_err(|e| AppError::AuthError(format!("Failed to generate token: {}", e)))
} 