use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::ocr::processor::ExtractedData;

#[derive(Debug, Serialize, Deserialize)]
pub struct Bill {
    pub id: Uuid,
    pub user_id: Uuid,
    pub file_path: String,
    pub file_name: String,
    pub file_size: i64,
    pub file_type: String,
    pub ocr_text: Option<String>,
    pub ocr_confidence: Option<f32>,
    pub extracted_data: Option<ExtractedData>,
    pub transaction_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BillResponse {
    pub id: Uuid,
    pub file_name: String,
    pub file_type: String,
    pub ocr_confidence: Option<f32>,
    pub extracted_data: Option<ExtractedData>,
    pub transaction_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBillDto {
    pub file_name: String,
    pub file_type: String,
    pub file_size: i64,
    pub file_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateBillDto {
    pub ocr_text: Option<String>,
    pub ocr_confidence: Option<f32>,
    pub extracted_data: Option<ExtractedData>,
    pub transaction_id: Option<Uuid>,
} 