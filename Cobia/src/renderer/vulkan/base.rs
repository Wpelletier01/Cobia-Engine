
// TODO: add comment

use ash::vk;
use std::ffi::CString;
use std::ptr;

use super::EVlk;
use super::utils;

use crate::CERROR;
use crate::CFATAL;
use crate::CWARN;
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
pub fn check_validation_layer_support(entry: &ash::Entry) -> bool {


    let layer_properties = match entry.enumerate_instance_layer_properties() {

        Ok(v) => v,
        Err(e) => {

            CFATAL!("Unable to enumerate layer properties because: {}",e.to_string().as_str());

            return false;

        }

    };


    if layer_properties.len() <= 0 {

        CERROR!("No available layer");

        return false;


    }

    for req_layer in super::REQUIRED_VALIDATION_LAYERS.iter() {

        let mut found = false;

        for layer in layer_properties.iter() {

            let layer_name = match utils::vk_to_string(&layer.layer_name) {

                Ok(v) => v,
                Err(e) => {

                    CERROR!("{}",&e.to_string());

                    return false;

                }

            };

            if *req_layer == layer_name {

                found = true;
                break;

            }
            
    
        }


        if !found {

            CWARN!("Didn't find layer {}",req_layer.to_owned());

            return false;
        }


    }
    
    
    true 

}