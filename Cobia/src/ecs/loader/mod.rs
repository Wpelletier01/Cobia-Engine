

pub mod file;
pub mod image;

use thiserror::Error;

use super::{get_next_id,types::CFile};


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
    LOAD_IMAGE(String, String)


}
//
//
