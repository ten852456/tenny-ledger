use crate::error::AppError;
use crate::ocr::processor::{OcrProcessor, OcrResult};
use actix_multipart::Multipart;
use actix_web::HttpResponse;
use futures::{StreamExt, TryStreamExt};
use std::fs;
use std::io::Write;
use std::path::Path;
use uuid::Uuid;

pub async fn process_image(mut payload: Multipart) -> Result<HttpResponse, AppError> {
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
        let mut processor = OcrProcessor::new()?;
        let result = processor.process_image(&file_path)?;
        
        // Clean up temp file
        if let Err(e) = fs::remove_file(&file_path) {
            log::warn!("Failed to remove temp file: {}", e);
        }
        
        return Ok(HttpResponse::Ok().json(serialize_ocr_result(result)));
    }
    
    Err(AppError::BadRequest("No image file found in the request".to_string()))
}

// Helper function to convert OcrResult to a serializable response
fn serialize_ocr_result(result: OcrResult) -> serde_json::Value {
    serde_json::json!({
        "text": result.text,
        "extractedData": {
            "total": result.extracted_data.total,
            "date": result.extracted_data.date,
            "merchant": result.extracted_data.merchant,
            "items": result.extracted_data.items
        },
        "confidence": result.confidence,
        "processingTime": result.processing_time
    })
} 