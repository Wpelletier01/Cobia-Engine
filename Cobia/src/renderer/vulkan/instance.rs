
// TODO: add comment

use crate::define::{VLK_API_VERSION,VLK_ENGINE_VERSION,VLK_APP_VERSION};
use super::{Result,EVlkApi,utils};
use crate::core::error_handler::EGeneral;
use crate::core::logs::CDEBUG;

use std::ffi::CString;
use std::ptr;

use ash::vk;

pub(crate) fn create_instance(app_name:&str,entry: &ash::Entry) -> Result<ash::Instance,EVlkApi> {

    let c_app_name = match CString::new(app_name) {

        Ok(cstr) => cstr,
        Err(e) => return Err(

            EGeneral::C_STRING_PARSING
                .attach_printable_default(e)
                .change_context(EVlkApi::INSTANCE)
                .attach_printable("error when creating instance")

        )
    
    
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
    
    CDEBUG("Initializing info structure done");

    unsafe {
        
        match entry.create_instance(&create_info, None) {

            Ok(inst) => Ok(inst),
            Err(e) => Err(EVlkApi::INSTANCE
                .as_report()
                .attach_printable(format!("Can't create instance because of {}",e.to_string())) 
                ) 
        }
    } 
    
}
//
//
