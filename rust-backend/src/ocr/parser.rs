use regex::Regex;

// Parse total amount from OCR text
pub fn parse_total(text: &str) -> Option<f64> {
    let total_regex = Regex::new(r"(?i)(total|amount|sum)[:\s]*[$]?(\d+\.\d{2})").ok()?;
    
    if let Some(cap) = total_regex.captures(text) {
        if let Some(amount_str) = cap.get(2) {
            if let Ok(amount) = amount_str.as_str().parse::<f64>() {
                return Some(amount);
            }
        }
    }
    
    // Try another common pattern (just the amount with $ sign at the end of a line)
    let alt_regex = Regex::new(r"[$]?(\d+\.\d{2})\s*$").ok()?;
    
    for line in text.lines() {
        if let Some(cap) = alt_regex.captures(line.trim()) {
            if let Some(amount_str) = cap.get(1) {
                if let Ok(amount) = amount_str.as_str().parse::<f64>() {
                    return Some(amount);
                }
            }
        }
    }
    
    None
}

// Parse date from OCR text
pub fn parse_date(text: &str) -> Option<String> {
    // Try common date formats
    let date_formats = [
        // MM/DD/YYYY
        Regex::new(r"(\d{1,2})[/\-\.](\d{1,2})[/\-\.](\d{2,4})").ok()?,
        // Textual format like "Jan 1, 2023"
        Regex::new(r"(Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec)[a-z]*\.?\s+(\d{1,2})(?:st|nd|rd|th)?,?\s+(\d{2,4})").ok()?,
        // YYYY-MM-DD
        Regex::new(r"(\d{4})[/\-\.](\d{1,2})[/\-\.](\d{1,2})").ok()?,
    ];
    
    for line in text.lines() {
        for regex in &date_formats {
            if let Some(cap) = regex.captures(line) {
                return Some(cap[0].to_string());
            }
        }
    }
    
    // Look for lines containing "date" or "time"
    let date_line_regex = Regex::new(r"(?i)date|time").ok()?;
    
    for line in text.lines() {
        if date_line_regex.is_match(line) {
            // Then try to match a date format in this line
            for regex in &date_formats {
                if let Some(cap) = regex.captures(line) {
                    return Some(cap[0].to_string());
                }
            }
        }
    }
    
    None
}

// Parse merchant name from OCR text
pub fn parse_merchant(text: &str) -> Option<String> {
    let lines: Vec<&str> = text.lines().collect();
    
    // Often the first non-empty line is the merchant name
    for line in lines.iter().take(5) {
        let trimmed = line.trim();
        if !trimmed.is_empty() && trimmed.len() > 3 {
            // Skip lines that are likely dates or times
            if !Regex::new(r"^\d+[/\-\.]\d+[/\-\.]\d+$").ok()?.is_match(trimmed) &&
               !Regex::new(r"^\d+:\d+").ok()?.is_match(trimmed) {
                return Some(trimmed.to_string());
            }
        }
    }
    
    None
} 