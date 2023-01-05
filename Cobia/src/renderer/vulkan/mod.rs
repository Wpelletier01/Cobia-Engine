
pub mod base; 
pub mod utils;


use crate::{ECobia, CTRACE};
use thiserror::Error;



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

} 


#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::logs;

    #[test]
    fn init_vlk_sys() {

        logs::init();
        
        let sys = VlkSystem::init("test").unwrap();

      


    }


}


const REQUIRED_VALIDATION_LAYERS: [&str;1] = ["VK_LAYER_KHRONOS_validation"];

pub(crate) struct VlkSystem {

    entry:      ash::Entry,
    instance:   ash::Instance,
    debuger:    base::VlkDebugSys

}
//
impl VlkSystem {


    fn init(appname: &str) -> Result<Self,EVlk> {


        let entry = unsafe {
            ash::Entry::load().map_err(|e| EVlk::ENTRY(e.to_string()))?
        }; 

        CTRACE!("Start vulkan instance creation");

        let instance = base::create_instance(appname, &entry)?;
        
        CTRACE!("Successfully create a vulkan instance");

        let vdebuger = base::set_debug_utils(&entry, &instance)?;


        Ok(
            VlkSystem{
                entry:      entry,
                instance:   instance,
                debuger:    vdebuger,
            }
        )


    }


   

    
}


