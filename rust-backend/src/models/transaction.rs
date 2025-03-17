use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Uuid,
    pub user_id: Uuid,
    pub amount: f64,
    pub date: DateTime<Utc>,
    pub merchant: String,
    pub category: String,
    pub notes: Option<String>,
    pub bill_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTransactionDto {
    pub amount: f64,
    pub date: DateTime<Utc>,
    pub merchant: String,
    pub category: String,
    pub notes: Option<String>,
    pub bill_image: Option<String>,
    pub items: Option<Vec<TransactionItem>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTransactionDto {
    pub amount: Option<f64>,
    pub date: Option<DateTime<Utc>>,
    pub merchant: Option<String>,
    pub category: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionItem {
    pub name: String,
    pub price: Option<f64>,
    pub quantity: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionResponse {
    pub id: Uuid,
    pub amount: f64,
    pub date: DateTime<Utc>,
    pub merchant: String,
    pub category: String,
    pub notes: Option<String>,
    pub items: Option<Vec<TransactionItem>>,
    pub bill_image: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsListResponse {
    pub transactions: Vec<TransactionResponse>,
    pub total: u64,
    pub page: u64,
    pub pages: u64,
}

// Categories for transactions
#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
}

// For filtering transactions
#[derive(Debug, Deserialize)]
pub struct TransactionFilters {
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub min_amount: Option<f64>,
    pub max_amount: Option<f64>,
    pub merchant: Option<String>,
    pub category: Option<String>,
    pub search: Option<String>,
    pub page: Option<u64>,
    pub limit: Option<u64>,
} 