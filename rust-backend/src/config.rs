use std::env;
use std::time::Duration;
use std::sync::OnceLock;
use dotenv::dotenv;
use log::{info, warn};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub jwt: JwtConfig,
}

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub cors_origin: String,
    pub upload_dir: String,
}

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub timeout: Duration,
}

#[derive(Debug, Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub expiry: Duration,
}

// Store configs in a static OnceLock for initialization once and immutable access
static SECRETS: OnceLock<Secrets> = OnceLock::new();

pub struct Secrets {
    google_vision_api_key: Option<String>,
    // Add other secrets here
}

// Public methods to access secrets without exposing them
impl Secrets {
    pub fn has_google_vision_api_key() -> bool {
        get_secrets().google_vision_api_key.is_some()
    }

    pub fn get_google_vision_api_key() -> Option<String> {
        get_secrets().google_vision_api_key.clone()
    }
}

// Public function to initialize app configuration
pub fn init_config() {
    // Load .env file if present
    if let Err(e) = dotenv() {
        warn!("Failed to load .env file: {}", e);
    }
    
    // Initialize secrets
    initialize_secrets();
    
    // Log which features are available, without exposing actual keys
    if Secrets::has_google_vision_api_key() {
        info!("Google Vision API is configured and available");
    } else {
        warn!("Google Vision API is not configured");
    }
}

// Private function to initialize secrets
fn initialize_secrets() {
    let _ = SECRETS.get_or_init(|| {
        Secrets {
            google_vision_api_key: read_secret("GOOGLE_VISION_API_KEY"),
            // Add other secrets here
        }
    });
}

// Helper function to get secrets from env vars in a secure way
fn read_secret(name: &str) -> Option<String> {
    match env::var(name) {
        Ok(value) if !value.trim().is_empty() => Some(value),
        _ => None,
    }
}

// Helper function to access the initialized secrets
fn get_secrets() -> &'static Secrets {
    SECRETS.get_or_init(|| {
        warn!("Secrets accessed before initialization");
        Secrets {
            google_vision_api_key: None,
            // Set other secrets to None
        }
    })
}

impl Config {
    pub fn from_env() -> Self {
        Config {
            server: ServerConfig {
                host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
                port: env::var("PORT")
                    .unwrap_or_else(|_| "8080".to_string())
                    .parse()
                    .unwrap_or(8080),
                cors_origin: env::var("CORS_ORIGIN")
                    .unwrap_or_else(|_| "http://localhost:3000".to_string()),
                upload_dir: env::var("UPLOAD_DIR").unwrap_or_else(|_| "./uploads".to_string()),
            },
            database: DatabaseConfig {
                url: env::var("DATABASE_URL")
                    .expect("DATABASE_URL environment variable must be set"),
                max_connections: env::var("DB_MAX_CONNECTIONS")
                    .unwrap_or_else(|_| "5".to_string())
                    .parse()
                    .unwrap_or(5),
                timeout: Duration::from_secs(
                    env::var("DB_TIMEOUT")
                        .unwrap_or_else(|_| "30".to_string())
                        .parse()
                        .unwrap_or(30),
                ),
            },
            jwt: JwtConfig {
                secret: env::var("JWT_SECRET")
                    .expect("JWT_SECRET environment variable must be set"),
                expiry: Duration::from_secs(
                    env::var("JWT_EXPIRY")
                        .unwrap_or_else(|_| "86400".to_string())
                        .parse()
                        .unwrap_or(86400),  // Default to 24 hours
                ),
            },
        }
    }
}

// Claims model for JWT
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Subject (user ID)
    pub exp: usize,  // Expiration time
    pub iat: usize,  // Issued at
}