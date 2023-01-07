


#[cfg(target_os = "windows")]
use ash::extensions::khr::Win32Surface;
#[cfg(all(unix, not(target_os = "android"), not(target_os = "macos")))]
use ash::extensions::khr::XlibSurface;

use ash::extensions::ext::DebugUtils;
use ash::extensions::khr::Surface;

use std::os::raw::c_char;
use std::ffi::CStr;

use crate::ECobia;
use crate::core::logs::CERRORS;

use super::EVlk;

//
//
// ------------------------------------------------------------------------------------------------
// Require extensions
//
//
#[cfg(all(windows))]
pub fn required_extension_names() -> Vec<*const i8> {
    vec![
        Surface::name().as_ptr(),
        Win32Surface::name().as_ptr(),
        DebugUtils::name().as_ptr(),
    ]
}

#[cfg(all(unix, not(target_os = "android"), not(target_os = "macos")))]
pub fn required_extension_names() -> Vec<*const i8> {
    vec![
        Surface::name().as_ptr(),
        XlibSurface::name().as_ptr(),
        DebugUtils::name().as_ptr(),
    ]
}
//
//
// ------------------------------------------------------------------------------------------------
// Conversion 
//
//
/// Helper function to convert [c_char; SIZE] to string
pub fn vk_to_string(raw_string_array: &[c_char]) -> &str {

    let raw_string = unsafe {
        let pointer = raw_string_array.as_ptr();
        CStr::from_ptr(pointer)
    };

    match raw_string.to_str() {

        Ok(string) => string,
        Err(e) =>  {

            CERRORS(
                "{}", 
                &[
                    &ECobia::CONVERSION { 
                        from: "[u8]".into(), 
                        to: "&str".into(), 
                        how: e.to_string()}.to_string()]
            );
        
            ""

        }

    }
        
    
}

