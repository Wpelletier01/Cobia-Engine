#![allow(dead_code)]


pub(crate) mod utils;
pub(crate) mod queue_family;
pub(crate) mod debug;
pub(crate) mod physical_device;
pub(crate) mod logical_device;
pub(crate) mod validation_layer;
pub(crate) mod surface;
pub(crate) mod swapchain;

use std::ffi::CString;
use std::ptr;


use ash::vk;
use thiserror::Error;

use crate::core::logs::{CFATALS};
use crate::ECobia;
use crate::core::logs::{CTRACE,CDEBUG,CERROR,CERRORS,CWARNS};
use crate::define::{VLK_API_VERSION,VLK_ENGINE_VERSION,VLK_APP_VERSION};

#[allow(non_camel_case_types)]
#[derive(Error, Debug)]
pub enum EVlk { 

    #[error("{source}")]
    GENERAL{
        #[from]
        source: ECobia,

    },

    #[error("Cant load vulkan library because: {0}")] 
    ENTRY(String), 

    #[error("Can't create a new vulkan instance because of {0}")]
    INSTANCE(String),

    #[error("Cant initialize vulkan debug utilities because: {0}")]
    DEBUG_UTILS(String),

    #[error("can't enumerate physical devices because: {0}")]
    PHYSICAL_DEVICES_ENUM(String),

    #[error("no physical device suitable for vulkan has been found")]
    PHYSICAL_DEVICES_FOUND,

    #[error("cant create logical device because: {0}")]
    LOGIC_DEVICE(String),

    #[error("cant create a window surface because of: {0}")]
    SURFACE(String),

    #[error("can't access surface data because: {0}")]
    SURFACE_DATA(String),

    #[error("cant create a swapchain because: {0}")]
    SWAPCHAIN(String),

} 


#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::logs;

    #[test]
    fn init_vlk_sys() {

        logs::init();
        
      



        // let sys = VlkSystem::init("test",&win).unwrap();

      


    }


}


const REQUIRED_LAYERS: [&str;1] = ["VK_LAYER_KHRONOS_validation"];

pub(crate) struct VlkSystem {

    entry:          ash::Entry,
    instance:       ash::Instance,
    vlayer:         validation_layer::ValidationLayer,
    debuger:        debug::VlkDebugSys,
    pdevices:       Vec<ash::vk::PhysicalDevice>,
    logical_device: logical_device::LogicDevice,
    surface:        surface::VSurface

}
//
impl VlkSystem {


    fn init(appname: &str,window:&winit::window::Window) -> Result<Self,EVlk> {


        let entry = unsafe {
            ash::Entry::load().map_err(|e| EVlk::ENTRY(e.to_string()))?
        }; 

        let vlayer = validation_layer::ValidationLayer::new(REQUIRED_LAYERS);

        vlayer.check(&entry);

        let instance = create_instance(appname, &entry)?;
        

        let vdebuger = debug::set_debug_utils(&entry, &instance)?;

        
        let pdevices = physical_device::get_physical_devices(&instance)?;

        // TODO: for now we just take the first pdevice in the vector 
        // TODO: implement something later
        let logic_device =  logical_device::LogicDevice::new(&instance, pdevices[0], &vlayer)?;

        let vsurface = surface::create_window_surface(&instance, &entry, window)?;

        Ok(
            VlkSystem{
                
                entry:          entry,
                vlayer:         vlayer,
                instance:       instance,
                debuger:        vdebuger,
                pdevices:       pdevices,
                logical_device: logic_device,
                surface:        vsurface

            }
        )


    }
    //
    //

    
}

impl Drop for VlkSystem {


    fn drop(&mut self) {

        unsafe {
            
            self.device.destroy_device(None);
            
            self.debuger.util_loader.destroy_debug_utils_messenger(self.debuger.messenger, None);
            
            self.surface.loader.destroy_surface(self.surface.surface,None);
            
            self.instance.destroy_instance(None);
            

        }
         
    }

}
//
//
// --------------------------------------------------------------------------------------------
// Create an instance 
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
