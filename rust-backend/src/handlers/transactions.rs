use crate::error::AppError;
use crate::models::category::{default_categories};
use crate::models::transaction::{
    CreateTransactionDto, Transaction, TransactionFilters, TransactionItem,
    TransactionResponse, TransactionsListResponse, UpdateTransactionDto,
};
use actix_web::{web, HttpResponse};
use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;
use chrono::DateTime;
use crate::db::DbPool as RealDbPool;

// Type alias for the database pool
type DbPool = RealDbPool;

// Get all transactions for a user
pub async fn get_transactions(
    _pool: web::Data<DbPool>,
    filters: web::Query<TransactionFilters>,
) -> Result<HttpResponse, AppError> {
    // In a real application, you would query the database
    // For now, return mock data
    let mock_transactions = vec![
        TransactionResponse {
            id: Uuid::new_v4(),
            amount: 42.99,
            date: Utc::now(),
            merchant: "Grocery Store".to_string(),
            category: "Groceries".to_string(),
            notes: Some("Weekly shopping".to_string()),
            items: None,
            bill_image: None,
            created_at: Utc::now(),
        },
        TransactionResponse {
            id: Uuid::new_v4(),
            amount: 15.50,
            date: Utc::now(),
            merchant: "Coffee Shop".to_string(),
            category: "Dining".to_string(),
            notes: None,
            items: None,
            bill_image: None,
            created_at: Utc::now(),
        },
    ];

    let response = TransactionsListResponse {
        transactions: mock_transactions,
        total: 2,
        page: filters.page.unwrap_or(1),
        pages: 1,
    };

    Ok(HttpResponse::Ok().json(response))
}

// Get a single transaction by ID
pub async fn get_transaction(
    _pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let transaction_id = path.into_inner();
    let uuid = Uuid::parse_str(&transaction_id)
        .map_err(|_| AppError::BadRequest("Invalid transaction ID".to_string()))?;

    // In a real application, you would query the database
    // For now, return mock data
    let transaction = TransactionResponse {
        id: uuid,
        amount: 42.99,
        date: Utc::now(),
        merchant: "Grocery Store".to_string(),
        category: "Groceries".to_string(),
        notes: Some("Weekly shopping".to_string()),
        items: None,
        bill_image: None,
        created_at: Utc::now(),
    };

    Ok(HttpResponse::Ok().json(transaction))
}

// Create a new transaction
pub async fn create_transaction(
    _pool: web::Data<DbPool>,
    transaction_data: web::Json<CreateTransactionDto>,
) -> Result<HttpResponse, AppError> {
    // In a real application, we would insert the transaction into the database
    let transaction = TransactionResponse {
        id: Uuid::new_v4(),
        amount: transaction_data.amount,
        date: transaction_data.date,
        merchant: transaction_data.merchant.clone(),
        category: transaction_data.category.clone(),
        notes: transaction_data.notes.clone(),
        items: transaction_data.items.clone(),
        bill_image: transaction_data.bill_image.clone(),
        created_at: Utc::now(),
    };

    Ok(HttpResponse::Created().json(transaction))
}

// Update an existing transaction
pub async fn update_transaction(
    _pool: web::Data<DbPool>,
    path: web::Path<String>,
    transaction_data: web::Json<UpdateTransactionDto>,
) -> Result<HttpResponse, AppError> {
    let transaction_id = path.into_inner();
    let uuid = Uuid::parse_str(&transaction_id)
        .map_err(|_| AppError::BadRequest("Invalid transaction ID".to_string()))?;

    // In a real application, we would update the transaction in the database
    let transaction = TransactionResponse {
        id: uuid,
        amount: transaction_data.amount.unwrap_or(42.99),
        date: transaction_data.date.unwrap_or(Utc::now()),
        merchant: transaction_data.merchant.clone().unwrap_or("Grocery Store".to_string()),
        category: transaction_data.category.clone().unwrap_or("Groceries".to_string()),
        notes: transaction_data.notes.clone(),
        items: transaction_data.items.clone(),
        bill_image: None,
        created_at: Utc::now(),
    };

    Ok(HttpResponse::Ok().json(transaction))
}

// Delete a transaction
pub async fn delete_transaction(
    _pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let transaction_id = path.into_inner();
    let _uuid = Uuid::parse_str(&transaction_id)
        .map_err(|_| AppError::BadRequest("Invalid transaction ID".to_string()))?;

    // In a real application, we would delete the transaction from the database
    // For now, just return a success response
    Ok(HttpResponse::NoContent().finish())
}

