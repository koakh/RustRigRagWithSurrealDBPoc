// https://claude.ai/chat/d7c6afcc-80a4-410b-b316-1ffe22f37e6a
use std::borrow::Cow;

/// Sanitizes a string by removing or replacing problematic characters
/// that can cause issues with SurrealDB serialization
pub fn _sanitize_chunk(input: &str) -> String {
    // Method 1: Simple NUL byte removal
    input.replace('\0', "")
}

/// More comprehensive sanitization that handles various problematic characters
pub fn sanitize_chunk_comprehensive(input: &str) -> String {
    input
        .chars()
        .filter_map(|c| {
            match c {
                // Remove NUL bytes entirely
                '\0' => None,
                // Replace other control characters with space (optional)
                c if c.is_control() && c != '\n' && c != '\r' && c != '\t' => Some(' '),
                // Keep everything else
                _ => Some(c),
            }
        })
        .collect::<String>()
        // Clean up multiple consecutive spaces
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// Zero-copy approach for better performance with large chunks
pub fn _sanitize_chunk_cow(input: &str) -> Cow<str> {
    if input.contains('\0') {
        Cow::Owned(input.replace('\0', ""))
    } else {
        Cow::Borrowed(input)
    }
}

/// Validates UTF-8 and fixes any issues
pub fn _ensure_valid_utf8(input: &[u8]) -> String {
    String::from_utf8_lossy(input)
        .replace('\0', "")
        .to_string()
}

/// Advanced sanitization with character replacement options
pub fn _sanitize_chunk_advanced(input: &str, replace_nul_with: &str) -> String {
    let mut result = String::with_capacity(input.len());
    
    for c in input.chars() {
        match c {
            '\0' => result.push_str(replace_nul_with),
            // Handle other problematic characters
            c if c.is_control() && !matches!(c, '\n' | '\r' | '\t') => {
                result.push(' ');
            }
            _ => result.push(c),
        }
    }
    
    // Normalize whitespace
    result.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_chunk() {
        let dirty = "Hello\0World\0Test";
        let clean = _sanitize_chunk(dirty);
        assert_eq!(clean, "HelloWorldTest");
        assert!(!clean.contains('\0'));
    }

    #[test]
    fn test_sanitize_comprehensive() {
        let dirty = "Hello\0World\x01\x02Test\n\nMore text";
        let clean = sanitize_chunk_comprehensive(dirty);
        assert!(!clean.contains('\0'));
        assert!(!clean.contains('\x01'));
        println!("Clean text: {}", clean);
    }

    #[test]
    fn test_cow_approach() {
        let clean_text = "Hello World";
        let dirty_text = "Hello\0World";
        
        // Should not allocate for clean text
        let result1 = _sanitize_chunk_cow(clean_text);
        assert!(matches!(result1, Cow::Borrowed(_)));
        
        // Should allocate for dirty text
        let result2 = _sanitize_chunk_cow(dirty_text);
        assert!(matches!(result2, Cow::Owned(_)));
    }
}
