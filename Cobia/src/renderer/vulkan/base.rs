
// TODO: add comment

use ash::vk;
use std::ffi::{CString,c_void,CStr};
use std::ptr;



use super::EVlk;
use super::utils;


use crate::core::logs::{CDEBUG,CERROR,CERRORS,CWARN,CTRACES,CFATAL,CINFO, CFATALS, CWARNS, CINFOS, CDEBUGS};
use crate::define::{VLK_API_VERSION,VLK_ENGINE_VERSION,VLK_APP_VERSION};
use crate::renderer::vulkan::base;
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

    CDEBUG("Initializing info structure done");

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

            CFATALS("Unable to enumerate layer properties because: {}",&[&e.to_string()]);

            return false;

        }

    };


    if layer_properties.len() <= 0 {

        CERROR("No available layer");

        return false;


    }

    for req_layer in super::REQUIRED_VALIDATION_LAYERS.iter() {

        let mut found = false;

        for layer in layer_properties.iter() {

            let layer_name = match utils::vk_to_string(&layer.layer_name) {

                Ok(v) => v,
                Err(e) => {

                    CERRORS("{}",&[&e.to_string()]);

                    return false;

                }

            };

            if *req_layer == layer_name {

                found = true;
                break;

            }
            
    
        }


        if !found {

            CWARNS("Didn't find layer {}",&[req_layer.to_owned()]);

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
pub(crate) struct VlkDebugSys {

    pub util_loader:    ash::extensions::ext::DebugUtils,
    pub messenger:      vk::DebugUtilsMessengerEXT

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
//
// ------------------------------------------------------------------------------------------------
// Physical devices
//
//
pub(crate) fn get_physical_devices(inst:&ash::Instance) -> Result<Vec<vk::PhysicalDevice>,EVlk> {

    let devs = unsafe {
        
        match inst.enumerate_physical_devices() {

            Ok(d) => d,
            Err(e) => return Err(EVlk::PHYSICAL_DEVICES_ENUM(e.to_string()))

        }
        
    };

    let mut suitable:Vec<vk::PhysicalDevice> = Vec::new();


    for dev in devs.iter() {

        if base::is_pdevice_suitable(inst,dev) {

            suitable.push(*dev);

        }

    }

    Ok(suitable)

}
//
//
pub(crate) fn is_pdevice_suitable(inst:&ash::Instance,pdevice:&vk::PhysicalDevice) -> bool {

    
    // TODO: to finish 


    true 

}
//
//
pub(crate) fn check_pdevice_family_queue(
    inst:       &ash::Instance,
    pdevice:    vk::PhysicalDevice) -> Option<u32> {

    
    // TODO: to finish
    
    None 

}
//
//
// ------------------------------------------------------------------------------------------------
// Logical devices
//
//
pub(crate) fn create_logical_device(
    pdevice:    vk::PhysicalDevice,
    inst:       &ash::Instance ) -> Result<ash::Device,EVlk> {

    let qpriorities = [1.0_f32];

    // TODO: to finish
    let physical_device_features = vk::PhysicalDeviceFeatures {
        ..Default::default() // default just enable no feature.
    };


    let qcreate_info = vk::DeviceQueueCreateInfo {

        s_type:                 vk::StructureType::DEVICE_QUEUE_CREATE_INFO,
        p_next:                 ptr::null(),
        flags:                  vk::DeviceQueueCreateFlags::empty(),
        queue_family_index:     0,
        p_queue_priorities:     qpriorities.as_ptr(),
        queue_count:            qpriorities.len() as u32,

    };

    let device_info = vk::DeviceCreateInfo {

        s_type:                     vk::StructureType::DEVICE_CREATE_INFO,
        p_next:                     ptr::null(),
        flags:                      vk::DeviceCreateFlags::empty(),
        queue_create_info_count:    1,
        p_queue_create_infos:       &qcreate_info,
        enabled_layer_count:        0,
        pp_enabled_layer_names:     ptr::null(),
        enabled_extension_count:    0,
        pp_enabled_extension_names: ptr::null(),
        p_enabled_features:         &physical_device_features

    };

    let device = unsafe {

        match inst.create_device(pdevice, &device_info, None) {

            Ok(d) => d,
            Err(e) => return Err(EVlk::DEVICE(e.to_string()))

        }
        
    };

    Ok(device)


}
//
//
// ------------------------------------------------------------------------------------------------
// Window Surface 
//
//

pub(crate) struct VSurface {

    pub(crate) surface: vk::SurfaceKHR,
    pub(crate) loader:  ash::extensions::khr::Surface,


}
//
//
#[cfg(target_os = "linux")]
pub(crate) fn create_window_surface(
    inst:   &ash::Instance,
    entry:  &ash::Entry,
    window: &winit::window::Window ) -> Result<VSurface,EVlk> {

    use winit::platform::unix::WindowExtUnix;
    use ash::extensions::khr::XlibSurface;

    let x11_display = match window.xlib_display() {

        Some(d) => d,
        None => return Err(EVlk::SURFACE("Current display session doesn't use xlib".to_string()))

    };

    let x11_window = match window.xlib_window() {

        Some(d) => d,
        None => return Err(EVlk::SURFACE("Current display session doesn't use xlib".to_string()))

    };


    let x11_create_info = vk::XlibSurfaceCreateInfoKHR {

        s_type: vk::StructureType::XLIB_SURFACE_CREATE_INFO_KHR,
        p_next: ptr::null(),
        flags:  Default::default(),
        window: x11_window as vk::Window,
        dpy: x11_display as *mut vk::Display

    };

    let xlib_surf_loader = XlibSurface::new(entry, inst);

    let surf = unsafe { 
        match xlib_surf_loader.create_xlib_surface(&x11_create_info, None) {

            Ok(s) => s,
            Err(e) => return Err(EVlk::SURFACE(e.to_string()))

        }
    
    };

    let surf_loader = ash::extensions::khr::Surface::new(entry, inst);


    Ok(VSurface { surface: surf, loader: surf_loader} ) 

}