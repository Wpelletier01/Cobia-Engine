

pub(crate) mod api;





#[allow(non_camel_case_types)]
#[derive(Debug,thiserror::Error)]
pub(crate) enum EOpenGL {
    //
    #[error("OpenGl has raised an error when calling {0}\n{1}")]
    API_CALL(String,String)
 
}