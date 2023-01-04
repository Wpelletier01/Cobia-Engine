#![allow(dead_code)]


pub mod types;

use types::Component;
use thiserror::Error;
use ShaderParser::EParser;



//
//
// ------------------------------------------------------------------------------------------------
// Error 
//
//
#[allow(non_camel_case_types)]
#[derive(Debug, Error)]
pub enum EComponent {

    #[error("Opengl Api cause an error {source}")]
    GL{ 
        #[from]
        source: crate::renderer::opengl::EOpenGL
    },

    #[error("General Error caused by: {source}")]
    GENERAL {
        #[from]
        source: crate::ECobia,
    },

    #[error("No component have the id {0}")]
    BAD_ID(u32),

    #[error("The component got a reference to the component with id {0} but got {1}")]
    BAD_REF(u32, u32),

    //
    // --------------------------------------------------------------------------------------------
    // CFILE Error
    //
    //
    #[error("Cant get file access contents because: {0}")]
    FILE_ACCESS(String,String),

    #[error("Cant open/load the file {file} because: {cause}")]
    LOADING_FILE { file: String, cause: String },

    #[error(
        "Cant access the file {0} extension. Possible cause: 
        - there is no file name
        - there is no embedded '.'
        - other 
    "
    )]
    FILE_EXT(String),

    #[error("Cant get file contents because: {0}")]
    FILE_CONTENT(String),
    //
    // --------------------------------------------------------------------------------------------
    // CImage 
    //
    //
    #[error("Unable to load image file {0} because {1}")]
    LOAD_IMAGE(String, String),
    //
    // --------------------------------------------------------------------------------------------
    // Shader 
    //
    //
    #[error("unsupported shader file with extension: {0}")]
    SHADER_TYPE(String),
    
    #[error("Error when parsing shader: {source} ")]
    PARSER {
        #[from]
        source: EParser

    },

    //
    // --------------------------------------------------------------------------------------------
    //
    //
    //
}
//
//
// ------------------------------------------------------------------------------------------------
// Subsystem
//
//
//
struct ComponentSystem {
    
    id_counter: u32,
    components: Vec<Box<dyn Component>>,
    current_shader: u32 

}
//
impl ComponentSystem {

    fn init() -> Self { ComponentSystem { id_counter:1, components: Vec::new(), current_shader: 0 } }

    fn push(&mut self,component:Box<dyn Component>)  { self.components.push(component); }

    fn get(&self,id:u32) -> &Box<dyn Component> { &self.components[id as usize] } 

    pub fn set_curent_shader(&mut self, id:u32) { self.current_shader = id }

    
}
//
//
// ------------------------------------------------------------------------------------------------
// Function Subsystem Access
//
//


