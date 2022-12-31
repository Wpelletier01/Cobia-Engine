#[allow(dead_code)]

pub mod image;
pub mod types;
pub mod file;


use thiserror::Error;
use std::sync::atomic::{AtomicU32,Ordering};

//
//
// ------------------------------------------------------------------------------------------------
// Error 
//
//
#[allow(non_camel_case_types)]
#[derive(Debug, Error)]
pub enum EComponent {

    #[error("File error caused by: {source}")]
    FILE_ERROR{ 

        #[from]
        source: file::EFile

    },

    #[error("Unable to load image file {0} because {1}")]
    LOAD_IMAGE(String, String)

}
//
//
// ------------------------------------------------------------------------------------------------
// Define
//
//
static COMPONENT_COUNTER: AtomicU32 = AtomicU32::new(0);


pub fn get_next_id() -> u32 {

    let id = COMPONENT_COUNTER.load(Ordering::Relaxed);

    COMPONENT_COUNTER.store(id + 1,Ordering::Release);

    id 

}
//
//
