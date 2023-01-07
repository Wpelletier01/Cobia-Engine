
use std::ffi::{c_char,CString};

use crate::core::logs::{CFATALS,CERROR,CWARNS};

use super::utils;

pub(crate) struct ValidationLayer {

    required: [&'static str; 1],
    is_enabled: bool 
}
//
impl ValidationLayer {

    pub(crate) fn new(required_layer: [&'static str;1]) -> ValidationLayer { 

        ValidationLayer { required: required_layer, is_enabled: true }
 
    }

    pub(crate) fn check(&mut self,entry:&ash::Entry) {

        match entry.enumerate_instance_layer_properties() {

            Ok(layers) => {


                if layers.len() <= 0 {

                    CERROR("Expected at least one layer properties. found None");
                    
                    self.is_enabled = false;

                } else {

                    for req_layer in self.required.iter() {
    
                        let mut found = false;
                
                        for layer in layers.iter() {
                
                            let layer_name = utils::vk_to_string(&layer.layer_name);
                
                            if *req_layer == layer_name {
                
                                found = true;
                                break;
                
                            }
                            
                    
                        }
                
                
                        if !found {
                
                            CWARNS("Didn't find layer {}",&[req_layer.to_owned()]);
                
                
                        } else {

                            self.is_enabled = true;
                            
                        
                        }
                
                
                    }

                }

            },
            Err(e) => {
    
                CFATALS("Unable to enumerate layer properties because: {}",&[&e.to_string()]);
    
                self.is_enabled = false;
    
            }
    
        };

    
     
    }

    pub(crate) fn is_enabled(&self) -> bool { self.is_enabled }

    pub(crate) fn get_enable_layer(&self) -> Vec<*const c_char> {

        let raw_name:Vec<CString> = self.required
            .iter()
            .map(|layer_name| CString::new(*layer_name).unwrap())
            .collect();

        let enable_layer_name: Vec<*const c_char> = raw_name
            .iter()
            .map(|name| name.as_ptr() )
            .collect();


        enable_layer_name

    }

    
}