// Get all categories
pub async fn get_categories(_pool: web::Data<DbPool>) -> Result<HttpResponse, AppError> {
    // In a real application, you would query the database
    // For now, return default categories
    let categories = default_categories();
    
    Ok(HttpResponse::Ok().json(categories))
}

#[derive(Deserialize)]
pub struct CreateTransactionRequest {
    pub amount: f64,
    pub date: DateTime<Utc>,
    pub merchant: String,
    pub category: String,
    pub notes: Option<String>,
    pub items: Option<Vec<TransactionItem>>,
    pub bill_image: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateTransactionRequest {
    pub amount: Option<f64>,
    pub date: Option<DateTime<Utc>>,
    pub merchant: Option<String>,
    pub category: Option<String>,
    pub notes: Option<String>,
    pub items: Option<Vec<TransactionItem>>,
}

pub async fn get_transactions_mock(_pool: DbPool) -> Result<HttpResponse, AppError> {
    // Mock data
    let transactions = vec![
        Transaction {
            id: Uuid::new_v4(),
            amount: 42.50,
            date: Utc::now(),
            merchant: "Grocery Store".to_string(),
            category: "Food".to_string(),
            notes: Some("Weekly groceries".to_string()),
            items: None,
            bill_image: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            user_id: Uuid::new_v4(),
            bill_id: None,
        },
        Transaction {
            id: Uuid::new_v4(),
            amount: 29.99,
            date: Utc::now(),
            merchant: "Bookstore".to_string(),
            category: "Entertainment".to_string(),
            notes: None,
            items: None,
            bill_image: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            user_id: Uuid::new_v4(),
            bill_id: None,
        },
    ];

    Ok(HttpResponse::Ok().json(transactions))
}

pub async fn get_transaction_mock(id: web::Path<Uuid>, _pool: DbPool) -> Result<HttpResponse, AppError> {
    // Mock data
    let transaction = Transaction {
        id: *id,
        amount: 42.50,
        date: Utc::now(),
        merchant: "Grocery Store".to_string(),
        category: "Food".to_string(),
        notes: Some("Weekly groceries".to_string()),
        items: None,
        bill_image: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        user_id: Uuid::new_v4(),
        bill_id: None,
    };

    Ok(HttpResponse::Ok().json(transaction))
}

pub async fn create_transaction_mock(
    _pool: web::Data<DbPool>,
    transaction_data: web::Json<CreateTransactionRequest>,
) -> Result<HttpResponse, AppError> {
    // In a real application, you would:
    // 1. Validate the data
    // 2. Store the transaction in the database
    // 3. Return the created transaction

    let transaction = TransactionResponse {
        id: Uuid::new_v4(),
        amount: transaction_data.amount,
        date: transaction_data.date,
        merchant: transaction_data.merchant.clone(),
        category: transaction_data.category.clone(),
        notes: transaction_data.notes.clone(),
        items: transaction_data.items.clone(),
        bill_image: transaction_data.bill_image.clone(),
        created_at: Utc::now(),
    };

    Ok(HttpResponse::Created().json(transaction))
}

pub async fn update_transaction_mock(
    _pool: web::Data<DbPool>,
    path: web::Path<(String,)>,
    transaction_data: web::Json<UpdateTransactionRequest>,
) -> Result<HttpResponse, AppError> {
    let transaction_id = path.into_inner().0;
    let uuid = Uuid::parse_str(&transaction_id)
        .map_err(|_| AppError::BadRequest("Invalid transaction ID".to_string()))?;

    // In a real application, you would:
    // 1. Find the transaction in the database
    // 2. Update it with the new data
    // 3. Return the updated transaction

    // For now, return mock data with updated fields
    let transaction = TransactionResponse {
        id: uuid,
        amount: transaction_data.amount.unwrap_or(42.99),
        date: transaction_data.date.unwrap_or(Utc::now()),
        merchant: transaction_data.merchant.clone().unwrap_or("Grocery Store".to_string()),
        category: transaction_data.category.clone().unwrap_or("Groceries".to_string()),
        notes: transaction_data.notes.clone(),
        items: transaction_data.items.clone(),
        bill_image: None,
        created_at: Utc::now(),
    };

    Ok(HttpResponse::Ok().json(transaction))
}

pub async fn delete_transaction_mock(
    _pool: web::Data<DbPool>,
    path: web::Path<(String,)>,
) -> Result<HttpResponse, AppError> {
    let transaction_id = path.into_inner().0;
    let _uuid = Uuid::parse_str(&transaction_id)
        .map_err(|_| AppError::BadRequest("Invalid transaction ID".to_string()))?;

    // In a real application, you would delete the transaction from the database
    // For now, just return a success response

    Ok(HttpResponse::NoContent().finish())
}

