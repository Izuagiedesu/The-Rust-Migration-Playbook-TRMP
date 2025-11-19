// jsmn-rs/src/lib.rs
use jsmn_sys as ffi;
use std::ffi::CString;

/// A safe wrapper around the C jsmn parser
pub struct JsmnParser {
    // We wrap the C struct directly
    raw: ffi::jsmn_parser,
}

impl JsmnParser {
    /// Initialize a new parser
    pub fn new() -> Self {
        // Create uninitialized memory for the C struct
        let mut parser = std::mem::MaybeUninit::uninit();
        unsafe {
            // Call the C init function to set up the struct
            ffi::jsmn_init(parser.as_mut_ptr());
            JsmnParser {
                raw: parser.assume_init(),
            }
        }
    }

    /// Parse JSON string into the provided slice of tokens.
    /// Returns the number of tokens parsed.
    pub fn parse(&mut self, js: &str, tokens: &mut [ffi::jsmntok_t]) -> Result<usize, String> {
        let c_js = CString::new(js).map_err(|e| e.to_string())?;
        
        unsafe {
            let count = ffi::jsmn_parse(
                &mut self.raw,        // Pointer to parser
                c_js.as_ptr(),        // Pointer to JSON string
                js.len(),             // Length of string
                tokens.as_mut_ptr(),  // Pointer to token array
                tokens.len() as u32   // Size of token array
            );

            // FIX: We use the raw integer values directly to avoid bindgen naming issues.
            // -1 = JSMN_ERROR_NOMEM (Not enough tokens)
            // -2 = JSMN_ERROR_INVAL (Invalid JSON)
            // -3 = JSMN_ERROR_PART (Incomplete JSON)
            if count < 0 {
                match count {
                    -1 => Err("Not enough tokens (JSMN_ERROR_NOMEM)".to_string()),
                    -2 => Err("Invalid JSON string (JSMN_ERROR_INVAL)".to_string()),
                    -3 => Err("Incomplete JSON string (JSMN_ERROR_PART)".to_string()),
                    _ => Err("Unknown error".to_string()),
                }
            } else {
                Ok(count as usize)
            }
        }
    }
}

// --- THE TEST ---
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_json() {
        let json = "{\"name\": \"Keshee\", \"id\": 123}";
        let mut parser = JsmnParser::new();
        
        // We allocate space for 10 tokens (on the stack, very fast)
        let mut tokens: [ffi::jsmntok_t; 10] = unsafe { std::mem::zeroed() };

        let count = parser.parse(json, &mut tokens).expect("Failed to parse");
        
        // 1 Object + 1 "name" + 1 "Keshee" + 1 "id" + 1 "123" = 5 tokens
        assert_eq!(count, 5); 
    }
}