#[allow(dead_code)]


pub mod types;

pub mod loader;

use types::{Component,CType};

use thiserror::Error;
use std::sync::{atomic::{AtomicU32,Ordering}, Mutex};



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
    BAD_ID(u32)

}
//
//
// ------------------------------------------------------------------------------------------------
// Define
//
//
static COMPONENT_ID_COUNTER: AtomicU32 = AtomicU32::new(0);
//
//
lazy_static::lazy_static! {

    static ref COMPONENT_SYS:Mutex<ComponentSystem> = Mutex::new(ComponentSystem::init());

}
//
//
// ------------------------------------------------------------------------------------------------
// Subsystem
//
// 
struct ComponentSystem{

    components: Vec<CType> 

}
//
impl ComponentSystem {

    fn init() -> Self { ComponentSystem { components: Vec::new() } }

    fn push_component(&mut self,component:CType) {self.components.push(component); } 

}
//
//
// ------------------------------------------------------------------------------------------------
// Function Subsystem Access
//
//
pub fn get_next_id() -> u32 {

    let id = COMPONENT_ID_COUNTER.load(Ordering::Relaxed);

    COMPONENT_ID_COUNTER.store(id + 1, Ordering::Release);

    id

}
//
//
pub fn add_component(comp:CType) -> Result<(),EComponent> {

    COMPONENT_SYS.lock().map_err(|e|
        EComponent::SYS_ACCESS(e.to_string())
    )?.push_component(comp);

    Ok(())

}
//
//
pub fn get_component(cid:u32) -> Result<CType,EComponent> {

    if COMPONENT_SYS.lock().map_err(|e|
        EComponent::SYS_ACCESS(e.to_string())
    )?.components.len() <= cid as usize {


        return Err(EComponent::BAD_ID(cid));

    }   

    Ok(&COMPONENT_SYS.lock().map_err(|e|
        EComponent::SYS_ACCESS(e.to_string())
        )?.components[cid as usize]
    )



}

