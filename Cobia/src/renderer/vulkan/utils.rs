


#[cfg(target_os = "windows")]
use ash::extensions::khr::Win32Surface;
#[cfg(all(unix, not(target_os = "android"), not(target_os = "macos")))]
use ash::extensions::khr::XlibSurface;

use ash::extensions::ext::DebugUtils;
use ash::extensions::khr::Surface;

use std::os::raw::c_char;
use std::ffi::CStr;

use crate::ECobia;

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
pub fn vk_to_string(raw_string_array: &[c_char]) -> Result<&str,EVlk> {

    let raw_string = unsafe {
        let pointer = raw_string_array.as_ptr();
        CStr::from_ptr(pointer)
    };

    match raw_string.to_str() {

        Ok(string) => Ok(string),
        Err(e) => return Err(
            EVlk::from(ECobia::CONVERSION { 
                from: "CStr".to_owned(), 
                to: "str".to_owned(),
                how: "translate str for vulkan".to_owned() 
                }
            )
        
        )

    }
        
    
}

