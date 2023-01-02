

pub mod file;
pub mod image;

use thiserror::Error;

use super::{types::CFile, EComponent};


//
//
// ------------------------------------------------------------------------------------------------
// Error 
//
//
#[allow(non_camel_case_types)]
#[derive(Debug, Error)]
pub enum ELoader {

    #[error("File error caused by: {source}")]
    FILE_ERROR{ 

        #[from]
        source: file::EFile

    },

    #[error("Unable to load image file {0} because {1}")]
    LOAD_IMAGE(String, String),

    #[error("Error caused by another component: {source}")]
    COMPONENT_REF {

        #[from]
        source: EComponent

    }


}
//
//
