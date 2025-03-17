use crate::error::AppError;
use crate::ocr::processor::{OcrProcessor, OcrResult};
use actix_multipart::Multipart;
use actix_web::{web, HttpResponse};
use futures::{StreamExt, TryStreamExt};
use std::fs;
use std::io::Write;
use std::path::Path;
use uuid::Uuid;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct OcrEngineQuery {
    engine: Option<String>, // "tesseract", "google", or "hybrid" (default)
}

// Existing function that uses hybrid processing by default
pub async fn process_image(mut payload: Multipart) -> Result<HttpResponse, AppError> {
    // Use hybrid as the default engine
    let engine = "hybrid".to_string();
    process_image_with_engine_internal(payload, engine).await
}

// New function that allows specifying the OCR engine via query parameter
pub async fn process_image_with_engine(
    mut payload: Multipart,
    query: web::Query<OcrEngineQuery>,
) -> Result<HttpResponse, AppError> {
    // Extract engine preference from query params or use hybrid by default
    let engine = query.engine.clone().unwrap_or_else(|| "hybrid".to_string());
    process_image_with_engine_internal(payload, engine).await
}

// Internal function that handles the actual processing with the specified engine
async fn process_image_with_engine_internal(
    mut payload: Multipart,
    engine: String,
) -> Result<HttpResponse, AppError> {
    // Create temp directory if it doesn't exist
    let upload_dir = Path::new("./temp");
    if !upload_dir.exists() {
        fs::create_dir_all(upload_dir)
            .map_err(|e| AppError::IoError(e))?;
    }
    
    // Process multipart form data
    let mut temp_file_path = None;
    
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_disposition = field.content_disposition();
        
        if let Some(filename) = content_disposition.get_filename() {
            // Generate a unique filename
            let file_id = Uuid::new_v4();
            let file_ext = Path::new(filename)
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("unknown");
                
            let file_path = upload_dir.join(format!("{}.{}", file_id, file_ext));
            let mut file = fs::File::create(&file_path)
                .map_err(|e| AppError::IoError(e))?;
                
            // Write file to disk
            while let Some(chunk) = field.next().await {
                let data = chunk
                    .map_err(|e| AppError::BadRequest(format!("Error reading multipart data: {}", e)))?;
                file.write_all(&data)
                    .map_err(|e| AppError::IoError(e))?;
            }
            
            temp_file_path = Some(file_path);
        }
    }
    
    // Process image with OCR
    if let Some(file_path) = temp_file_path {
        let mut processor = OcrProcessor::new();
        
        // Choose OCR engine based on the parameter
        let result = match engine.as_str() {
            "tesseract" => {
                let file_bytes = fs::read(&file_path)
                    .map_err(|e| AppError::IoError(e))?;
                let extracted_data = processor.process_image(&file_bytes)?;
                OcrResult {
                    text: "".to_string(),
                    extracted_data: extracted_data.clone(),
                    confidence: extracted_data.confidence,
                    processing_time: 0.0,
                }
            },
            "google" => processor.process_with_google_vision(&file_path).await?,
            _ => processor.process_image_hybrid(&file_path).await?, // Default to hybrid
        };
        
        // Clean up temp file
        if let Err(e) = fs::remove_file(&file_path) {
            log::warn!("Failed to remove temp file: {}", e);
        }
        
        return Ok(HttpResponse::Ok().json(serialize_ocr_result(result, engine)));
    }
    
    Err(AppError::BadRequest("No image file found in the request".to_string()))
}

// Helper function to convert OcrResult to a serializable response
fn serialize_ocr_result(result: OcrResult, engine: String) -> serde_json::Value {
    let source = match engine.as_str() {
        "tesseract" => "Tesseract OCR",
        "google" => "Google Vision API",
        _ => if result.confidence <= 0.7 { "Google Vision API" } else { "Tesseract OCR" },
    };
    
    serde_json::json!({
        "text": result.text,
        "extractedData": {
            "total": result.extracted_data.total,
            "date": result.extracted_data.date,
            "merchant": result.extracted_data.merchant,
            "items": result.extracted_data.items
        },
        "confidence": result.confidence,
        "processingTime": result.processing_time,
        "source": source,
        "engine": engine
    })
} 