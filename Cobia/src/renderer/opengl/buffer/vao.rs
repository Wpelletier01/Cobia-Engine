

use crate::renderer::opengl::{EOpenGL,api};

use std::ffi::c_void;
use std::mem;


#[derive(Debug)]
pub(crate) struct VAO { id: u32 }
//
impl VAO {
    //
    //
    /// Generate an Vertex Array buffer
    pub(crate) fn new() -> Result<Self,EOpenGL> {

        let mut id:u32 = 0;

        api::gen_vertex_array(1, &mut id)?;


        Ok( VAO { id: id })


    }
    //
    //
    /// Set the vertex attribute pointer that will detail to OpenGL how to interpret each
    /// stripe of the array buffer
    /// 
    /// * 'index'           - number of vertex attributes that have been set
    /// * 'offset'          - where the attribute starts on each stripe
    /// * 'length'          - number of entries in the array that the attribute have 
    /// * 'stride_length'   - the number of entries in the array that represent one vertex
    /// 
    pub fn set_vertex_attribute_pointer(
        &mut self,
        index:          u32,
        offset:         usize,
        lenght:         i32,
        stride_length:  i32, ) -> Result<(),EOpenGL> {

        
        
        api::vertex_attrib_pointer(
            index,
            lenght,
            gl::FLOAT, 
            false, 
            stride_length*mem::size_of::<f32>() as i32, 
            (offset*mem::size_of::<f32>()) as *const c_void
        )?;

        api::enable_vertex_attrib_array(index)?;

        Ok(())
    }
    //
    //
}
//
impl super::Binding for VAO {

    fn bind(&self) -> Result<(),EOpenGL> {
        
        api::bind_vertex_array(self.id)?;

        Ok(())

    }

    fn unbind(&self) -> Result<(),EOpenGL> {
        
        api::bind_vertex_array(0)?;

        Ok(())

    }

}