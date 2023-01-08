
// TODO: add comment

use std::ptr;
use std::ffi::{c_void,CStr};

use ash::vk;
use error_stack::{Result};

use crate::core::logs::*;
use crate::core::error_handler::EVlkApi;



unsafe extern "system" fn vlk_debug_utils_callback(

    severity :      vk::DebugUtilsMessageSeverityFlagsEXT,
    type_:          vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback:     *const vk::DebugUtilsMessengerCallbackDataEXT,
    _p_usr_data:    *mut c_void


) -> vk::Bool32 {

    let type_str = match type_ {

        vk::DebugUtilsMessageTypeFlagsEXT::GENERAL => "VLK: [General]",
        vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE => "VLK: [Performance]",
        vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION => "VLK: [Validation]",
        vk::DebugUtilsMessageTypeFlagsEXT::DEVICE_ADDRESS_BINDING => "VLK: [Device Address Binding]",
        _ => { CFATAL("Unknown message type passed to the debug callback"); 

        // TODO: find a way to handle this situation
        panic!()

    }   


    };

    let msg = match CStr::from_ptr((*p_callback).p_message).to_str() {

        Ok(m) => m,
        Err(e) => {
            
            CERROR("Vulkan debug callback receive a message with invalid utf-8 char");

            CWARN("Important debug info will be missing");
            
        
            ""
            
        }


    };

    
    match severity {

        vk::DebugUtilsMessageSeverityFlagsEXT::ERROR =>    { CERRORS("{}: {}",&[type_str,msg]); },
        vk::DebugUtilsMessageSeverityFlagsEXT::INFO  =>    { CINFOS("{}: {}",&[type_str,msg]);  },
        vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE =>  { CDEBUGS("{}: {}",&[type_str,msg]); },
        _ => { CFATAL("Unknown debug severity passed to the debug callback"); }

    }

    vk::FALSE

}
//
//
//
pub(crate) struct VlkDebugSys {

    util_loader:    ash::extensions::ext::DebugUtils,
    messenger:      vk::DebugUtilsMessengerEXT

}
//
impl VlkDebugSys {

    pub(crate) fn new(entry:&ash::Entry,instance:&ash::Instance) -> Result<Self,EVlkApi> {


        let dutils_loader = ash::extensions::ext::DebugUtils::new(entry, instance);

        let msg_ci = vk::DebugUtilsMessengerCreateInfoEXT {

            s_type: vk::StructureType::DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT,
            p_next: ptr::null(),
            flags:  vk::DebugUtilsMessengerCreateFlagsEXT::empty(),
            message_severity: 
                vk::DebugUtilsMessageSeverityFlagsEXT::WARNING |
                vk::DebugUtilsMessageSeverityFlagsEXT::ERROR |
                vk::DebugUtilsMessageSeverityFlagsEXT::INFO |
                vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE,
            message_type: 
                vk::DebugUtilsMessageTypeFlagsEXT::GENERAL |
                vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE |
                vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION |
                vk::DebugUtilsMessageTypeFlagsEXT::DEVICE_ADDRESS_BINDING,
            
            pfn_user_callback: Some(vlk_debug_utils_callback),
            p_user_data: ptr::null_mut()


        };

        let utils_messenger = unsafe {
            match dutils_loader.create_debug_utils_messenger(&msg_ci, None) {

                Ok(m) => m,
                Err(e) => return Err(
                    EVlkApi::DEBUG
                        .as_report()
                        .attach_printable(
                            format!(
                                "Could not create debug utils messenger because of {}", 
                                e.to_string()
                            )
                        )
                )

            }
        };

        Ok( VlkDebugSys { util_loader: dutils_loader, messenger: utils_messenger } )


    }



}


//
//
pub(crate) fn set_debug_utils(entry:&ash::Entry,instance:&ash::Instance) -> Result<VlkDebugSys,EVlkApi> {

    let dutils_loader = ash::extensions::ext::DebugUtils::new(entry, instance);

    let msg_ci = vk::DebugUtilsMessengerCreateInfoEXT {

        s_type: vk::StructureType::DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT,
        p_next: ptr::null(),
        flags:  vk::DebugUtilsMessengerCreateFlagsEXT::empty(),
        message_severity: 
            vk::DebugUtilsMessageSeverityFlagsEXT::WARNING |
            vk::DebugUtilsMessageSeverityFlagsEXT::ERROR |
            vk::DebugUtilsMessageSeverityFlagsEXT::INFO |
            vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE,
        message_type: 
            vk::DebugUtilsMessageTypeFlagsEXT::GENERAL |
            vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE |
            vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION |
            vk::DebugUtilsMessageTypeFlagsEXT::DEVICE_ADDRESS_BINDING,
        
        pfn_user_callback: Some(vlk_debug_utils_callback),
        p_user_data: ptr::null_mut()


    };

    let utils_messenger = unsafe {
        match dutils_loader.create_debug_utils_messenger(&msg_ci, None) {

            Ok(m) => m,
            Err(e) => return Err(EVlkApi::DEBUG.attach_printable_default(e))

        }
    };

    Ok( VlkDebugSys { util_loader: dutils_loader, messenger: utils_messenger } )


}