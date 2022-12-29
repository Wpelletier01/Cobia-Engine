

use std::ffi::c_void;
use std::mem;

use crate::renderer::opengl::{EOpenGL,api};


pub(crate) struct VBO { id: u32 }
//
impl VBO {
    //
    /// Generates a new array buffer 
    pub fn new() -> Result<Self,EOpenGL> {

        let mut id = 0;

        api::gen_buffer(1, &mut id)?;

        Ok(VBO { id: id })

    }
    //
    /// Pass vertex data to this vbo buffer 
    pub fn set_data(&self, data: &Vec<f32>,usage:gl::types::GLenum) -> Result<(),EOpenGL> {
        
        api::buffer_data(
            gl::ARRAY_BUFFER,
            (data.len()*mem::size_of::<f32>()) as gl::types::GLsizeiptr ,
            data.as_ptr() as *const c_void,
            usage
        )?;


        Ok(())


    }
    //
    //
}
//
impl super::Binding for VBO {
    
    fn bind(&self) -> Result<(),EOpenGL> {
        
        api::bind_buffer(gl::ARRAY_BUFFER, self.id)?;

        Ok(())
        
    }


    fn unbind(&self) -> Result<(),EOpenGL> {
        
        api::bind_buffer(gl::ARRAY_BUFFER, self.id)?;

        Ok(())

    }

}