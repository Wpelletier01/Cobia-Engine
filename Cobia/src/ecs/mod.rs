#[allow(dead_code)]


pub mod types;

pub mod loader;


use types::Component;
use thiserror::Error;




//
//
// ------------------------------------------------------------------------------------------------
// Error 
//
//
#[allow(non_camel_case_types)]
#[derive(Debug, Error)]
pub enum EComponent {

    #[error("unable to access the component subsystem because: {0}")]
    SYS_ACCESS(String),

    #[error("No component have the id {0}")]
    BAD_ID(u32),

    #[error("Cant get file access contents because: {0}")]
    FILE_ACCESS(String),

}
//
//
// ------------------------------------------------------------------------------------------------
// Subsystem
//
// 
struct ComponentSystem {
    
    id_counter: u32,
    components: Vec<Box<dyn Component>> 

}
//
impl ComponentSystem {

    fn init() -> Self { ComponentSystem { id_counter:0, components: Vec::new() } }

    fn push(&mut self,component:Box<dyn Component>)  { self.components.push(component); }

    fn get(&self,id:u32) -> &Box<dyn Component> { &self.components[id as usize] } 

}
//
//
// ------------------------------------------------------------------------------------------------
// Function Subsystem Access
//
//


