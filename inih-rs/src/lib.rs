// inih-rs/src/lib.rs

// FIX #1: Changed `inih-sys` to `inih_sys` (hyphen to underscore)
use inih_sys as ffi; 
use std::collections::HashMap;
use std::ffi::{c_char, c_int, c_void, CStr, CString};
use std::path::Path;
// FIX #2: Removed the unused `use std::str;` line

#[derive(Debug, PartialEq, Default)]
pub struct Ini {
    data: HashMap<String, HashMap<String, String>>,
}

impl Ini {
    pub fn from_file(path: &Path) -> Result<Self, String> {
        let mut ini = Ini::default();
        let user_ptr = &mut ini as *mut _ as *mut c_void;

        // --- This is the "unsafe" part ---
        unsafe {
            // 1. Create a C-compatible string for the path
            let path_str = path.to_str().ok_or("Invalid path string")?;
            let c_path = CString::new(path_str).map_err(|e| e.to_string())?;

            // 2. Call the C function from our -sys crate
            let result_code = ffi::ini_parse(
                c_path.as_ptr(),        // <-- The C-style path
                Some(rust_ini_handler), // <-- Our Rust callback function
                user_ptr                // <-- The pointer to our 'ini' struct
            );

            // 3. Convert C error code to Rust Result
            if result_code == 0 {
                Ok(ini) // Success!
            } else if result_code == -1 {
                Err(format!("File not found: {:?}", path))
            } else {
                Err(format!("Error parsing file on line {}", result_code))
            }
        }
        // --- End of "unsafe" block ---
    }
}

// This is our Rust function that C will call
// FIX #3: Changed `#[no_mangle]` to `#[unsafe(no_mangle)]`
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

// --- This is our test ---
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