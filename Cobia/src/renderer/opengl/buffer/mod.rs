
pub(crate) mod ebo;
pub(crate) mod vao;
pub(crate) mod vbo;

use super::EOpenGL;


pub(crate) trait Binding {

    /// Make the buffer current 
    fn bind(&self) -> Result<(),EOpenGL>;
    /// Unset the buffer in current use 
    fn unbind(&self) -> Result<(),EOpenGL>;
    
}