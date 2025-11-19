// inih-rs/src/lib.rs

// --- We no longer use `inih-sys`, so the `use` is gone ---
use std::collections::HashMap;
use std::ffi::{c_char, c_int, c_void, CStr, CString};
use std::path::Path;

#[derive(Debug, PartialEq, Default)]
pub struct Ini {
    data: HashMap<String, HashMap<String, String>>,
}

// --- THIS IS THE NEW, PURE-RUST REWRITE ---
impl Ini {
    pub fn from_file(path: &Path) -> Result<Self, String> {
        // 1. Create our Rust struct that will hold the data.
        let mut ini = Ini::default();

        // 2. Create a C-style `void*` pointer to our struct.
        //    We still need this because our `rust_ini_handler` expects it.
        let user_ptr = &mut ini as *mut _ as *mut c_void;

        // --- PURE RUST PARSER ---
        // 3. Read the entire file into a string.
        let file_content = std::fs::read_to_string(path)
            .map_err(|e| format!("File not found or unreadable: {:?}", e))?; // Replicates C error -1
        
        let mut current_section = CString::new("").unwrap(); // Default section
        let mut line_number = 0;

        // 4. Process the file line by line
        for line in file_content.lines() {
            line_number += 1;
            let line = line.trim();

            // 5. Skip empty lines and comments
            if line.is_empty() || line.starts_with(';') || line.starts_with('#') {
                continue;
            }

            // 6. Check for [section]
            if line.starts_with('[') && line.ends_with(']') {
                // Get the section name, trim it, and make a CString
                let section_name = line[1..line.len() - 1].trim();
                current_section = CString::new(section_name)
                    .map_err(|e| format!("Error on line {}: Invalid section name. {}", line_number, e))?;
                continue;
            }

            // 7. Check for key = value
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim();

                // Create CStrings to pass to our handler
                let c_key = CString::new(key)
                    .map_err(|e| format!("Error on line {}: Invalid key. {}", line_number, e))?;
                let c_value = CString::new(value)
                    .map_err(|e| format!("Error on line {}: Invalid value. {}", line_number, e))?;

                // 8. Call our *own* handler function
                //    This is the "hot-swap"
                let result = rust_ini_handler(
                    user_ptr,
                    current_section.as_ptr(),
                    c_key.as_ptr(),
                    c_value.as_ptr()
                );

                // Check the handler's result code
                if result != 1 {
                    // This error comes from our handler (or would, if it ever returned != 1)
                    return Err(format!("Error parsing file on line {}", line_number)); // Replicates C error
                }
            }
            // If the line is not a section, comment, or key/value,
            // the `inih` C parser just ignores it. So do we.
        }

        // 9. Success!
        Ok(ini)
        // --- END OF PURE RUST PARSER ---
    }
}

// --- THIS HANDLER IS STILL USED BY OUR RUST PARSER ---
#[unsafe(no_mangle)]
extern "C" fn rust_ini_handler(
    user_data: *mut c_void,
    section: *const c_char,
    name: *const c_char,
    value: *const c_char,
) -> c_int {
    // Unsafe block to convert C pointers back to Rust data
    unsafe {
        let ini = &mut *(user_data as *mut Ini);
        let section_str = CStr::from_ptr(section).to_str().unwrap_or("");
        let name_str = CStr::from_ptr(name).to_str().unwrap_or("");
        let value_str = CStr::from_ptr(value).to_str().unwrap_or("");

        // Now we are in safe Rust!
        let section_map = ini.data.entry(section_str.to_string()).or_default();
        section_map.insert(name_str.to_string(), value_str.to_string());
    }
    
    // 1 means "success" to C
    1
}

// --- OUR TEST REMAINS UNCHANGED (AND SHOULD STILL PASS) ---
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn it_parses_a_real_file() {
        // 1. Define the expected data
        let mut expected_user = HashMap::new();
        expected_user.insert("name".to_string(), "Test User".to_string());
        expected_user.insert("email".to_string(), "test@example.com".to_string());

        let mut expected_proto = HashMap::new();
        expected_proto.insert("version".to_string(), "6".to_string());

        let mut expected_data = HashMap::new();
        expected_data.insert("user".to_string(), expected_user);
        expected_data.insert("protocol".to_string(), expected_proto);

        let expected_ini = Ini { data: expected_data };

        // 2. Call our function (this assumes 'test.ini' exists)
        let test_path = Path::new("test.ini");
        let parsed_ini = Ini::from_file(test_path).expect("Failed to parse test.ini");

        // 3. Assert
        assert_eq!(parsed_ini, expected_ini);
    }
}