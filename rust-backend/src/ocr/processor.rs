use crate::error::AppError;
use image::{DynamicImage, GenericImageView};
use leptess::{LepTess, Variable};
use std::path::Path;
use std::time::Instant;
use regex::Regex;

pub struct OcrProcessor {
    tesseract: LepTess,
}

pub struct OcrResult {
    pub text: String,
    pub extracted_data: ExtractedData,
    pub confidence: f32,
    pub processing_time: f64,
}

#[derive(serde::Serialize, Debug)]
pub struct ExtractedData {
    pub total: Option<f64>,
    pub date: Option<String>,
    pub merchant: Option<String>,
    pub items: Vec<ItemData>,
}

#[derive(serde::Serialize, Debug)]
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
        
        // Load image
        let img = image::open(image_path)
            .map_err(|e| AppError::ImageError(format!("Failed to open image: {}", e)))?;
            
        // Preprocess image for better OCR results
        let processed_img = self.preprocess_image(img);
        
        // Convert to bytes
        let mut buffer = Vec::new();
        processed_img.write_to(&mut buffer, image::ImageOutputFormat::Png)
            .map_err(|e| AppError::ImageError(format!("Failed to write image: {}", e)))?;
            
        // Set image data for OCR
        self.tesseract.set_image_from_mem(&buffer)
            .map_err(|e| AppError::OcrError(format!("Failed to set image: {}", e)))?;
            
        // Set parameters for receipt OCR
        self.tesseract.set_variable(Variable::TesseditPageSegMode, "6") // Assume single uniform block of text
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
        let item_regex = Regex::new(r"([A-Za-z\s]+)[\s]+(\d+(?:\.\d{2})?)").ok()?;
        
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
        
        items
    }
} 