use crate::ocr::processor::{ExtractedData, ItemData};
use regex::Regex;

// Extract key information from OCR text
pub fn extract_structured_data(text: &str) -> ExtractedData {
    ExtractedData {
        total: extract_total(text),
        date: extract_date(text),
        merchant: extract_merchant(text),
        items: extract_items(text),
        confidence: 0.0,
        ocr_source: "tesseract".to_string(),
    }
}

// Extract total amount
fn extract_total(text: &str) -> Option<f64> {
    // Try to find a line with "total" and a price
    if let Some(total_regex) = Regex::new(r"(?i)(total|amount|sum)[:\s]*[$]?(\d+\.\d{2})").ok() {
        if let Some(cap) = total_regex.captures(text) {
            if let Some(amount_str) = cap.get(2) {
                if let Ok(amount) = amount_str.as_str().parse::<f64>() {
                    return Some(amount);
                }
            }
        }
    }
    
    // Try to find any price at the end of a line (often the total)
    if let Some(alt_regex) = Regex::new(r"[\$]?(\d+\.\d{2})\s*$").ok() {
        for line in text.lines() {
            let line = line.trim();
            // Skip lines that are likely to be individual items
            if line.contains("subtotal") || line.contains("tax") || 
               line.len() < 10 || line.chars().filter(|c| c.is_alphabetic()).count() < 3 {
                continue;
            }
            
            if let Some(cap) = alt_regex.captures(line) {
                if let Some(amount_str) = cap.get(1) {
                    if let Ok(amount) = amount_str.as_str().parse::<f64>() {
                        return Some(amount);
                    }
                }
            }
        }
    }
    
    None
}

// Extract date
fn extract_date(text: &str) -> Option<String> {
    // Try to find a date in various formats
    let date_patterns = [
        r"(\d{1,2})[/\-\.](\d{1,2})[/\-\.](\d{2,4})",
        r"(\d{4})[/\-\.](\d{1,2})[/\-\.](\d{1,2})",
        r"(Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec)[a-z]*\.?\s+(\d{1,2})(?:st|nd|rd|th)?,?\s+(\d{2,4})",
    ];
    
    for pattern in &date_patterns {
        if let Some(regex) = Regex::new(pattern).ok() {
            for line in text.lines() {
                if let Some(cap) = regex.captures(line) {
                    return Some(cap[0].to_string());
                }
            }
        }
    }
    
    // Look for lines with "date" and try to extract a date
    if let Some(date_line_regex) = Regex::new(r"(?i)\b(date|time)\b").ok() {
        for line in text.lines() {
            if date_line_regex.is_match(line) {
                for pattern in &date_patterns {
                    if let Some(regex) = Regex::new(pattern).ok() {
                        if let Some(cap) = regex.captures(line) {
                            return Some(cap[0].to_string());
                        }
                    }
                }
            }
        }
    }
    
    None
}

// Extract merchant name
fn extract_merchant(text: &str) -> Option<String> {
    let lines: Vec<&str> = text.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect();
    
    // Often the first non-empty line is the merchant name
    // But we want to avoid lines that look like dates or times
    for line in lines.iter().take(5) {
        if line.len() > 3 {
            // Skip if it looks like a date or time
            if Regex::new(r"^\d+[/\-\.]\d+[/\-\.]\d+$").ok()?.is_match(line) ||
               Regex::new(r"^\d+:\d+").ok()?.is_match(line) {
                continue;
            }
            
            // Skip if it's just a number
            if Regex::new(r"^\d+$").ok()?.is_match(line) {
                continue;
            }
            
            return Some(line.to_string());
        }
    }
    
    None
}

// Extract individual items
fn extract_items(text: &str) -> Vec<ItemData> {
    let mut items = Vec::new();
    
    // This requires more sophisticated parsing based on receipt format
    // Here's a simplified approach for common formats
    if let Some(item_regex) = Regex::new(r"([a-zA-Z\s]+)[\s]+(\d+(?:\.\d{2})?)").ok() {
        for line in text.lines() {
            let line = line.trim();
            
            // Skip short lines or ones with too many numbers (likely not items)
            if line.len() < 5 || line.chars().filter(|c| c.is_numeric()).count() > 7 {
                continue;
            }
            
            // Skip lines containing keywords like "total", "subtotal", etc.
            if line.to_lowercase().contains("total") || 
               line.to_lowercase().contains("subtotal") || 
               line.to_lowercase().contains("tax") {
                continue;
            }
            
            if let Some(cap) = item_regex.captures(line) {
                if cap.len() >= 3 {
                    let name = cap[1].trim().to_string();
                    let price = cap[2].parse::<f64>().ok();
                    
                    if name.len() > 2 {
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