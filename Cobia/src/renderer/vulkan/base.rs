
// TODO: add comment

use ash::vk;
use std::ffi::{CString,c_void,CStr};
use std::ptr;



use super::EVlk;
use super::utils;


use crate::core::logs::{CTRACE,CDEBUG,CERROR,CWARN,CTRACES,CFATAL,CINFO};
use crate::define::{VLK_API_VERSION,VLK_ENGINE_VERSION,VLK_APP_VERSION};
//
//
// ------------------------------------------------------------------------------------------------
// Instance creation
//
//
pub(crate) fn create_instance(app_name:&str,entry: &ash::Entry) -> Result<ash::Instance,EVlk> {

    let c_app_name = match CString::new(app_name) {

        Ok(cstr) => cstr,
        Err(e) => return Err(EVlk::INSTANCE("app_name have a null character".to_string()))


    };

    
    let engine = CString::new("Cobia").unwrap();

    let app_info = vk::ApplicationInfo {

        s_type:                 vk::StructureType::APPLICATION_INFO,
        p_next:                 ptr::null(),
        p_application_name:     c_app_name.as_ptr(),
        application_version:    VLK_APP_VERSION,
        p_engine_name:          engine.as_ptr(),
        engine_version:         VLK_ENGINE_VERSION,
        api_version:            VLK_API_VERSION

    }; 

    let require_extensions = utils::required_extension_names();

    

    let create_info = vk::InstanceCreateInfo {

        s_type: vk::StructureType::INSTANCE_CREATE_INFO,
        p_next: ptr::null(),
        flags:  vk::InstanceCreateFlags::empty(),
        p_application_info: &app_info,
        pp_enabled_layer_names: ptr::null(),
        enabled_layer_count: 0,
        pp_enabled_extension_names: require_extensions.as_ptr(),
        enabled_extension_count: require_extensions.len() as u32,

    };

    unsafe {
        
        match entry.create_instance(&create_info, None) {

            Ok(inst) => Ok(inst),
            Err(e) => Err(EVlk::INSTANCE(e.to_string()))
        }
    } 

}
// 
//
// ------------------------------------------------------------------------------------------------
// Validation layer
//
//
// TODO: integrate with debug and instance creation
//
pub fn check_validation_layer_support(entry: &ash::Entry) -> bool {


    let layer_properties = match entry.enumerate_instance_layer_properties() {

        Ok(v) => v,
        Err(e) => {

            CFATAL("Unable to enumerate layer properties because: {}",&[&e.to_string()]);

            return false;

        }

    };


    if layer_properties.len() <= 0 {

        CERROR("No available layer",&[]);

        return false;


    }

    for req_layer in super::REQUIRED_VALIDATION_LAYERS.iter() {

        let mut found = false;

        for layer in layer_properties.iter() {

            let layer_name = match utils::vk_to_string(&layer.layer_name) {

                Ok(v) => v,
                Err(e) => {

                    CERROR("{}",&[&e.to_string()]);

                    return false;

                }

            };

            if *req_layer == layer_name {

                found = true;
                break;

            }
            
    
        }


        if !found {

            CWARN("Didn't find layer {}",&[req_layer.to_owned()]);

            return false;
        }


    }
    
    
    true 

}
//
//
// ------------------------------------------------------------------------------------------------
// Debug 
//
//
///
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
        _ => { CFATAL("Unknown message type passed to the debug callback",&[]); 

        // TODO: find a way to handle this situation
        panic!()

    }   


    };

    let msg = match CStr::from_ptr((*p_callback).p_message).to_str() {

        Ok(m) => m,
        Err(e) => {
            
            CERROR("Vulkan debug callback receive a message with invalid utf-8 char",&[]);

            CWARN("Important debug info will be missing",&[]);
            
        
            ""
            
        }


    };

    
    match severity {

        vk::DebugUtilsMessageSeverityFlagsEXT::ERROR =>    { CERROR("{}: {}",&[type_str,msg]); },
        vk::DebugUtilsMessageSeverityFlagsEXT::INFO  =>    { CINFO("{}: {}",&[type_str,msg]);  },
        vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE =>  { CDEBUG("{}: {}",&[type_str,msg]); },
        _ => { CFATAL("Unknow debug severity passed to the debug callback",&[]); }

    }

    vk::FALSE

}
//
//
pub(crate) struct VlkDebugSys {

    util_loader:    ash::extensions::ext::DebugUtils,
    messenger:      vk::DebugUtilsMessengerEXT

}

//
//
pub(crate) fn set_debug_utils(entry:&ash::Entry,instance:&ash::Instance) -> Result<VlkDebugSys,EVlk> {

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
            Err(e) => return Err(EVlk::DEBUG_UTILS(e.to_string()))

        }
    };

    Ok( VlkDebugSys { util_loader: dutils_loader, messenger: utils_messenger } )


}
//

