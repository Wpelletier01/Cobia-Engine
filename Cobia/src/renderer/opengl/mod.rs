

pub(crate) mod api;
pub(crate) mod buffer;

use thiserror::Error;


#[allow(non_camel_case_types)]
#[derive(Debug,Error)]
pub(crate) enum EOpenGL {
    //
    #[error("OpenGl has raised an error when calling {0}\n{1}")]
    API_CALL(String,String),

    #[error("An OpenGl {0} buffer error ocurred because: {1}")]
    BUFFER(String,String)
 
}