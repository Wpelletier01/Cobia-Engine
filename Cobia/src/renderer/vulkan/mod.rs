
pub mod base; 
pub mod utils;


use crate::ECobia;
use thiserror::Error;

use crate::core::logs::{CTRACE,CDEBUG};

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

    #[error("cant create device because: {0}")]
    DEVICE(String),

    #[error("cant create a window surface because of: {0}")]
    SURFACE(String)

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


const REQUIRED_VALIDATION_LAYERS: [&str;1] = ["VK_LAYER_KHRONOS_validation"];

pub(crate) struct VlkSystem {

    entry:      ash::Entry,
    instance:   ash::Instance,
    debuger:    base::VlkDebugSys,
    pdevices:   Vec<ash::vk::PhysicalDevice>,
    device:     ash::Device,
    surface:    base::VSurface

}
//
impl VlkSystem {


    fn init(appname: &str,window:&winit::window::Window) -> Result<Self,EVlk> {


        let entry = unsafe {
            ash::Entry::load().map_err(|e| EVlk::ENTRY(e.to_string()))?
        }; 

        CTRACE("Start vulkan instance creation");

        let instance = base::create_instance(appname, &entry)?;
        
        CTRACE("Successfully create a vulkan instance");

        let vdebuger = base::set_debug_utils(&entry, &instance)?;


        let pdevices = base::get_physical_devices(&instance)?;

        // TODO: 
        let qdevice = base::create_logical_device(pdevices[0], &instance)?;

        let vsurface = base::create_window_surface(&instance, &entry, window)?;

        Ok(
            VlkSystem{
                
                entry:      entry,
                instance:   instance,
                debuger:    vdebuger,
                pdevices:   pdevices,
                device:     qdevice,
                surface:    vsurface

            }
        )


    }


   

    
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