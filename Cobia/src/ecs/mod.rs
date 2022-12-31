#[allow(dead_code)]

pub mod image;


use thiserror::Error;
use std::sync::atomic::{AtomicU32,Ordering};


static COMPONENT_COUNTER: AtomicU32 = AtomicU32::new(0);


pub fn get_next_id() -> u32 {

    let id = COMPONENT_COUNTER.load(Ordering::Relaxed);

    COMPONENT_COUNTER.store(id + 1,Ordering::Release);

    id 

}

enum CType {

    IMAGE,
    TEXTURE

}
//
trait Component {
    //
    /// get th id of the component 
    /// TODO: maybe it represents the index of the component in an array of components?
    fn get_id(&self) -> u32;
    //
    /// return the type of the component
    fn get_type(&self) -> CType;
    //
}
//
//

#[derive(Debug, Error)]
pub enum EComponent {

    #[error("Unable to load image file {0} because {1}")]
    LOAD_IMAGE(String, String)


}

