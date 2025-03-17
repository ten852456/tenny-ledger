use std::path::Path;
use std::fs;
use std::io::Cursor;
use regex::Regex;
use image::{DynamicImage, GenericImageView};
use image::imageops;
use imageproc::contrast;
use leptess::LepTess;
use serde::{Serialize, Deserialize};
use base64;
use log;
use crate::error::AppError;

pub struct OcrProcessor {
    bottom_crop: Option<Vec<u8>>,
    top_crop: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcrResult {
    pub text: String,
    pub extracted_data: ExtractedData,
    pub confidence: f32,
    pub processing_time: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemData {
    pub name: String,
    pub price: Option<f64>,
    pub quantity: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedData {
    pub total: Option<f64>,
    pub date: Option<String>,
    pub merchant: Option<String>,
    pub items: Vec<ItemData>,
    pub confidence: f32,
    pub ocr_source: String,
}

impl Default for ExtractedData {
    fn default() -> Self {
        Self {
            total: None,
            date: None,
            merchant: None,
            items: Vec::new(),
            confidence: 0.0,
            ocr_source: "tesseract".to_string(),
        }
    }
}

impl OcrProcessor {
    pub fn new() -> Self {
        Self {
            bottom_crop: None,
            top_crop: None,
        }
    }
    
    pub fn process_image(&mut self, image_data: &[u8]) -> Result<ExtractedData, AppError> {
        // Process the image
        let (processed_image, bottom_crop, top_crop) = self.preprocess_image(image_data)?;
        
        // Store the crops for later use
        self.bottom_crop = Some(bottom_crop);
        self.top_crop = Some(top_crop);
        
        // Extract text using Tesseract
        let mut tesseract = LepTess::new(None, "eng+tha")
            .map_err(|e| AppError::OcrError(format!("Failed to initialize Tesseract: {}", e)))?;
        tesseract.set_image_from_mem(&processed_image)
            .map_err(|e| AppError::OcrError(format!("Failed to set image: {}", e)))?;
        let text = tesseract.get_utf8_text()
            .map_err(|e| AppError::OcrError(format!("Failed to get text: {}", e)))?;
        
        // Extract data from the text
        let total = self.extract_total(&text);
        let date = self.extract_date(&text);
        let merchant = self.extract_merchant(&text);
        let items = self.extract_items(&text);
        
        Ok(ExtractedData {
            total,
            date,
            merchant,
            items,
            confidence: 0.7, // Default confidence for Tesseract
            ocr_source: "tesseract".to_string(),
        })
    }
    
    fn preprocess_image(&self, image_data: &[u8]) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>), AppError> {
        // Load the image from bytes
        let img = image::load_from_memory(image_data)
            .map_err(|e| AppError::OcrError(format!("Failed to load image: {}", e)))?;
        
        // Get dimensions
        let (width, height) = img.dimensions();
        
        // Create grayscale image with enhanced contrast for better OCR
        let gray_img = img.to_luma8();
        let contrast_img = contrast::stretch_contrast(&gray_img, 50, 200);
        
        // Create crops for specific parts of the receipt
        
        // Bottom crop (for total amount) - bottom 20% of the image
        let bottom_height = (height as f32 * 0.2) as u32;
        let bottom_y = height.saturating_sub(bottom_height);
        let bottom_crop = imageops::crop_imm(&contrast_img, 0, bottom_y, width, bottom_height).to_image();
        
        // Top crop (for merchant name) - top 30% of the image
        let top_height = (height as f32 * 0.3) as u32;
        let top_crop = imageops::crop_imm(&contrast_img, 0, 0, width, top_height).to_image();
        
        // Convert images to byte arrays
        let mut full_buffer = Vec::new();
        let mut bottom_buffer = Vec::new();
        let mut top_buffer = Vec::new();
        
        // Write full image to buffer
        let mut cursor = Cursor::new(&mut full_buffer);
        contrast_img.write_to(&mut cursor, image::ImageFormat::Png)
            .map_err(|e| AppError::OcrError(format!("Failed to encode full image: {}", e)))?;
        
        // Write bottom crop to buffer
        let mut cursor = Cursor::new(&mut bottom_buffer);
        bottom_crop.write_to(&mut cursor, image::ImageFormat::Png)
            .map_err(|e| AppError::OcrError(format!("Failed to encode bottom crop: {}", e)))?;
        
        // Write top crop to buffer
        let mut cursor = Cursor::new(&mut top_buffer);
        top_crop.write_to(&mut cursor, image::ImageFormat::Png)
            .map_err(|e| AppError::OcrError(format!("Failed to encode top crop: {}", e)))?;
        
        Ok((full_buffer, bottom_buffer, top_buffer))
    }
    
    fn process_image_parts(&self, img: DynamicImage) -> (DynamicImage, Vec<u8>, Vec<u8>) {
        // Get image dimensions
        let (width, height) = img.dimensions();
        
        // Create enhanced version of the full image
        let mut enhanced = img.to_luma8();
        for pixel in enhanced.iter_mut() {
            // Increase contrast
            if *pixel > 127 {
                *pixel = 255.min(*pixel + 30);
            } else {
                *pixel = 0.max(*pixel - 30);
            }
        }
        let enhanced_img = DynamicImage::ImageLuma8(enhanced);
        
        // Create cropped versions for specific processing
        // Bottom part (for totals)
        let bottom_height = height / 3;
        let bottom_img = img.crop_imm(0, height - bottom_height, width, bottom_height);
        
        // Top part (for merchant info)
        let top_height = height / 4;
        let top_img = img.crop_imm(0, 0, width, top_height);
        
        // Convert cropped images to bytes
        let mut bottom_buffer = Vec::new();
        let mut bottom_cursor = std::io::Cursor::new(&mut bottom_buffer);
        bottom_img.write_to(&mut bottom_cursor, image::ImageFormat::Jpeg)
            .expect("Failed to encode bottom crop");
            
        let mut top_buffer = Vec::new();
        let mut top_cursor = std::io::Cursor::new(&mut top_buffer);
        top_img.write_to(&mut top_cursor, image::ImageFormat::Jpeg)
            .expect("Failed to encode top crop");
        
        (enhanced_img, bottom_buffer, top_buffer)
    }
    
    fn extract_data(&self, text: &str) -> Result<ExtractedData, AppError> {
        let total = self.extract_total(text);
        let date = self.extract_date(text);
        let merchant = self.extract_merchant(text);
        let items = self.extract_items(text);
        
        Ok(ExtractedData {
            total,
            date,
            merchant,
            items,
            confidence: 0.0,
            ocr_source: "tesseract".to_string(),
        })
    }
    
    fn extract_total(&self, text: &str) -> Option<f64> {
        // First try looking at the bottom crop where totals often appear
        if let Some(bottom_data) = &self.bottom_crop {
            // Initialize a new tesseract instance specifically for the bottom crop
            if let Ok(mut crop_tesseract) = LepTess::new(None, "eng+tha") {
                if crop_tesseract.set_image_from_mem(&bottom_data).is_ok() {
                    if let Ok(crop_text) = crop_tesseract.get_utf8_text() {
                        // Try to find total in the bottom crop
                        if let Some(total) = self.find_total_in_text(&crop_text) {
                            return Some(total);
                        }
                    }
                }
            }
        }
        
        // Fallback to full text
        self.find_total_in_text(text)
    }
    
    fn find_total_in_text(&self, text: &str) -> Option<f64> {
        // Look for total amount patterns in Thai receipts
        let total_indicators = [
            "total", "รวม", "ทั้งหมด", "รวมทั้งสิ้น", "รวมเงิน", "จำนวนเงิน", "ยอดรวม", "ยอดเงิน"
        ];
        
        // Thai currency symbols and formats
        let currency_patterns = [
            r"(?:฿|บาท|บ\.|THB)\s*(\d+(?:[,.]\d{1,3})*(?:\.\d{1,2})?)",  // ฿100.00, 100.00บาท
            r"(\d+(?:[,.]\d{1,3})*(?:\.\d{1,2})?)\s*(?:฿|บาท|บ\.)",      // 100.00฿, 100.00 บาท
            r"(?:total|รวม|ทั้งหมด|รวมทั้งสิ้น|รวมเงิน|จำนวนเงิน|ยอดรวม|ยอดเงิน)[^\d]*(\d+(?:[,.]\d{1,3})*(?:\.\d{1,2})?)" // total: 100.00
        ];
        
        // First look for lines with total indicators
        for line in text.lines() {
            let line_lower = line.to_lowercase();
            
            // Check if line contains total indicators
            let has_indicator = total_indicators.iter()
                .any(|&indicator| line_lower.contains(&indicator.to_lowercase()));
                
            if has_indicator {
                // Try each currency pattern
                for pattern in &currency_patterns {
                    if let Ok(regex) = Regex::new(pattern) {
                        if let Some(cap) = regex.captures(&line) {
                            if let Some(amount_str) = cap.get(1) {
                                // Clean up the amount string (remove commas, spaces)
                                let clean_amount = amount_str.as_str()
                                    .replace(",", "")
                                    .replace(" ", "");
                                
                                // Parse as float
                                if let Ok(amount) = clean_amount.parse::<f64>() {
                                    return Some(amount);
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // Fallback: look for currency patterns in all lines
        for line in text.lines() {
            for pattern in &currency_patterns {
                if let Ok(regex) = Regex::new(pattern) {
                    if let Some(cap) = regex.captures(&line) {
                        if let Some(amount_str) = cap.get(1) {
                            let clean_amount = amount_str.as_str()
                                .replace(",", "")
                                .replace(" ", "");
                            
                            if let Ok(amount) = clean_amount.parse::<f64>() {
                                return Some(amount);
                            }
                        }
                    }
                }
            }
        }
        
        None
    }
    
    fn extract_merchant(&self, text: &str) -> Option<String> {
        // First try using the top part of the receipt for merchant name
        if let Some(top_data) = &self.top_crop {
            if let Ok(mut crop_tesseract) = LepTess::new(None, "eng+tha") {
                if crop_tesseract.set_image_from_mem(&top_data).is_ok() {
                    if let Ok(crop_text) = crop_tesseract.get_utf8_text() {
                        // Try to find merchant in the top crop
                        if let Some(merchant) = self.find_merchant_in_text(&crop_text) {
                            return Some(merchant);
                        }
                    }
                }
            }
        }
        
        // Fallback to full text
        self.find_merchant_in_text(text)
    }
    
    fn extract_date(&self, text: &str) -> Option<String> {
        // First try using the top part of the receipt for date
        if let Some(top_data) = &self.top_crop {
            if let Ok(mut crop_tesseract) = LepTess::new(None, "eng+tha") {
                if crop_tesseract.set_image_from_mem(&top_data).is_ok() {
                    if let Ok(crop_text) = crop_tesseract.get_utf8_text() {
                        // Try to find date in the top crop
                        if let Some(date) = self.find_date_in_text(&crop_text) {
                            return Some(date);
                        }
                    }
                }
            }
        }
        
        // Fallback to full text
        self.find_date_in_text(text)
    }
    
    fn find_date_in_text(&self, text: &str) -> Option<String> {
        // Look for common date formats
        
        // Thai date format: DD/MM/YYYY or DD-MM-YYYY
        if let Ok(thai_date_regex) = Regex::new(r"(\d{1,2})[/-](\d{1,2})[/-](\d{4})") {
            if let Some(caps) = thai_date_regex.captures(text) {
                if let (Some(day), Some(month), Some(year)) = (caps.get(1), caps.get(2), caps.get(3)) {
                    let day = day.as_str().parse::<u32>().ok()?;
                    let month = month.as_str().parse::<u32>().ok()?;
                    let year = year.as_str().parse::<i32>().ok()?;
                    
                    // Validate date components
                    if day > 0 && day <= 31 && month > 0 && month <= 12 {
                        // Format as ISO date string
                        return Some(format!("{:04}-{:02}-{:02}", year, month, day));
                    }
                }
            }
        }
        
        // Western date format: YYYY-MM-DD
        if let Ok(iso_date_regex) = Regex::new(r"(\d{4})-(\d{1,2})-(\d{1,2})") {
            if let Some(caps) = iso_date_regex.captures(text) {
                if let (Some(year), Some(month), Some(day)) = (caps.get(1), caps.get(2), caps.get(3)) {
                    let year = year.as_str().parse::<i32>().ok()?;
                    let month = month.as_str().parse::<u32>().ok()?;
                    let day = day.as_str().parse::<u32>().ok()?;
                    
                    // Validate date components
                    if day > 0 && day <= 31 && month > 0 && month <= 12 {
                        // Format as ISO date string
                        return Some(format!("{:04}-{:02}-{:02}", year, month, day));
                    }
                }
            }
        }
        
        // Try to find date with Thai month names
        let thai_months = [
            "มกราคม", "กุมภาพันธ์", "มีนาคม", "เมษายน", "พฤษภาคม", "มิถุนายน",
            "กรกฎาคม", "สิงหาคม", "กันยายน", "ตุลาคม", "พฤศจิกายน", "ธันวาคม"
        ];
        
        for (i, month_name) in thai_months.iter().enumerate() {
            let month_num = i + 1;
            // Pattern: day month_name year (e.g., "15 มกราคม 2566")
            let pattern = format!(r"(\d{{1,2}})\s*{}\s*(\d{{4}})", month_name);
            if let Ok(thai_text_date_regex) = Regex::new(&pattern) {
                if let Some(caps) = thai_text_date_regex.captures(text) {
                    if let (Some(day), Some(year)) = (caps.get(1), caps.get(2)) {
                        let day = day.as_str().parse::<u32>().ok()?;
                        let year = year.as_str().parse::<i32>().ok()?;
                        
                        // Adjust year if it's in Buddhist Era (BE)
                        let year_ce = if year > 2500 { year - 543 } else { year };
                        
                        // Validate date components
                        if day > 0 && day <= 31 {
                            // Format as ISO date string
                            return Some(format!("{:04}-{:02}-{:02}", year_ce, month_num, day));
                        }
                    }
                }
            }
        }
        
        None
    }
    
    fn extract_items(&self, text: &str) -> Vec<ItemData> {
        let mut items = Vec::new();
        
        // This is a simplified approach - in reality, we'd need more sophisticated parsing
        // based on the specific receipt format, which varies greatly
        
        // For Thai receipts, look for patterns like "item name....95" or "item name 1x95"
        if let Ok(item_regex) = Regex::new(r"([ก-๙a-zA-Z0-9\s]+)[\s\.]+(\d+)(?:x|\*)?(\d+(?:\.\d{1,2})?)?") {
            for line in text.lines() {
                if let Some(cap) = item_regex.captures(line) {
                    // Safely get the name from capture group 1
                    if let Some(name_match) = cap.get(1) {
                        let name = name_match.as_str().trim().to_string();
                        
                        // Check if this is a quantity x price pattern
                        let (quantity, price) = if let Some(price_match) = cap.get(3) {
                            // We have both quantity and price
                            let qty = cap.get(2)
                                .and_then(|q| q.as_str().parse::<u32>().ok());
                            let prc = price_match.as_str().parse::<f64>().ok();
                            (qty, prc)
                        } else {
                            // Just a price in group 2
                            let prc = cap.get(2)
                                .and_then(|p| p.as_str().parse::<f64>().ok());
                            (Some(1), prc)
                        };
                        
                        // Skip if it looks like a total line
                        let lower_name = name.to_lowercase();
                        if lower_name.contains("total") || 
                           lower_name.contains("รวม") || 
                           lower_name.contains("ทั้งหมด") ||
                           lower_name.len() < 2 {
                            continue;
                        }
                        
                        items.push(ItemData {
                            name,
                            price,
                            quantity,
                        });
                    }
                }
            }
        }
        
        items
    }
    
    pub async fn process_with_google_vision(&self, image_path: &Path) -> Result<OcrResult, AppError> {
        let start = std::time::Instant::now();
        
        // Read image file to bytes
        let img_bytes = fs::read(image_path).map_err(|e| AppError::IoError(e))?;
        
        // Set up the Vision API request
        let client = reqwest::Client::new();
        let api_key = std::env::var("GOOGLE_VISION_API_KEY")
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
        
        // Use the process_google_vision method to extract data
        let extracted_data = self.process_google_vision(&text, 0.9)?;
        
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
        let img_bytes = fs::read(image_path).map_err(|e| AppError::IoError(e))?;
        let tesseract_data = self.process_image(&img_bytes)?;
        
        // Create OcrResult from ExtractedData
        let tesseract_result = OcrResult {
            text: "".to_string(), // We don't store the full text in our new implementation
            extracted_data: tesseract_data.clone(),
            confidence: tesseract_data.confidence,
            processing_time: 0.0, // We don't track this in the new implementation
        };
        
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
    
    pub fn process_google_vision(&self, text: &str, confidence: f32) -> Result<ExtractedData, AppError> {
        // Extract data from the Google Vision API text
        let total = self.find_total_in_text(text);
        let date = self.find_date_in_text(text);
        let merchant = self.find_merchant_in_text(text);
        let items = self.extract_items(text);
        
        Ok(ExtractedData {
            total,
            date,
            merchant,
            items,
            confidence,
            ocr_source: "google_vision".to_string(),
        })
    }
    
    fn find_merchant_in_text(&self, text: &str) -> Option<String> {
        // Common Thai business name indicators
        let business_indicators = [
            "บริษัท", "ร้าน", "ห้าง", "สาขา", "ใบเสร็จ", "ใบกำกับภาษี"
        ];
        
        // Common merchant indicators in English
        let merchant_indicators = [
            "store", "shop", "restaurant", "cafe", "market", "mall", "supermarket"
        ];
        
        // First look for lines with business indicators
        let lines: Vec<&str> = text.lines().collect();
        
        // Check first few lines for business name
        for line in lines.iter().take(10) {
            let line_trimmed = line.trim();
            if line_trimmed.is_empty() {
                continue;
            }
            
            // Check for Thai business indicators
            for &indicator in &business_indicators {
                if line_trimmed.contains(indicator) {
                    return Some(line_trimmed.to_string());
                }
            }
            
            // Check for English merchant indicators
            let line_lower = line_trimmed.to_lowercase();
            for &indicator in &merchant_indicators {
                if line_lower.contains(indicator) {
                    return Some(line_trimmed.to_string());
                }
            }
        }
        
        // If no indicators found, use the first non-empty line that's not a date/time
        for line in lines.iter().take(3) {
            let line_trimmed = line.trim();
            if !line_trimmed.is_empty() && 
               !line_trimmed.contains("/") && 
               !line_trimmed.contains(":") && 
               !line_trimmed.chars().all(|c| c.is_digit(10) || c == '.' || c == ',' || c == '-') {
                return Some(line_trimmed.to_string());
            }
        }
        
        None
    }
} 