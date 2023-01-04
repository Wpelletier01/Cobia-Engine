
pub mod base; 
pub mod utils;

use crate::ECobia;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EVlk { 

    #[error("{source}")]
    GENERAL{
        #[from]
        source: ECobia,

    },

    #[error("Can't create a new vulkan instance because of {0}")]
    INSTANCE(String)


} 


#[cfg(test)]
mod tests {
    use super::*;

}


const REQUIRED_VALIDATION_LAYERS: [&str;1] = ["VK_LAYER_KHRONOS_validation"];

pub(crate) struct VlkSystem {

    entry:      ash::Entry,
    instance:   ash::Instance,
    
}
//
impl VlkSystem {


    fn init(appname: &str) -> Result<(),EVlk> {


        let entry = ash::Entry::new();

        if base::check_validation_layer_support(entry)
    



    }
    
}


