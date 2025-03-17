use crate::error::AppError;
use image::DynamicImage;
use leptess::{LepTess, Variable};
use std::path::Path;
use std::time::Instant;
use regex::Regex;
use serde;
use std::fs;
use base64;
use reqwest;
use std::env;

pub struct OcrProcessor {
    tesseract: LepTess,
}

pub struct OcrResult {
    pub text: String,
    pub extracted_data: ExtractedData,
    pub confidence: f32,
    pub processing_time: f64,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ExtractedData {
    pub total: Option<f64>,
    pub date: Option<String>,
    pub merchant: Option<String>,
    pub items: Vec<ItemData>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ItemData {
    pub name: String,
    pub price: Option<f64>,
    pub quantity: Option<u32>,
}

impl OcrProcessor {
    pub fn new() -> Result<Self, AppError> {
        let tesseract = LepTess::new(None, "eng")
            .map_err(|e| AppError::OcrError(format!("Failed to initialize Tesseract: {}", e)))?;
            
        Ok(OcrProcessor { tesseract })
    }
    
    pub fn process_image(&mut self, image_path: &Path) -> Result<OcrResult, AppError> {
        let start = Instant::now();
        
        // Read the file into memory
        let img_bytes = fs::read(image_path)
            .map_err(|e| AppError::IoError(e))?;
            
        // Set image data for OCR
        self.tesseract.set_image_from_mem(&img_bytes)
            .map_err(|e| AppError::OcrError(format!("Failed to set image: {}", e)))?;
            
        // Set parameters for receipt OCR
        self.tesseract.set_variable(Variable::TesseditPagesegMode, "6") // Assume single uniform block of text
            .map_err(|e| AppError::OcrError(format!("Failed to set page segmentation mode: {}", e)))?;
            
        // Get OCR text
        let text = self.tesseract.get_utf8_text()
            .map_err(|e| AppError::OcrError(format!("Failed to recognize text: {}", e)))?;
            
        // Get confidence
        let confidence = self.tesseract.mean_text_conf() as f32 / 100.0;
        
        // Extract structured data
        let extracted_data = self.extract_data(&text)?;
        
        let duration = start.elapsed();
        let processing_time = duration.as_secs_f64();
        
        Ok(OcrResult {
            text,
            extracted_data,
            confidence,
            processing_time,
        })
    }
    
    fn preprocess_image(&self, img: DynamicImage) -> DynamicImage {
        // Simple preprocessing - convert to grayscale and increase contrast
        // In a real application, we would apply more sophisticated preprocessing
        let grayscale = img.grayscale();
        grayscale
    }
    
    fn extract_data(&self, text: &str) -> Result<ExtractedData, AppError> {
        // Extract total amount
        let total = self.extract_total(text);
        
        // Extract date
        let date = self.extract_date(text);
        
        // Extract merchant
        let merchant = self.extract_merchant(text);
        
        // Extract items
        let items = self.extract_items(text);
        
        Ok(ExtractedData {
            total,
            date,
            merchant,
            items,
        })
    }
    
    fn extract_total(&self, text: &str) -> Option<f64> {
        // Pattern for matching money amounts like $42.99, 42.99, or TOTAL: $42.99
        let total_regex = Regex::new(r"(?i)(total|amount|sum)[:\s]*[$]?(\d+\.\d{2})").ok()?;
        
        if let Some(cap) = total_regex.captures(text) {
            if let Some(amount_str) = cap.get(2) {
                if let Ok(amount) = amount_str.as_str().parse::<f64>() {
                    return Some(amount);
                }
            }
        }
        
        None
    }
    
    fn extract_date(&self, text: &str) -> Option<String> {
        // Pattern for common date formats
        let date_regex = Regex::new(r"(\d{1,2})[/\-\.](\d{1,2})[/\-\.](\d{2,4})").ok()?;
        
        if let Some(cap) = date_regex.captures(text) {
            return Some(cap[0].to_string());
        }
        
        None
    }
    
    fn extract_merchant(&self, text: &str) -> Option<String> {
        // Many receipts have the merchant name at the beginning
        let lines: Vec<&str> = text.lines().collect();
        
        // Often the first non-empty line is the merchant name
        for line in lines.iter().take(5) {
            let trimmed = line.trim();
            if !trimmed.is_empty() && trimmed.len() > 3 {
                return Some(trimmed.to_string());
            }
        }
        
        None
    }
    
    fn extract_items(&self, text: &str) -> Vec<ItemData> {
        let mut items = Vec::new();
        
        // This is a simplified approach - in reality, we'd need more sophisticated parsing
        // based on the specific receipt format, which varies greatly
        if let Ok(item_regex) = Regex::new(r"([A-Za-z\s]+)[\s]+(\d+(?:\.\d{2})?)") {
            for line in text.lines() {
                if let Some(cap) = item_regex.captures(line) {
                    if cap.len() >= 3 {
                        let name = cap[1].trim().to_string();
                        let price = cap[2].parse::<f64>().ok();
                        
                        if name.len() > 2 && !name.to_lowercase().contains("total") {
                            items.push(ItemData {
                                name,
                                price,
                                quantity: Some(1), // Default quantity
                            });
                        }
                    }
                }
            }
        }
        
        items
    }
    
    pub async fn process_with_google_vision(&self, image_path: &Path) -> Result<OcrResult, AppError> {
        let start = Instant::now();
        
        // Read image file to bytes
        let img_bytes = fs::read(image_path).map_err(|e| AppError::IoError(e))?;
        
        // Set up the Vision API request
        let client = reqwest::Client::new();
        let api_key = env::var("GOOGLE_VISION_API_KEY")
            .map_err(|_| AppError::ConfigError("GOOGLE_VISION_API_KEY not set".to_string()))?;
        
        let base64_image = base64::encode(&img_bytes);
        
        let request_body = serde_json::json!({
            "requests": [{
                "image": {
                    "content": base64_image
                },
                "features": [{
                    "type": "DOCUMENT_TEXT_DETECTION"
                }]
            }]
        });
        
        // Make the API request
        let response = client
            .post(format!("https://vision.googleapis.com/v1/images:annotate?key={}", api_key))
            .json(&request_body)
            .send()
            .await
            .map_err(|e| AppError::ExternalApiError(format!("Google Vision API request failed: {}", e)))?;
        
        let vision_result: serde_json::Value = response
            .json()
            .await
            .map_err(|e| AppError::ExternalApiError(format!("Failed to parse API response: {}", e)))?;
        
        // Extract text
        let text = vision_result["responses"][0]["fullTextAnnotation"]["text"]
            .as_str()
            .unwrap_or("")
            .to_string();
        
        // Use your existing extraction logic
        let extracted_data = self.extract_data(&text)?;
        
        let duration = start.elapsed();
        let processing_time = duration.as_secs_f64();
        
        Ok(OcrResult {
            text,
            extracted_data,
            confidence: 0.9, // Google doesn't provide a direct confidence score for the whole document
            processing_time,
        })
    }
    
    // Add a hybrid processing method that uses Tesseract first, then Google Vision if confidence is low
    pub async fn process_image_hybrid(&mut self, image_path: &Path) -> Result<OcrResult, AppError> {
        // First try with Tesseract
        let tesseract_result = self.process_image(image_path)?;
        
        // If confidence is high and we extracted what we need, return the result
        if tesseract_result.confidence > 0.7 && 
           tesseract_result.extracted_data.total.is_some() && 
           tesseract_result.extracted_data.merchant.is_some() {
            return Ok(tesseract_result);
        }
        
        // Otherwise, fallback to Vision API
        log::info!("Tesseract confidence too low ({}), falling back to Google Vision API", tesseract_result.confidence);
        self.process_with_google_vision(image_path).await
    }
} 